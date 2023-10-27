use std::{
    fs::{self},
    io,
};

use log::{error, info};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, PartialEq)]
pub struct DiscordUserId(u64);

impl From<u64> for DiscordUserId {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Config {
    allow_upload_from: Option<Vec<DiscordUserId>>,
}

impl Config {
    pub fn load_config() -> Option<Self> {
        let config_path = dotenv::var("CONFIG_PATH").unwrap_or("./config.json".to_string());
        let file_data = match fs::read(config_path) {
            Ok(data) => data,
            Err(e) => match e.kind() {
                io::ErrorKind::NotFound => {
                    info!("Config file not found! Skipping...");
                    return None;
                }
                _ => panic!("{}", e),
            },
        };
        match serde_json::from_slice(&file_data) {
            Ok(config) => Some(config),
            Err(e) => {
                error!("Failed to parse config: {}", e);
                None
            }
        }
    }

    pub fn get_allowed_uploaders(&self) -> &[DiscordUserId] {
        match &self.allow_upload_from {
            Some(uploaders) => uploaders,
            None => &[],
        }
    }
}
