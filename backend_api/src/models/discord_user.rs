use std::{fmt::Display, str::FromStr};

use serde::{Deserialize, Deserializer, Serialize};

use crate::utils::deserialize_string_to_number;

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
