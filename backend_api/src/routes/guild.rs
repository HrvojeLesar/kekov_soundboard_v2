use actix_web::{
    web::{scope, ServiceConfig, Data},
    HttpResponse, post,
};
use sqlx::PgPool;

use crate::error::errors::KekServerError;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/guilds"), // .wrap(AuthService)
    );
}

#[post("")]
pub async fn add_guild(db_pool: Data<PgPool>) -> Result<HttpResponse, KekServerError> {
    todo!()
}
