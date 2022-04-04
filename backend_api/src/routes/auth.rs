use actix_web::{
    get,
    http::header::LOCATION,
    web::{scope, Data, Form, Query, ServiceConfig},
    HttpResponse,
};
use awc::Client;
use chrono::Utc;
use oauth2::{
    basic::BasicTokenType, AccessToken, AuthorizationCode, EmptyExtraTokenFields,
    PkceCodeChallenge, PkceCodeVerifier, RefreshToken, StandardRevocableToken,
    StandardTokenResponse, TokenResponse,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::{
    error::errors::KekServerError,
    oauth_client::OAuthClient,
    utils::{auth::get_discord_user_from_token, GenericSuccess},
};

#[derive(Serialize, Deserialize)]
struct AuthInit {
    pub url: String,
}

#[derive(Deserialize, Debug)]
pub struct AuthCallbackParams {
    code: Option<String>,
    state: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum TokenType {
    AccessToken,
    RefreshToken,
    #[serde(other)]
    InvalidTokenType,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RevokeToken {
    token: String,
    token_type: Option<TokenType>,
}

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/auth")
            .service(auth_init)
            .service(auth_callback)
            .service(auth_revoke),
    );
}

async fn send_oauth_request(
    request: oauth2::HttpRequest,
) -> Result<oauth2::HttpResponse, KekServerError> {
    let http_client = Client::new();
    let mut request_builder = http_client.request(request.method, request.url.to_string());

    for header in &request.headers {
        request_builder = request_builder.append_header(header);
    }

    let mut response = request_builder.send_body(request.body).await?;

    let mut headers = oauth2::http::HeaderMap::new();
    for (name, value) in response.headers().iter() {
        headers.insert(name, value.to_owned());
    }

    return Ok(oauth2::HttpResponse {
        status_code: response.status(),
        headers,
        body: response.body().await?.to_vec(),
    });
}

async fn fetch_access_token(
    oauth_client: Data<OAuthClient>,
    params_code: String,
    pkce_verifier: String,
) -> Result<StandardTokenResponse<EmptyExtraTokenFields, BasicTokenType>, KekServerError> {
    return match oauth_client
        .get_client()
        .exchange_code(AuthorizationCode::new(params_code))
        .set_pkce_verifier(PkceCodeVerifier::new(pkce_verifier))
        .request_async(send_oauth_request)
        .await
    {
        Ok(resp) => Ok(resp),
        Err(err) => return Err(KekServerError::RequestTokenError(Box::new(err))),
    };
}

#[get("/init")]
pub async fn auth_init(
    oauth_client: Data<OAuthClient>,
    db_pool: Data<PgPool>,
) -> Result<HttpResponse, KekServerError> {
    let (pkce_challange, pkce_verifier) = PkceCodeChallenge::new_random_sha256();
    let (mut auth_url, mut csrf_token) = oauth_client.get_url(pkce_challange.clone());

    let mut transaction = db_pool.begin().await?;
    loop {
        let res = sqlx::query!(
            "
            SELECT * FROM state
            WHERE csrf_token = $1
            ",
            csrf_token.secret()
        )
        .fetch_optional(&mut transaction)
        .await?;

        if let Some(_) = res {
            (auth_url, csrf_token) = oauth_client.get_url(pkce_challange.clone());
        } else {
            break;
        }
    }

    sqlx::query!(
        "
        INSERT INTO state (csrf_token, pkce_verifier)
        VALUES ($1, $2)
        ",
        csrf_token.secret(),
        pkce_verifier.secret(),
    )
    .execute(&mut transaction)
    .await?;

    transaction.commit().await?;

    return Ok(HttpResponse::TemporaryRedirect()
        .insert_header((LOCATION, auth_url.to_string()))
        .json(AuthInit {
            url: auth_url.to_string(),
        }));
}

#[get("/callback")]
pub async fn auth_callback(
    oauth_client: Data<OAuthClient>,
    db_pool: Data<PgPool>,
    Query(auth_params): Query<AuthCallbackParams>,
) -> Result<HttpResponse, KekServerError> {
    if let (Some(params_code), Some(params_state)) = (auth_params.code, auth_params.state) {
        let mut transaction = db_pool.begin().await?;
        let auth_state = sqlx::query!(
            "
            SELECT * FROM state
            WHERE csrf_token = $1
            ",
            params_state
        )
        .fetch_optional(&mut transaction)
        .await?;

        if let Some(state) = auth_state {
            let duration = state.expires.signed_duration_since(Utc::now());
            if duration.num_seconds() < 0 {
                return Err(KekServerError::AuthorizationTimeExpiredError);
            }

            let access_token =
                fetch_access_token(oauth_client, params_code, state.pkce_verifier).await?;

            sqlx::query!(
                "
                DELETE FROM state
                WHERE csrf_token = $1
                ",
                state.csrf_token
            )
            .execute(&mut transaction)
            .await?;

            let user = get_discord_user_from_token(access_token.access_token().secret()).await?;
            if let None = sqlx::query!(
                "
                SELECT * FROM users
                WHERE id = $1
                ",
                *user.get_id()
            )
            .fetch_optional(&mut transaction)
            .await?
            {
                sqlx::query!(
                    "
                    INSERT INTO users (id, username, avatar)
                    VALUES ($1, $2, $3)
                    ",
                    *user.get_id(),
                    *user.get_username(),
                    user.get_avatar()
                )
                .execute(&mut transaction)
                .await?;
            }

            transaction.commit().await?;

            return Ok(HttpResponse::Ok().json(access_token));
        } else {
            return Err(KekServerError::InvalidCredentialsError);
        }
    } else {
        return Err(KekServerError::InvalidCredentialsError);
    }
}

// TODO: maybe change revoke to usable only by logged in user i.e.
// only when Authorization header is used with token
// current implementation might be a bit illogical
// but works fine
#[get("/revoke")]
pub async fn auth_revoke(
    oauth_client: Data<OAuthClient>,
    Form(revoke_token): Form<RevokeToken>,
) -> Result<HttpResponse, KekServerError> {
    let client = oauth_client.get_client();
    let request;
    if let Some(token_type) = revoke_token.token_type {
        match token_type {
            TokenType::AccessToken | TokenType::InvalidTokenType => {
                request = client.revoke_token(StandardRevocableToken::AccessToken(
                    AccessToken::new(revoke_token.token),
                ))?;
            }
            TokenType::RefreshToken => {
                request = client.revoke_token(StandardRevocableToken::RefreshToken(
                    RefreshToken::new(revoke_token.token),
                ))?;
            }
        }
    } else {
        request = client.revoke_token(StandardRevocableToken::AccessToken(AccessToken::new(
            revoke_token.token,
        )))?;
    }

    match request.request_async(send_oauth_request).await {
        Ok(_) => (),
        Err(err) => return Err(KekServerError::RevocationRequestTokenError(Box::new(err))),
    }

    return Ok(HttpResponse::Ok().json(GenericSuccess::default()));
}

#[cfg(test)]
mod tests {
    use actix_web::{
        http::header::LOCATION,
        test,
        web::{Data, Query},
        App,
    };

    use crate::{
        database, oauth_client,
        routes::auth::{auth_init, AuthCallbackParams},
    };

    #[actix_web::test]
    async fn test_auth_init() {
        let pool = Data::new(
            database::create_pool()
                .await
                .expect("Failed to create database connection pool!"),
        );
        let oauth = Data::new(oauth_client::OAuthClient::new());

        let app = test::init_service(
            App::new()
                .app_data(pool.clone())
                .app_data(oauth.clone()) // oauth2::basic::BasicClient
                .service(auth_init),
        )
        .await;

        let req = test::TestRequest::get().uri("/auth").to_request();
        let resp = test::call_service(&app, req).await;
        let location = resp.headers().get(LOCATION).unwrap().to_str().unwrap();
        let Query(params) = Query::<AuthCallbackParams>::from_query(location).unwrap();

        let mut transaction = pool.begin().await.unwrap();
        let auth_state = sqlx::query!(
            "
            SELECT * FROM state
            WHERE csrf_token = $1
            ",
            params.state
        )
        .fetch_optional(&mut transaction)
        .await
        .unwrap();

        sqlx::query!(
            "
            DELETE FROM state
            WHERE csrf_token = $1
            ",
            params.state
        )
        .fetch_optional(&mut transaction)
        .await
        .unwrap();
        transaction.commit().await.unwrap();

        assert!(auth_state.is_some());
    }
}
