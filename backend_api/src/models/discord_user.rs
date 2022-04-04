use std::{fmt::Display, str::FromStr};

use serde::{Deserialize, Deserializer, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(deserialize_with = "deserialize_string_to_number")]
    id: i64,
    #[serde(rename = "username")]
    discord_username: String,
    #[serde(rename = "avatar")]
    discord_avatar: Option<String>,
}

impl User {
    pub fn get_id(&self) -> &i64 {
        return &self.id;
    }

    pub fn get_username(&self) -> &String {
        return &self.discord_username;
    }

    pub fn get_avatar(&self) -> Option<&String> {
        return self.discord_avatar.as_ref();
    }
}

fn deserialize_string_to_number<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr + Deserialize<'de>,
    <T as FromStr>::Err: Display,
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
