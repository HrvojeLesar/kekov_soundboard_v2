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
use tokio::sync::Mutex;

use crate::utils::{
    auth::{get_access_token, AuthorizedUserServiceType},
    cache::{AuthMiddlewareQueueCache, AuthorizedUsersCache},
};

use super::authorize_user;

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

            let queue_cache = match req.app_data::<Data<Mutex<AuthMiddlewareQueueCache>>>() {
                Some(c) => c.clone(),
                None => panic!("Queue cache should always be present!"),
            };

            let access_token = Arc::new(get_access_token(&req).await?);
            let authorized_user = authorize_user(access_token, cache, queue_cache).await?;

            let authorized_user: AuthorizedUserServiceType = Arc::clone(&authorized_user);
            req.extensions_mut().insert(Arc::clone(&authorized_user));

            let resp = service.call(req).await?;
            return Ok(resp);
        });
    }
}
