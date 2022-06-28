use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::models::ids::{ChannelId, UserId};

pub mod channels_client;
pub mod channels_server;
pub mod ws_server;
pub mod ws_session;
pub mod ws_sync;

#[derive(Clone, Debug, Serialize, Deserialize)]
struct User {
    id: UserId,
    discriminator: String,
    username: String,
    nickname: Option<String>,
    avatar_hash: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Channel {
    id: ChannelId,
    users: Vec<User>,
    channel_name: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GuildVoiceChannels {
    channels: HashMap<u64, Channel>,
}

impl GuildVoiceChannels {
    pub fn empty() -> Self {
        return Self {
            channels: HashMap::new(),
        };
    }
}
