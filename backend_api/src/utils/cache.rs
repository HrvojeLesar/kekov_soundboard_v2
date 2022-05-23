use std::{sync::Arc, time::Duration};

use actix_web::web::Data;
use moka::future::Cache;

use crate::{
    error::errors::KekServerError,
    models::ids::{GuildId, UserId},
};

use super::auth::{AccessToken, AuthorizedUser};

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
        .time_to_live(Duration::from_secs(60 * 5))
        .build();
}

pub struct UserGuildsCacheUtil;

impl UserGuildsCacheUtil {
    pub fn get_user_guilds(
        authorized_user: &AuthorizedUser,
        user_guilds_cache: &Data<UserGuildsCache>,
    ) -> Result<Arc<Vec<GuildId>>, KekServerError> {
        match user_guilds_cache.get(authorized_user.get_discord_user().get_id()) {
            Some(ug) => return Ok(ug),
            None => return Err(KekServerError::UserNotInCacheError),
        };
    }
}
