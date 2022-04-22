use std::time::{Duration, Instant};

use actix::{
    fut, Actor, ActorContext, ActorFutureExt, Addr, AsyncContext, ContextFutureSpawner, Handler,
    StreamHandler, WrapFuture,
};
use actix_http::ws;
use actix_web_actors::ws::WebsocketContext;
use log::{error, warn};

use super::ws_server::{Connect, Controls, ControlsServer, ControlsServerMessage, ControlsServerMessage2, OpCode};

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(10);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(20);

pub struct ControlsSession {
    heartbeat: Instant,
    server_address: Addr<ControlsServer>,
}

impl ControlsSession {
    pub fn new(server_address: Addr<ControlsServer>) -> Self {
        return Self {
            heartbeat: Instant::now(),
            server_address,
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
}

impl Actor for ControlsSession {
    type Context = WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.heartbeat(ctx);

        let address = ctx.address();
        self.server_address
            .send(Connect::new(address))
            .into_actor(self)
            .then(|resp, actor, ctx| {
                return fut::ready(());
            })
            .wait(ctx);
    }
}

impl Handler<ControlsServerMessage> for ControlsSession {
    type Result = ();

    fn handle(&mut self, msg: ControlsServerMessage, ctx: &mut Self::Context) -> Self::Result {
        ctx.text(msg.0);
    }
}

impl Handler<Controls> for ControlsSession {
    type Result = u64;

    fn handle(&mut self, msg: Controls, ctx: &mut Self::Context) -> Self::Result {
        match msg {
            Controls::Play(p) => {
                match serde_json::to_string(&p) {
                    Ok(pl) => ctx.text(pl), 
                    Err(e) => error!("ControlsSession play control send error: {}", e),
                };
            }
            Controls::Stop => ctx.text("Very pog"),
            Controls::GetQueue => ctx.text("Poggers"),
        }
        0
    }
}

impl Handler<ControlsServerMessage2> for ControlsSession {
    type Result = ();

    fn handle(&mut self, msg: ControlsServerMessage2, ctx: &mut Self::Context) -> Self::Result {
        // TODO: Optimize, make nicer
        match msg.get_op_code() {
            OpCode::Play => {
                match serde_json::to_string(&msg) {
                    Ok(pl) => ctx.text(pl), 
                    Err(e) => error!("ControlsSession play control send error: {}", e),
                };
            },
            OpCode::Connection => {
                match serde_json::to_string(&msg) {
                    Ok(m) => ctx.text(m), 
                    Err(e) => error!("ControlsSession Connection send error: {}", e),
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
                ctx.close(reason);
                ctx.stop();
            }
            _ => (),
        }
    }
}
