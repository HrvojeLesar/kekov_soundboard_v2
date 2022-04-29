use std::{time::Duration, sync::Arc};

use moka::future::Cache;

use crate::models::ids::{GuildId, UserId};

use super::auth::{AuthorizedUser, AccessToken};

pub type UserGuildsCache = Cache<UserId, Arc<Vec<GuildId>>>;
pub type AuthorizedUsersCache = Cache<Arc<AccessToken>, Arc<AuthorizedUser>>;

pub fn create_user_guilds_cache() -> UserGuildsCache {
    return Cache::builder()
        .max_capacity(1000)
        .initial_capacity(100)
        .time_to_live(Duration::from_secs(60 * 5))
        .build();
}

pub fn create_authorized_user_cache() -> AuthorizedUsersCache {
    return Cache::builder()
        .max_capacity(1000)
        .initial_capacity(200)
        .time_to_live(Duration::from_secs(60))
        .build()
}
