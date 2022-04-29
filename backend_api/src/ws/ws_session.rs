use std::{
    collections::HashMap,
    time::{Duration, Instant},
};

use actix::{
    fut, Actor, ActorContext, ActorFutureExt, Addr, AsyncContext, ContextFutureSpawner, Handler,
    StreamHandler, WrapFuture,
};
use actix_http::ws;
use actix_web::web::Data;
use actix_web_actors::ws::WebsocketContext;
use log::{error, warn, debug};
use tokio::sync::{oneshot::Sender, RwLock};
use uuid::Uuid;

use crate::error::errors::KekServerError;

use super::ws_server::{
    ClientError, Connect, ControlsServer, ControlsServerMessage, Disconnect, OpCode,
};

pub type WsSessionCommChannels = RwLock<HashMap<u128, Sender<Result<(), ClientError>>>>;

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(10);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(20);

pub struct ControlsSession {
    id: u128,
    heartbeat: Instant,
    server_address: Addr<ControlsServer>,
    communication_channels: Data<WsSessionCommChannels>,
}

impl ControlsSession {
    pub fn new(
        server_address: Addr<ControlsServer>,
        communication_channels: Data<WsSessionCommChannels>,
    ) -> Self {
        return Self {
            id: Uuid::new_v4().as_u128(),
            heartbeat: Instant::now(),
            server_address,
            communication_channels,
        };
    }

    fn heartbeat(&self, ctx: &mut <Self as Actor>::Context) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |actor, context| {
            if Instant::now().duration_since(actor.heartbeat) > CLIENT_TIMEOUT {
                warn!("ControlsSession heartbeat failed, disconnecting client!");
                context.stop();
            } else {
                context.ping(b"");
            }
        });
    }

    async fn handle_message(msg: ControlsServerMessage, channels: Data<WsSessionCommChannels>) {
        // TODO: make sender actually usefull info ?
        let sender;
        {
            let mut lock = channels.write().await;
            sender = match lock.remove(&msg.get_id()) {
                Some(s) => s,
                None => return error!("WsSession lock error: Id not found!"),
            };
        }

        match msg.get_op_code() {
            OpCode::PlayResponse => match sender.send(Ok(())) {
                Ok(_) => (),
                Err(_) => return error!("WsSession sender failed!\nPossible receiver dropped!"),
            },
            OpCode::StopResponse => match sender.send(Ok(())) {
                Ok(_) => (),
                Err(_) => return error!("WsSession sender failed!\nPossible receiver dropped!"),
            },
            OpCode::Error => match sender.send(Err(msg.get_error())) {
                Ok(_) => (),
                Err(_) => return error!("WsSession sender failed!\nPossible receiver dropped!"),
            },
            _ => (),
        }
    }
}

impl Actor for ControlsSession {
    type Context = WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.heartbeat(ctx);

        let address = ctx.address();
        self.server_address
            .send(Connect::new(address, self.id))
            .into_actor(self)
            .then(|resp, actor, ctx| {
                return fut::ready(());
            })
            .wait(ctx);
    }

    fn stopping(&mut self, _: &mut Self::Context) -> actix::Running {
        info!("Stopping sessions websocket");
        return actix::Running::Stop;
    }
}

impl Handler<ControlsServerMessage> for ControlsSession {
    type Result = ();

    fn handle(&mut self, msg: ControlsServerMessage, ctx: &mut Self::Context) -> Self::Result {
        // TODO: Optimize, make nicer
        match msg.get_op_code() {
            OpCode::Play => {
                match serde_json::to_string(&msg) {
                    Ok(pl) => ctx.text(pl),
                    Err(e) => error!("ControlsSession [Play] control send error: {}", e),
                };
            }
            OpCode::Stop => {
                match serde_json::to_string(&msg) {
                    Ok(stop) => ctx.text(stop),
                    Err(e) => error!("ControlsSession [Stop] control send error: {}", e),
                };
            }
            OpCode::Connection => {
                match serde_json::to_string(&msg) {
                    Ok(m) => ctx.text(m),
                    Err(e) => error!("ControlsSession [Connection] send error: {}", e),
                };
            }
            _ => ctx.text("Poggers"),
        }
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for ControlsSession {
    fn handle(&mut self, item: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        let msg = match item {
            Ok(msg) => msg,
            Err(err) => {
                error!("ControlsSession actor error: {}", err);
                ctx.stop();
                return;
            }
        };

        match msg {
            ws::Message::Ping(msg) => {
                self.heartbeat = Instant::now();
                ctx.pong(&msg);
            }
            ws::Message::Pong(_) => {
                self.heartbeat = Instant::now();
            }
            ws::Message::Close(reason) => {
                self.server_address.do_send(Disconnect { id: self.id });
                ctx.close(reason);
                ctx.stop();
            }
            ws::Message::Text(msg) => {
                let channels = self.communication_channels.clone();
                async move {
                    let control_message: ControlsServerMessage = match serde_json::from_str(&msg) {
                        Ok(c) => c,
                        Err(e) => return error!("WsSession Error: {}", e),
                    };

                    ControlsSession::handle_message(control_message, channels).await;
                }
                .into_actor(self)
                .wait(ctx);
            }
            _ => (),
        }
    }
}
