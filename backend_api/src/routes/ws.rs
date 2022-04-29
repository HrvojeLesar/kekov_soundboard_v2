use std::sync::Arc;

use actix::Addr;
use actix_web::{
    get,
    web::{scope, Payload, ServiceConfig, Data},
    HttpRequest, HttpResponse,
};
use actix_web_actors::ws;
use sqlx::PgPool;

use crate::{error::errors::KekServerError, ws::{ws_session::{ControlsSession, WsSessionCommChannels}, ws_server::ControlsServer, ws_sync::SyncSession}, utils::cache::UserGuildsCache};

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/ws")
            // .wrap(AuthService)
            .service(controls_ws)
            .service(sync_ws),
    );
}

#[get("controls")]
pub async fn controls_ws(
    request: HttpRequest,
    stream: Payload,
    server_address: Data<Addr<ControlsServer>>,
    ws_channels: Data<WsSessionCommChannels>,
) -> Result<HttpResponse, KekServerError> {
    let address = server_address.get_ref().clone();
    return Ok(ws::start(ControlsSession::new(address, ws_channels), &request, stream)?);
}

#[get("sync")]
pub async fn sync_ws(
    request: HttpRequest,
    stream: Payload,
    user_guilds_cache: Data<UserGuildsCache>,
    db_pool: Data<PgPool>,
) -> Result<HttpResponse, KekServerError> {
    return Ok(ws::start(SyncSession::new(user_guilds_cache, db_pool), &request, stream)?);
}
