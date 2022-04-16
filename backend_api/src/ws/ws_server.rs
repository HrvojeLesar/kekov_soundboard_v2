use actix::{Recipient, Message, Actor, Context, AsyncContext, Addr};
use actix_web_actors::ws::WebsocketContext;

use super::ws_session::ControlsSession;

#[derive(Message)]
#[rtype(result = "()")]
pub struct ControlsServerMessage(pub String);

pub struct ControlsServer {
    sessions: Vec<Addr<ControlsSession>>,
}

impl ControlsServer {
    pub fn new() -> Self {
        return Self {
            sessions: Vec::new(),
        };
    }
}

impl Actor for ControlsServer {
    type Context = Context<Self>;
}
