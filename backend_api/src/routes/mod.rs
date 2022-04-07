use actix_web::web::{scope, ServiceConfig};

mod auth;
mod file;
mod user;
mod guild;
pub mod not_found;

pub fn routes_config(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/v1")
            .configure(auth::config)
            .configure(file::config)
            .configure(user::config)
            .configure(guild::config)
    );
}
