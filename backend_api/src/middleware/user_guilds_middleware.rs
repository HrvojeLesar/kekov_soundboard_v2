use std::{
    future::{ready, Ready},
    rc::Rc,
    sync::Arc,
};

use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    web::Data,
    Error, HttpMessage,
};
use log::warn;

use crate::{
    error::errors::KekServerError,
    utils::{auth::AuthorizedUserServiceType, cache::UserGuildsCache},
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

                let user_id = authorized_user.get_discord_user().get_id();
                if !cache.contains_key(user_id) {
                    let user_guilds = Arc::new(
                        authorized_user
                            .get_guilds()
                            .await?
                    );
                    cache.insert(user_id.clone(), user_guilds).await;
                }
            }

            let resp = service.call(req).await?;
            return Ok(resp);
        });
    }
}
