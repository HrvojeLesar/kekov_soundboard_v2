use std::{
    collections::HashMap,
    sync::Arc,
    time::{Duration, Instant},
};

use actix::{
    fut, Actor, ActorContext, ActorFutureExt, Addr, AsyncContext, ContextFutureSpawner, Handler,
    StreamHandler, WrapFuture,
};
use actix_http::ws;
use actix_web::web::Data;
use actix_web_actors::ws::WebsocketContext;
use log::{error, info, warn};
use tokio::sync::{oneshot::Sender, RwLock};
use uuid::Uuid;

use super::ws_server::{
    ClientError, Connect, ControlsServer, ControlsServerMessage, Disconnect, OpCode,
};

pub type WsSessionCommChannels =
    RwLock<HashMap<u128, Sender<Result<ControlsServerMessage, ControlsServerMessage>>>>;

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
        // TODO: handle proper cleanup of stale channels
        let sender;
        {
            let mut lock = channels.write().await;
            sender = match lock.remove(&msg.get_id()) {
                Some(s) => s,
                None => return error!("WsSession lock error: Id not found!"),
            };
        }

        match msg.get_op_code() {
            OpCode::PlayResponse
            | OpCode::StopResponse
            | OpCode::SkipResponse
            | OpCode::GetQueueResponse
            | OpCode::PlayResponseQueued => match sender.send(Ok(msg)) {
                Ok(_) => (),
                Err(_) => return error!("WsSession sender failed!\nPossible receiver dropped!"),
            },
            OpCode::Error => match sender.send(Err(msg)) {
                Ok(_) => (),
                Err(_) => return error!("WsSession sender failed!\nPossible receiver dropped!"),
            },
            _ => return error!("Invalid opcode received"),
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
            .then(|_, _, _| {
                return fut::ready(());
            })
            .wait(ctx);

        let channels = Arc::clone(&self.communication_channels);
        async move {
            {
                channels.write().await.clear();
            }
        }
        .into_actor(self)
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
        match msg.get_op_code() {
            OpCode::Play | OpCode::Stop | OpCode::Connection | OpCode::Skip | OpCode::GetQueue => {
                match serde_json::to_string(&msg) {
                    Ok(msg) => ctx.text(msg),
                    Err(e) => error!(
                        "ControlsSession [{}] control send error: {}",
                        msg.get_op_code(),
                        e
                    ),
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
