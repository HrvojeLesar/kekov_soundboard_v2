use actix_web::web::{scope, ServiceConfig};

mod auth;
mod controls;
mod file;
mod guild;
mod user;
mod ws;

pub mod not_found;
pub mod status;

pub fn routes_config(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/v1")
            .configure(auth::config)
            .configure(file::config)
            .configure(user::config)
            .configure(guild::config)
            .configure(ws::config)
            .configure(controls::config)
            .configure(status::config),
    );
}
