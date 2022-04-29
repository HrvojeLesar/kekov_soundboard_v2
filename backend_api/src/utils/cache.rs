use std::{time::Duration, sync::Arc};

use moka::future::Cache;

use crate::models::ids::{GuildId, UserId};

pub type UserGuildsCache = Cache<UserId, Arc<Vec<GuildId>>>;

pub fn create_cache() -> UserGuildsCache {
    return Cache::builder()
        .max_capacity(1000)
        .initial_capacity(100)
        .time_to_live(Duration::from_secs(60 * 5))
        .build();
}
