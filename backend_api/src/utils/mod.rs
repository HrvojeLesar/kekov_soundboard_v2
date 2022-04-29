use std::{
    str::FromStr,
    time::{Duration, Instant},
};

use actix::clock::{sleep, sleep_until};
use actix_http::{encoding::Decoder, Payload, StatusCode};
use actix_web::http::header::AUTHORIZATION;
use awc::{error::SendRequestError, ClientResponse};
use log::{debug, warn};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::{error::errors::KekServerError, models::ids::Id};

use self::auth::AuthorizedUser;

pub mod auth;
pub mod cache;
pub mod validation;

pub const USERGUILDS: &str = "/users/@me/guilds";

#[derive(Serialize, Deserialize)]
pub struct GenericSuccess {
    pub success: String,
}

impl GenericSuccess {
    pub fn new(message: &str) -> Self {
        return GenericSuccess {
            success: message.to_owned(),
        };
    }
}

impl Default for GenericSuccess {
    fn default() -> Self {
        return Self {
            success: "success".to_owned(),
        };
    }
}

#[derive(Clone, Debug, Deserialize)]
struct RateLimitResponse {
    message: String,
    retry_after: f64,
    global: bool,
}

pub fn deserialize_string_to_number<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr + Deserialize<'de>,
    <T as FromStr>::Err: std::fmt::Display,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StringOrNum<T> {
        String(String),
        Number(T),
    }

    return match StringOrNum::<T>::deserialize(deserializer)? {
        StringOrNum::String(s) => s.parse::<T>().map_err(serde::de::Error::custom),
        StringOrNum::Number(n) => Ok(n),
    };
}

pub fn serialize_i64_to_string<S>(num: &i64, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    return serializer.serialize_str(&num.to_string());
}

pub fn serialize_id_to_string<S>(num: &dyn Id, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    return serializer.serialize_str(&num.get_id().to_string());
}

async fn get_request(
    autorized_user: &AuthorizedUser,
    url: &str,
) -> Result<ClientResponse<Decoder<Payload>>, SendRequestError> {
    return awc::Client::new()
        .get(format!("https://discord.com/api/v9{}", url))
        .append_header((
            AUTHORIZATION,
            format!("Bearer {}", autorized_user.get_access_token()),
        ))
        .send()
        .await;
}

pub async fn make_discord_get_request(
    autorized_user: &AuthorizedUser,
    url: &str,
) -> Result<ClientResponse<Decoder<Payload>>, KekServerError> {
    let mut resp = get_request(autorized_user, url).await?;
    while resp.status() == StatusCode::TOO_MANY_REQUESTS {
        warn!("Rate limit exceeded");
        if let Some(after) = resp.headers().get("retry-after") {
            let after = after.to_str()?.parse()?;
            let sleep_dur = Duration::from_secs_f64(after);
            sleep(sleep_dur).await;
        } else {
            return Err(KekServerError::DiscordRequestError);
        }
        resp = get_request(autorized_user, url).await?;
    }

    if resp.status().is_client_error() {
        debug!("{:#?}", &resp.status());
        debug!("{:#?}", &resp.headers());
        return Err(KekServerError::DiscordRequestError);
    }

    return Ok(resp);
}
