use std::sync::Arc;

use actix::Addr;
use actix_web::{
    get,
    web::{scope, Payload, ServiceConfig, Data},
    HttpRequest, HttpResponse,
};
use actix_web_actors::ws;

use crate::{error::errors::KekServerError, ws::{ws_session::ControlsSession, ws_server::ControlsServer}};

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(
        scope("/ws")
            // .wrap(AuthService)
            .service(connect_to_websocket),
    );
}

#[get("")]
pub async fn connect_to_websocket(
    request: HttpRequest,
    stream: Payload,
    server_address: Data<Addr<ControlsServer>>,
) -> Result<HttpResponse, KekServerError> {
    let address = server_address.get_ref().clone();
    return Ok(ws::start(ControlsSession::new(address), &request, stream)?);
}
