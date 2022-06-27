use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};

use actix::{
    Actor, ActorContext, Addr, AsyncContext, ContextFutureSpawner, Handler, Message, StreamHandler,
    WrapFuture,
};
use actix_http::ws;

use actix_web::web::Data;
use actix_web_actors::ws::WebsocketContext;
use log::{debug, error, info, warn};
use serde::{Deserialize, Serialize};
use tokio::sync::{oneshot::Sender, RwLock};
use uuid::Uuid;

use crate::{
    models::ids::GuildId,
    utils::{
        auth::{AccessToken, AuthorizedUser},
        cache::UserGuildsCache,
    },
    ws::channels_server::Unsubscribe,
};

use super::channels_server::{
    CacheClearing, ChannelsServer, Identify, IdentifyResponse, Subscribe,
};

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(10);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(20);

#[derive(Debug, Deserialize)]
enum ChannelsClientOpCode {
    Identify,
    Subscribe,
}

#[derive(Debug, Deserialize)]
struct ClientMessage {
    op: ChannelsClientOpCode,
    guild_id: Option<GuildId>,
    access_token: Option<AccessToken>,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct SubscribeResponse {
    pub new_guild: GuildId,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct ChannelsMessage {
    pub channels: String,
}

#[derive(Clone, Debug)]
pub struct ChannelsClient {
    id: u128,
    heartbeat: Instant,
    server_address: Addr<ChannelsServer>,
    current_guild: Option<GuildId>,
    access_token: Option<Arc<AccessToken>>,
    identified: bool,
}

impl ChannelsClient {
    pub fn new(server_address: Addr<ChannelsServer>) -> Self {
        return Self {
            id: Uuid::new_v4().as_u128(),
            heartbeat: Instant::now(),
            server_address,
            current_guild: None,
            identified: false,
            access_token: None,
        };
    }

    fn heartbeat(&self, ctx: &mut <Self as Actor>::Context) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |actor, context| {
            if Instant::now().duration_since(actor.heartbeat) > CLIENT_TIMEOUT {
                warn!("ChannelsClient heartbeat failed, disconnecting client!");
                context.stop();
            } else {
                context.ping(b"");
            }
        });
    }

    fn subscribe(&self, guild_id: GuildId, ctx: &mut <Self as Actor>::Context) {
        if self.identified {
            self.server_address.do_send(Subscribe {
                id: self.id,
                guild: guild_id,
                old_guild: self.current_guild.clone(),
                client: ctx.address(),
                access_token: self.access_token.clone(),
            });
        }
    }
}

impl Actor for ChannelsClient {
    type Context = WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.heartbeat(ctx);
    }

    fn stopping(&mut self, _: &mut Self::Context) -> actix::Running {
        info!("Stopping channels client!");
        self.server_address.do_send(Unsubscribe {
            id: self.id,
            guild: self.current_guild.clone(),
        });
        return actix::Running::Stop;
    }
}

impl Handler<SubscribeResponse> for ChannelsClient {
    type Result = ();

    fn handle(&mut self, msg: SubscribeResponse, _ctx: &mut Self::Context) -> Self::Result {
        debug!("SubscribeResponse");
        self.current_guild = Some(msg.new_guild);
    }
}

impl Handler<ChannelsMessage> for ChannelsClient {
    type Result = ();

    fn handle(&mut self, msg: ChannelsMessage, ctx: &mut Self::Context) -> Self::Result {
        debug!("ChannelsMessage");
        ctx.text(msg.channels);
    }
}

impl Handler<CacheClearing> for ChannelsClient {
    type Result = ();

    fn handle(&mut self, _msg: CacheClearing, ctx: &mut Self::Context) -> Self::Result {
        debug!("CacheClearing");
        self.current_guild = None;
        ctx.text("Disconnected");
    }
}

impl Handler<IdentifyResponse> for ChannelsClient {
    type Result = ();

    fn handle(&mut self, msg: IdentifyResponse, ctx: &mut Self::Context) -> Self::Result {
        if msg.success {
            ctx.text("Identified");
        } else {
            ctx.stop();
        }
        self.identified = msg.success;
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for ChannelsClient {
    fn handle(&mut self, item: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        let msg = match item {
            Ok(msg) => msg,
            Err(err) => {
                error!("ChannelsClient actor error: {}", err);
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
            ws::Message::Text(msg) => {
                let message: ClientMessage = match serde_json::from_str(&msg) {
                    Ok(c) => c,
                    Err(e) => {
                        error!("WsSession Error: {}", e);
                        debug!("{:#?}", &msg);
                        return;
                    }
                };

                match &message.op {
                    ChannelsClientOpCode::Identify => {
                        if let Some(access_token) = message.access_token {
                            self.access_token = Some(Arc::new(access_token));
                            if let Some(access_token) = &self.access_token {
                                self.server_address.do_send(Identify {
                                    access_token: access_token.clone(),
                                    client: ctx.address(),
                                });
                            } else {
                                error!("This path should be unreachable!!!");
                            }
                        } else {
                            error!("Error in Identify: access_token field is not set");
                        }
                    }
                    ChannelsClientOpCode::Subscribe => {
                        if let Some(guild_id) = message.guild_id {
                            self.subscribe(guild_id, ctx);
                        } else {
                            error!("Error in Subscribe: guild_id field is not set");
                        }
                    }
                }
            }
            _ => (),
        }
    }
}
