use std::str::FromStr;

use actix_http::{encoding::Decoder, Payload};
use actix_web::http::header::AUTHORIZATION;
use awc::ClientResponse;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::{error::errors::KekServerError, models::ids::Id};

use self::auth::AuthorizedUser;

pub mod auth;
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

pub async fn make_discord_get_request(
    autorized_user: &AuthorizedUser,
    url: &str,
) -> Result<ClientResponse<Decoder<Payload>>, KekServerError> {
    let resp = awc::Client::new()
        .get(format!("https://discord.com/api/v9{}", url))
        .append_header((
            AUTHORIZATION,
            format!("Bearer {}", autorized_user.get_access_token()),
        ))
        .send()
        .await?;
    if resp.status().is_client_error() {
        return Err(KekServerError::DiscordRequestError);
    }
    return Ok(resp);
}
