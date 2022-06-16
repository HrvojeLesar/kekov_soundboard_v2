use actix::Addr;
use actix_web::{
    get,
    guard::{self, Guard, GuardContext},
    web::{scope, Data, Payload, ServiceConfig},
    HttpRequest, HttpResponse,
};
use actix_web_actors::ws;
use log::info;
use sqlx::PgPool;

use crate::{
    error::errors::KekServerError,
    utils::cache::UserGuildsCache,
    ws::{
        ws_server::ControlsServer,
        ws_session::{ControlsSession, WsSessionCommChannels},
        ws_sync::SyncSession,
    },
};

use lazy_static::lazy_static;

lazy_static! {
    static ref TOKEN: String = dotenv::var("WS_TOKEN").unwrap();
}

fn websocket_token_guard(ctx: &GuardContext) -> bool {
    return guard::Header("X-Ws-Token", &TOKEN).check(ctx);
}

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(scope("/ws").service(controls_ws).service(sync_ws));
}

#[get("/controls", guard = "websocket_token_guard")]
pub async fn controls_ws(
    request: HttpRequest,
    stream: Payload,
    server_address: Data<Addr<ControlsServer>>,
    ws_channels: Data<WsSessionCommChannels>,
) -> Result<HttpResponse, KekServerError> {
    info!("New controls websocket connection");
    let address = server_address.get_ref().clone();
    return Ok(ws::start(
        ControlsSession::new(address, ws_channels),
        &request,
        stream,
    )?);
}

#[get("/sync", guard = "websocket_token_guard")]
pub async fn sync_ws(
    request: HttpRequest,
    stream: Payload,
    user_guilds_cache: Data<UserGuildsCache>,
    db_pool: Data<PgPool>,
) -> Result<HttpResponse, KekServerError> {
    info!("New sync websocket connection");
    return Ok(ws::start(
        SyncSession::new(user_guilds_cache, db_pool),
        &request,
        stream,
    )?);
}
