use actix_web::web::{scope, ServiceConfig};

use crate::middleware::auth_middleware::AuthService;

mod auth;
mod file;

pub fn routes_config(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("v1")
            .configure(auth::config)
            .configure(file::config)
    );
}
