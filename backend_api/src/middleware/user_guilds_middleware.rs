use std::{
    future::{ready, Ready},
    rc::Rc,
};

use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    web::Data,
    Error, HttpMessage,
};
use log::warn;
use tokio::sync::Mutex;

use crate::{
    error::errors::KekServerError,
    utils::{
        auth::AuthorizedUserServiceType,
        cache::{UserGuildsCache, UserGuildsMiddlwareQueueCache},
    },
};

use super::cache_authorized_user_guilds;

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
                let authorized_user = {
                    let extensions = req.extensions();
                    match extensions.get::<AuthorizedUserServiceType>() {
                        Some(a) => a.clone(),
                        None => {
                            warn!("AuthorizedUser is added to extensions with middlware. Possible reason for missing user.");
                            return Err(KekServerError::AuthorizedUserNotFoundError.into());
                        }
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

                cache_authorized_user_guilds(&authorized_user, cache, queue_cache).await?;
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
                    permissions: None,
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
