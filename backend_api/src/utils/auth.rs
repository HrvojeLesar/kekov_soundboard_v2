use actix_http::StatusCode;
use actix_web::{dev::ServiceRequest, http::header::AUTHORIZATION, FromRequest, HttpMessage};
use log::debug;
use std::{future::Future, pin::Pin, sync::Arc};

use crate::{error::errors::KekServerError, models::user::User};

use super::{cache::DiscordGuild, make_discord_get_request, USERGUILDS};

pub type AuthorizedUserServiceType = Arc<AuthorizedUser>;

#[derive(Debug)]
pub struct AuthorizedUser {
    pub access_token: Arc<AccessToken>,
    pub discord_user: User,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct AccessToken(pub String);

impl From<String> for AccessToken {
    fn from(string: String) -> Self {
        return Self(string);
    }
}

impl AuthorizedUser {
    pub async fn get_guilds(&self) -> Result<Vec<DiscordGuild>, KekServerError> {
        return Ok(make_discord_get_request(self, USERGUILDS)
            .await?
            .json()
            .await?);
    }
}

pub struct AuthorizedUserExt(pub Arc<AuthorizedUser>);

impl FromRequest for AuthorizedUserExt {
    type Error = KekServerError;

    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(
        req: &actix_web::HttpRequest,
        _payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        let req = req.clone();

        return Box::pin(async move {
            if let Some(user) = req.extensions_mut().remove::<Arc<AuthorizedUser>>() {
                return Ok(AuthorizedUserExt(user));
            }
            debug!("AuthorizedUser is added to extensions with middlware. Possible reason for missing user.");
            return Err(KekServerError::RequestExtensionsError);
        });
    }
}

pub async fn get_access_token(req: &ServiceRequest) -> Result<AccessToken, KekServerError> {
    let token = req
        .headers()
        .get("Authorization")
        .ok_or(KekServerError::InvalidCredentialsError)?
        .to_str()?;

    return Ok(AccessToken(token.to_owned()));
}

// TODO: Handle rate limiting
pub async fn get_discord_user_from_token(
    access_token: &AccessToken,
) -> Result<User, KekServerError> {
    let mut resp = awc::Client::new()
        .get("https://discord.com/api/v9/users/@me")
        .append_header((AUTHORIZATION, format!("Bearer {}", access_token.0)))
        .send()
        .await?;

    if resp.status() == StatusCode::TOO_MANY_REQUESTS {
        log::error!("EXCEEDING RATE LIMIT!!!");
        // TODO: Rate limit user for the duration and notify them
    }

    if resp.status().is_client_error() {
        return Err(KekServerError::InvalidCredentialsError);
    }

    return Ok(resp.json().await?);
}
