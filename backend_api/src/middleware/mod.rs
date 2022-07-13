use std::sync::Arc;

use actix_web::web::Data;
use log::debug;
use tokio::sync::{Notify, Mutex};

use crate::{
    error::errors::KekServerError,
    utils::{
        auth::{get_discord_user_from_token, AccessToken, AuthorizedUser},
        cache::{
            AuthMiddlewareQueueCache, AuthorizedUsersCache, UserGuildsCache,
            UserGuildsMiddlwareQueueCache,
        },
    },
};

pub mod auth_middleware;
pub mod user_guilds_middleware;

pub async fn authorize_user(
    access_token: Arc<AccessToken>,
    cache: Data<AuthorizedUsersCache>,
    queue_cache: Data<Mutex<AuthMiddlewareQueueCache>>,
) -> Result<Arc<AuthorizedUser>, KekServerError> {
    let authorized_user;
    if !cache.contains_key(&access_token) {
        let lock = queue_cache.lock().await;
        let notify;
        if let Some(n) = lock.0.get(&access_token) {
            drop(lock);
            notify = n;
            // wait
            notify.notified().await;
        } else {
            notify = Arc::new(Notify::new());
            lock.0.insert(access_token.clone(), notify.clone()).await;
            drop(lock);
        }

        if !cache.contains_key(&access_token) {
            debug!("Auth");
            let user = match get_discord_user_from_token(&access_token).await {
                Ok(u) => u,
                Err(e) => {
                    notify.notify_one();
                    return Err(e);
                }
            };
            authorized_user = Arc::new(AuthorizedUser {
                access_token: Arc::clone(&access_token),
                discord_user: user,
            });

            cache
                .insert(access_token.clone(), Arc::clone(&authorized_user))
                .await;
            notify.notify_waiters();
            let lock = queue_cache.lock().await;
            lock.0.invalidate(&access_token).await;
        } else {
            authorized_user = match cache.get(&access_token) {
                Some(au) => au,
                None => return Err(KekServerError::UserNotInCacheError),
            };
            debug!("Skip auth");
        }
    } else {
        authorized_user = match cache.get(&access_token) {
            Some(au) => au,
            None => return Err(KekServerError::UserNotInCacheError),
        }
    }
    return Ok(authorized_user);
}

pub async fn cache_authorized_user_guilds(
    authorized_user: &AuthorizedUser,
    cache: Data<UserGuildsCache>,
    queue_cache: Data<Mutex<UserGuildsMiddlwareQueueCache>>,
) -> Result<(), KekServerError> {
    let user_id = &authorized_user.discord_user.id;
    if !cache.contains_key(user_id) {
        let lock = queue_cache.lock().await;
        let notify;
        if let Some(n) = lock.0.get(&authorized_user.access_token) {
            drop(lock);
            notify = n;
            // wait
            notify.notified().await;
        } else {
            let token = authorized_user.access_token.clone();
            notify = Arc::new(Notify::new());
            lock.0.insert(token, notify.clone()).await;
            drop(lock);
        }
        if !cache.contains_key(user_id) {
            debug!("Getting guilds");
            let user_guilds = match authorized_user.get_guilds().await {
                Ok(g) => Arc::new(g),
                Err(e) => {
                    notify.notify_one();
                    return Err(e);
                }
            };
            // let user_guilds = Arc::new(authorized_user.get_guilds().await?);
            cache.insert(user_id.clone(), user_guilds).await;
            notify.notify_waiters();
            let lock = queue_cache.lock().await;
            lock.0.invalidate(&authorized_user.access_token).await;
        } else {
            debug!("Skip guilds");
        }
    }
    return Ok(());
}
