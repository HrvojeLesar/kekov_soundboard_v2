use std::{fmt::Display, str::FromStr};

use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Serialize, Deserialize)]
pub struct DiscordUser {
    #[serde(deserialize_with = "deserialize_string_to_number")]
    id: u64,
    username: String,
    avatar: Option<String>,
}

impl DiscordUser {
    pub fn get_id(&self) -> &u64 {
        return &self.id;
    }

    pub fn get_username(&self) -> &String {
        return &self.username;
    }

    pub fn get_avatar(&self) -> Option<&String> {
        return self.avatar.as_ref();
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
