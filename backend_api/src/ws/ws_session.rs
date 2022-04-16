use std::time::{Duration, Instant};

use actix::{Actor, ActorContext, AsyncContext, Handler, StreamHandler};
use actix_http::ws;
use actix_web_actors::ws::WebsocketContext;
use log::{debug, error, warn};

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(10);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(20);

pub struct ControlsSession {
    heartbeat: Instant,
}

impl ControlsSession {
    pub fn new() -> Self {
        return Self {
            heartbeat: Instant::now(),
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
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for ControlsSession {
    fn handle(&mut self, item: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        let msg = match item {
            Ok(msg) => msg,
            Err(err) => {
                error!("{}", err);
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
                debug!("I'm pogging");
                self.heartbeat = Instant::now();
            }
            _ => unimplemented!(),
        }
    }
}
