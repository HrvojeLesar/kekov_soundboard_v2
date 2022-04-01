use actix_web::{web::{ServiceConfig, scope}, get, HttpResponse};

use crate::{error::errors::KekServerError, middleware::auth_middleware::AuthService};

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(scope("/user").wrap(AuthService).service(get_user_files));
}

#[get("/files")]
pub async fn get_user_files() -> Result<HttpResponse, KekServerError> {

    todo!()
}
