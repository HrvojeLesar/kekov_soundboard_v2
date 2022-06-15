use actix_web::{get, web::{ServiceConfig, scope, Data}, HttpResponse};
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

use crate::error::errors::KekServerError;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Status {
    pub ws_channel_num: usize,
}

impl Status {
    pub fn new() -> Self {
        return Self { ws_channel_num: 0 };
    }
}

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(scope("/status").service(get_status));
}

#[get("")]
pub async fn get_status(status: Data<RwLock<Status>>) -> Result<HttpResponse, KekServerError> {
    return Ok(HttpResponse::Ok().json(status.read().await.to_owned()));
}
