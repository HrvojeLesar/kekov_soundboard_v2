use std::str::FromStr;

use serde::{Deserialize, Serialize, Deserializer};

pub mod auth;

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
