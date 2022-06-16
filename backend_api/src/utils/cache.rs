use std::{sync::Arc, time::Duration};

use actix_web::web::Data;
use moka::future::Cache;
use serde::{Deserialize, Serialize};
use tokio::sync::Notify;

use crate::{
    error::errors::KekServerError,
    models::ids::{GuildId, UserId},
};

use super::auth::{AccessToken, AuthorizedUser};

pub type UserGuildsCache = Cache<UserId, Arc<Vec<DiscordGuild>>>;
pub type AuthorizedUsersCache = Cache<Arc<AccessToken>, Arc<AuthorizedUser>>;
pub struct UserGuildsMiddlwareQueueCache(pub Cache<Arc<AccessToken>, Arc<Notify>>);
pub struct AuthMiddlewareQueueCache(pub Cache<Arc<AccessToken>, Arc<Notify>>);

#[derive(Clone, Debug, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub struct DiscordGuild {
    pub id: GuildId,
    pub name: String,
    pub icon: Option<String>,
    pub icon_hash: Option<String>,
}

pub fn create_user_guilds_cache() -> UserGuildsCache {
    return Cache::builder()
        .max_capacity(1000)
        .initial_capacity(100)
        .time_to_live(Duration::from_secs(60 * 60))
        .build();
}

pub fn create_authorized_user_cache() -> AuthorizedUsersCache {
    return Cache::builder()
        .max_capacity(1000)
        .initial_capacity(200)
        .time_to_live(Duration::from_secs(60 * 5))
        .build();
}

pub fn create_user_guilds_middlware_queue_cache() -> UserGuildsMiddlwareQueueCache {
    return UserGuildsMiddlwareQueueCache(
        Cache::builder()
            .max_capacity(1000)
            .initial_capacity(200)
            .time_to_live(Duration::from_secs(60 * 5))
            .build(),
    );
}

pub fn create_auth_middlware_queue_cache() -> AuthMiddlewareQueueCache {
    return AuthMiddlewareQueueCache(
        Cache::builder()
            .max_capacity(1000)
            .initial_capacity(200)
            .time_to_live(Duration::from_secs(60 * 5))
            .build(),
    );
}

pub struct UserGuildsCacheUtil;

impl UserGuildsCacheUtil {
    pub fn get_user_guilds(
        authorized_user: &AuthorizedUser,
        user_guilds_cache: &Data<UserGuildsCache>,
    ) -> Result<Arc<Vec<DiscordGuild>>, KekServerError> {
        match user_guilds_cache.get(&authorized_user.discord_user.id) {
            Some(ug) => return Ok(ug),
            None => return Err(KekServerError::UserNotInCacheError),
        };
    }
}
