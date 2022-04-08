use actix_web::{dev::ServiceRequest, http::header::AUTHORIZATION, FromRequest, HttpMessage};
use std::{future::Future, pin::Pin};

use crate::{error::errors::KekServerError, models::user::User};

pub struct AuthorizedUser {
    access_token: String,
    discord_user: User,
}

impl AuthorizedUser {
    pub fn get_access_token(&self) -> &String {
        return &self.access_token;
    }

    pub fn get_discord_user(&self) -> &User {
        return &self.discord_user;
    }
}

impl FromRequest for AuthorizedUser {
    type Error = KekServerError;

    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(
        req: &actix_web::HttpRequest,
        _payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        let req = req.clone();

        return Box::pin(async move {
            if let Some(user) = req.extensions_mut().remove::<AuthorizedUser>() {
                return Ok(user);
            }
            return Err(KekServerError::RequestExtensionsError);
        });
    }
}

async fn get_access_token(req: &ServiceRequest) -> Result<String, KekServerError> {
    let token = req
        .headers()
        .get("Authorization")
        .ok_or(KekServerError::InvalidCredentialsError)?
        .to_str()
        .map_err(|_| KekServerError::InvalidCredentialsError)?;

    return Ok(token.to_owned());
}

pub async fn get_discord_user_from_token(access_token: &str) -> Result<User, KekServerError> {
    let mut resp = awc::Client::new()
        .get("https://discord.com/api/v9/users/@me")
        .append_header((AUTHORIZATION, format!("Bearer {}", access_token)))
        .send()
        .await?;

    if resp.status().is_client_error() {
        return Err(KekServerError::InvalidCredentialsError);
    }

    return Ok(resp.json().await?);
}

pub async fn validate_request(req: &ServiceRequest) -> Result<AuthorizedUser, KekServerError> {
    let token = get_access_token(req).await?;
    let discord_user = get_discord_user_from_token(&token).await?;

    return Ok(AuthorizedUser {
        access_token: token,
        discord_user,
    });
}

#[cfg(test)]
mod tests {
    use actix_web::{http::header::AUTHORIZATION, test::TestRequest};

    use super::{get_access_token, get_discord_user_from_token};

    #[actix_web::test]
    async fn test_validate_discord_token() {
        let res = match get_discord_user_from_token("invalid token").await {
            Ok(d) => Some(d),
            Err(_) => None,
        };

        assert!(res.is_none());
    }

    #[actix_web::test]
    async fn test_get_auth_token() {
        let req = TestRequest::default();
        let req = req
            .append_header((AUTHORIZATION, "auth_token"))
            .to_srv_request();
        let token = get_access_token(&req).await.unwrap();
        assert_eq!(token, "auth_token");
    }
}
