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
use log::warn;
use tokio::sync::Notify;

use crate::{
    error::errors::KekServerError,
    utils::{
        auth::{
            get_access_token, get_discord_user_from_token, AuthorizedUser,
            AuthorizedUserServiceType,
        },
        cache::{AuthMiddlewareQueueCache, AuthorizedUsersCache},
    },
};

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
            let authorized_user;
            if !cache.contains_key(&access_token) {
                let lock = queue_cache.lock().unwrap();
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
                            return Err(e.into());
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
                    let lock = queue_cache.lock().unwrap();
                    lock.0.invalidate(&access_token).await;
                } else {
                    authorized_user = match cache.get(&access_token) {
                        Some(au) => au,
                        None => return Err(KekServerError::UserNotInCacheError.into()),
                    };
                    debug!("Skip auth");
                }
            } else {
                authorized_user = match cache.get(&access_token) {
                    Some(au) => au,
                    None => return Err(KekServerError::UserNotInCacheError.into()),
                }
            }

            let authorized_user: AuthorizedUserServiceType = Arc::clone(&authorized_user);
            req.extensions_mut().insert(Arc::clone(&authorized_user));

            let resp = service.call(req).await?;
            return Ok(resp);
        });
    }
}
