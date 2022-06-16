use std::{
    future::{ready, Ready},
    rc::Rc,
    sync::{Arc, Mutex},
};

use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    web::Data,
    Error, HttpMessage,
};
use log::{debug, warn};
use tokio::sync::Notify;

use crate::{
    error::errors::KekServerError,
    utils::{
        auth::AuthorizedUserServiceType,
        cache::{UserGuildsCache, UserGuildsMiddlwareQueueCache},
    },
};

pub struct UserGuildsService;

impl<S> Transform<S, ServiceRequest> for UserGuildsService
where
    S: Service<ServiceRequest, Response = ServiceResponse, Error = Error> + 'static,
{
    type Response = ServiceResponse;
    type Error = Error;
    type Transform = UserGuildsServiceMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        return ready(Ok(UserGuildsServiceMiddleware {
            service: Rc::new(service),
        }));
    }
}

pub struct UserGuildsServiceMiddleware<S> {
    service: Rc<S>,
}

impl<S> Service<ServiceRequest> for UserGuildsServiceMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse, Error = Error> + 'static,
{
    type Response = ServiceResponse;
    type Error = Error;
    type Future =
        std::pin::Pin<Box<dyn std::future::Future<Output = Result<Self::Response, Self::Error>>>>;

    actix_web::dev::forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let service = Rc::clone(&self.service);
        return Box::pin(async move {
            {
                let extensions = req.extensions();
                let authorized_user = match extensions.get::<AuthorizedUserServiceType>() {
                    Some(a) => a,
                    None => {
                        warn!("AuthorizedUser is added to extensions with middlware. Possible reason for missing user.");
                        return Err(KekServerError::AuthorizedUserNotFoundError.into());
                    }
                };
                let cache = match req.app_data::<Data<UserGuildsCache>>() {
                    Some(c) => c.clone(),
                    None => panic!("Guild cache should always be present!"),
                };

                let queue_cache = match req.app_data::<Data<Mutex<UserGuildsMiddlwareQueueCache>>>()
                {
                    Some(c) => c.clone(),
                    None => panic!("Queue cache should always be present!"),
                };

                let user_id = &authorized_user.discord_user.id;
                if !cache.contains_key(user_id) {
                    let lock = queue_cache.lock().unwrap();
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
                                return Err(e.into());
                            }
                        };
                        // let user_guilds = Arc::new(authorized_user.get_guilds().await?);
                        cache.insert(user_id.clone(), user_guilds).await;
                        notify.notify_waiters();
                        let lock = queue_cache.lock().unwrap();
                        lock.0.invalidate(&authorized_user.access_token).await;
                    } else {
                        debug!("Skip guilds");
                    }
                }
            }

            let resp = service.call(req).await?;
            return Ok(resp);
        });
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use actix_http::HttpMessage;
    use actix_web::{
        test::{call_service, init_service, TestRequest},
        web::{get, Data},
        App, HttpResponse, Responder,
    };

    use crate::{
        models::{ids::GuildId, user::User},
        utils::{
            auth::{AccessToken, AuthorizedUser},
            cache::{AuthorizedUsersCache, DiscordGuild, UserGuildsCache},
        },
    };

    use super::UserGuildsService;

    async fn dummy_route() -> impl Responder {
        return HttpResponse::Ok().finish();
    }

    #[actix_web::test]
    async fn test_user_guilds_middleware() {
        let authorized_user = Arc::new(AuthorizedUser {
            access_token: Arc::new(AccessToken("test_token".to_owned())),
            discord_user: User::get_test_user(),
        });
        let user_cache = AuthorizedUsersCache::new(1);
        let user_guilds_cache = UserGuildsCache::new(1);
        user_cache
            .insert(
                authorized_user.access_token.clone(),
                authorized_user.clone(),
            )
            .await;
        user_guilds_cache
            .insert(
                authorized_user.discord_user.id.clone(),
                Arc::new(vec![DiscordGuild {
                    id: GuildId(1),
                    name: "test_guild".to_owned(),
                    icon: None,
                    icon_hash: None,
                }]),
            )
            .await;
        let app = init_service(
            App::new()
                .wrap(UserGuildsService)
                .app_data(Data::new(user_cache))
                .app_data(Data::new(user_guilds_cache))
                .route("/", get().to(dummy_route)),
        )
        .await;
        let req = TestRequest::default().to_request();
        req.extensions_mut().insert(authorized_user);
        let resp = call_service(&app, req).await;
        assert!(resp.status().is_success());
    }
}
