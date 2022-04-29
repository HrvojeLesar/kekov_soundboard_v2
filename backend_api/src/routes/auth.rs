use std::sync::Arc;

use actix_web::{
    get,
    http::header::LOCATION,
    post,
    web::{scope, Data, Form, Query, ServiceConfig},
    HttpResponse,
};
use awc::Client;
use chrono::Utc;
use oauth2::{
    basic::BasicTokenType, url::Url, AccessToken, AuthorizationCode, CsrfToken, PkceCodeChallenge,
    PkceCodeVerifier, RefreshToken, StandardRevocableToken, StandardTokenResponse, TokenResponse,
};
use serde::{Deserialize, Serialize};
use sqlx::{PgPool, Postgres, Transaction};

use crate::{
    error::errors::KekServerError,
    models::{guild::Guild, state::State, user::User},
    oauth_client::{GuildTokenField, OAuthClient},
    utils::{
        auth::{self, get_discord_user_from_token},
        GenericSuccess, cache::AuthorizedUsersCache,
    },
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

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
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
    token_type: TokenType,
}

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/auth")
            .service(auth_init)
            .service(auth_callback)
            .service(auth_revoke)
            .service(bot_invite),
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
) -> Result<StandardTokenResponse<GuildTokenField, BasicTokenType>, KekServerError> {
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

async fn check_collision(
    transaction: &mut Transaction<'_, Postgres>,
    auth_url: &mut Url,
    csrf_token: &mut CsrfToken,
    pkce_challange: PkceCodeChallenge,
    oauth_client_url_fn: impl Fn(PkceCodeChallenge) -> (Url, CsrfToken),
) -> Result<(), KekServerError> {
    loop {
        if let Some(_) = sqlx::query!(
            // TODO: Expire or cleanup old states
            // Replace or update old state if key matches (csrf_token)
            "
            SELECT * FROM state
            WHERE csrf_token = $1
            ",
            csrf_token.secret()
        )
        .fetch_optional(&mut *transaction)
        .await?
        {
            (*auth_url, *csrf_token) = oauth_client_url_fn(pkce_challange.clone());
        } else {
            break;
        }
    }
    return Ok(());
}

async fn insert_state(
    transaction: &mut Transaction<'_, Postgres>,
    csrf_token: CsrfToken,
    pkce_verifier: PkceCodeVerifier,
) -> Result<(), KekServerError> {
    sqlx::query!(
        "
        INSERT INTO state (csrf_token, pkce_verifier)
        VALUES ($1, $2)
        ",
        csrf_token.secret(),
        pkce_verifier.secret(),
    )
    .execute(transaction)
    .await?;
    return Ok(());
}

async fn create_url(
    oauth_client_url_fn: impl Fn(PkceCodeChallenge) -> (Url, CsrfToken),
    db_pool: Data<PgPool>,
) -> Result<HttpResponse, KekServerError> {
    let (pkce_challange, pkce_verifier) = PkceCodeChallenge::new_random_sha256();
    let (mut auth_url, mut csrf_token) = oauth_client_url_fn(pkce_challange.clone());

    let mut transaction = db_pool.begin().await?;

    check_collision(
        &mut transaction,
        &mut auth_url,
        &mut csrf_token,
        pkce_challange,
        oauth_client_url_fn,
    )
    .await?;
    insert_state(&mut transaction, csrf_token, pkce_verifier).await?;

    transaction.commit().await?;

    return Ok(HttpResponse::TemporaryRedirect()
        .insert_header((LOCATION, auth_url.to_string()))
        .json(AuthInit {
            url: auth_url.to_string(),
        }));
}

#[get("/init")]
pub async fn auth_init(
    oauth_client: Data<OAuthClient>,
    db_pool: Data<PgPool>,
) -> Result<HttpResponse, KekServerError> {
    let url_getter_fn = |pkce: PkceCodeChallenge| oauth_client.get_url(pkce);
    return Ok(create_url(url_getter_fn, db_pool).await?);
}

#[get("/botinvite")]
pub async fn bot_invite(
    oauth_client: Data<OAuthClient>,
    db_pool: Data<PgPool>,
) -> Result<HttpResponse, KekServerError> {
    let url_getter_fn = |pkce: PkceCodeChallenge| oauth_client.get_bot_url(pkce);
    return Ok(create_url(url_getter_fn, db_pool).await?);
}

#[get("/callback")]
pub async fn auth_callback(
    oauth_client: Data<OAuthClient>,
    db_pool: Data<PgPool>,
    Query(auth_params): Query<AuthCallbackParams>,
) -> Result<HttpResponse, KekServerError> {
    if let (Some(params_code), Some(params_state)) = (auth_params.code, auth_params.state) {
        let mut transaction = db_pool.begin().await?;
        let auth_state = State::get_with_token(&params_state, &mut transaction).await?;

        if let Some(state) = auth_state {
            let duration = state.get_expires_date().signed_duration_since(Utc::now());
            if duration.num_seconds() < 0 {
                return Err(KekServerError::AuthorizationTimeExpiredError);
            }

            let access_token = fetch_access_token(
                oauth_client,
                params_code,
                state.get_pkce_verifier().to_string(),
            )
            .await?;

            State::delete_state(state.get_csrf_token(), &mut transaction).await?;

            let user = get_discord_user_from_token(&auth::AccessToken(
                access_token.access_token().secret().to_string(),
            ))
            .await?;
            if let None = User::get_with_id(user.get_id(), &mut transaction).await? {
                User::insert_user(
                    user.get_id(),
                    user.get_username(),
                    user.get_avatar(),
                    &mut transaction,
                )
                .await?;
            }

            // Adds guild to database
            // TODO: If guild exists mark as active (if bot was previously in guild)
            if let Some(guild) = &access_token.extra_fields().guild {
                if let None = Guild::get_guild_from_id(guild.get_id(), &mut transaction).await? {
                    Guild::insert_guild(
                        guild.get_id(),
                        guild.get_name(),
                        guild.get_icon(),
                        guild.get_icon_hash(),
                        &mut transaction,
                    )
                    .await?;
                }
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
#[post("/revoke")]
pub async fn auth_revoke(
    oauth_client: Data<OAuthClient>,
    Form(revoke_token): Form<RevokeToken>,
    authorized_users_cache: Data<AuthorizedUsersCache>,
) -> Result<HttpResponse, KekServerError> {
    let client = oauth_client.get_client();
    let request;

    match revoke_token.token_type {
        TokenType::AccessToken | TokenType::InvalidTokenType => {
            request = client.revoke_token(StandardRevocableToken::AccessToken(
                    AccessToken::new(revoke_token.token.clone()),
                    ))?;
        }
        TokenType::RefreshToken => {
            request = client.revoke_token(StandardRevocableToken::RefreshToken(
                    RefreshToken::new(revoke_token.token.clone()),
                    ))?;
        }
    }

    match request.request_async(send_oauth_request).await {
        Ok(_) => {
            if revoke_token.token_type == TokenType::AccessToken || revoke_token.token_type == TokenType::InvalidTokenType {
                authorized_users_cache.invalidate(&Arc::new(auth::AccessToken(revoke_token.token))).await;
            }
        },
        Err(err) => return Err(KekServerError::RevocationRequestTokenError(Box::new(err))),
    }

    return Ok(HttpResponse::Ok().finish());
}
