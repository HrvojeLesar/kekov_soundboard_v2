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

use crate::{utils::{
    auth::{get_access_token, get_discord_user_from_token, validate_request, AuthorizedUser},
    cache::AuthorizedUsersCache,
}, error::errors::KekServerError};

pub struct AuthService;

impl<S> Transform<S, ServiceRequest> for AuthService
where
    S: Service<ServiceRequest, Response = ServiceResponse, Error = Error> + 'static,
{
    type Response = ServiceResponse;
    type Error = Error;
    type Transform = AuthServiceMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        return ready(Ok(AuthServiceMiddleware {
            service: Rc::new(service),
        }));
    }
}

pub struct AuthServiceMiddleware<S> {
    service: Rc<S>,
}

impl<S> Service<ServiceRequest> for AuthServiceMiddleware<S>
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
            let cache = match req.app_data::<Data<AuthorizedUsersCache>>() {
                Some(c) => c.clone(),
                None => panic!("Authorized user cache should always be present"),
            };

            let access_token = Arc::new(get_access_token(&req).await?);
            let authorized_user;
            if !cache.contains_key(&access_token) {
                let user = get_discord_user_from_token(&access_token).await?;
                authorized_user = Arc::new(AuthorizedUser {
                    access_token: Arc::clone(&access_token),
                    discord_user: user,
                });

                cache.insert(access_token, Arc::clone(&authorized_user)).await;
            } else {
                authorized_user = match cache.get(&access_token) {
                    Some(au) => au,
                    None => return Err(KekServerError::UserNotInCacheError.into()),
                }
            }

            req.extensions_mut().insert(Arc::clone(&authorized_user));

            let resp = service.call(req).await?;
            return Ok(resp);
        });
    }
}
