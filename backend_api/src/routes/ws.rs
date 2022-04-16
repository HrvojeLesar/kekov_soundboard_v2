use actix_web::{
    get,
    web::{scope, Payload, ServiceConfig},
    HttpRequest, HttpResponse,
};
use actix_web_actors::ws;

use crate::{error::errors::KekServerError, ws::ws_session::ControlsSession};

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
) -> Result<HttpResponse, KekServerError> {
    return Ok(ws::start(ControlsSession::new(), &request, stream)?);
}
