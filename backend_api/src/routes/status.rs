use actix_web::{
    get,
    web::{scope, Data, ServiceConfig},
    HttpResponse,
};
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

use crate::error::errors::KekServerError;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Status {
    pub ws_control_message_channels: usize,
    pub ws_control_clients: usize,
    pub auth_queue_cache: usize,
    pub guilds_queue_cache: usize,

    pub ws_sync_sessions: usize,
    pub ws_guilds_cached: usize,
    pub ws_active_connections: usize,
    pub channels_server_cache_capacity: usize,
}

impl Status {
    pub fn new() -> Self {
        return Self {
            ws_control_message_channels: 0,
            ws_control_clients: 0,
            auth_queue_cache: 0,
            guilds_queue_cache: 0,
            ws_sync_sessions: 0,
            ws_guilds_cached: 0,
            ws_active_connections: 0,
            channels_server_cache_capacity: 0
        };
    }
}

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(scope("/status").service(get_status));
}

#[get("")]
pub async fn get_status(status: Data<RwLock<Status>>) -> Result<HttpResponse, KekServerError> {
    return Ok(HttpResponse::Ok().json(status.read().await.to_owned()));
}
