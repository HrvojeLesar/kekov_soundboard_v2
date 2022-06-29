use std::{
    sync::Arc,
    time::{Duration, Instant},
};

use actix::{
    fut, Actor, ActorContext, ActorFutureExt, Addr, AsyncContext, ContextFutureSpawner, Handler,
    Message, SpawnHandle, StreamHandler, WrapFuture,
};
use actix_http::ws::{self, CloseCode, CloseReason};

use actix_web_actors::ws::WebsocketContext;
use log::{debug, error, info, warn};
use serde::Deserialize;

use uuid::Uuid;

use crate::{
    models::ids::GuildId,
    utils::{auth::AccessToken, cache::AUTHORIZED_USER_CACHE_TTL},
    ws::channels_server::Unsubscribe,
};

use super::channels_server::{CacheClearing, ChannelsServer, Identify, Subscribe};

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(10);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(20);
const REIDENTIFY_INTERVAL: Duration = Duration::from_secs(AUTHORIZED_USER_CACHE_TTL);
const TIME_TO_IDENTIFY: Duration = Duration::from_secs(20);

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

#[derive(Message)]
#[rtype(result = "()")]
pub struct Removed {}

#[derive(Clone, Debug)]
pub struct ChannelsClient {
    id: u128,
    heartbeat: Instant,
    server_address: Addr<ChannelsServer>,
    current_guild: Option<GuildId>,
    access_token: Option<Arc<AccessToken>>,
    identified: bool,
    reidentify_handle: Option<SpawnHandle>,
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
            reidentify_handle: None,
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

    fn terminate(&self, ctx: &mut <Self as Actor>::Context) {
        ctx.text("TERMINATED");
        ctx.close(Some(CloseReason {
            code: CloseCode::Policy,
            description: Some("Terminated".to_string()),
        }));
        ctx.stop();
    }

    fn reidentify_watcher(&self, ctx: &mut <Self as Actor>::Context) {
        ctx.run_interval(REIDENTIFY_INTERVAL, |actor, context| {
            actor.request_reidentify(context);
            actor.reidentify_handle = Some(context.run_later(TIME_TO_IDENTIFY, |a, c| {
                a.terminate(c);
            }));
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

    fn request_reidentify(&self, ctx: &mut <Self as Actor>::Context) {
        ctx.text("Reidentify");
    }

    fn disconnect(&mut self, ctx: &mut <Self as Actor>::Context) {
        self.current_guild = None;
        ctx.text("Disconnected");
    }
}

impl Actor for ChannelsClient {
    type Context = WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.heartbeat(ctx);
        self.reidentify_watcher(ctx);
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
        self.disconnect(ctx);
    }
}

impl Handler<Removed> for ChannelsClient {
    type Result = ();

    fn handle(&mut self, _msg: Removed, ctx: &mut Self::Context) -> Self::Result {
        debug!("Removed");
        self.disconnect(ctx);
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
                        error!("ChannelsClient Error: {}", e);
                        debug!("{:#?}", &msg);
                        return;
                    }
                };

                match &message.op {
                    ChannelsClientOpCode::Identify => {
                        if let Some(access_token) = message.access_token {
                            self.access_token = Some(Arc::new(access_token));
                            if let Some(access_token) = &self.access_token {
                                self.server_address
                                    .send(Identify {
                                        access_token: access_token.clone(),
                                        client: ctx.address(),
                                    })
                                    .into_actor(self)
                                    .then(|resp, act, ctx| {
                                        match resp {
                                            Ok(identified) => {
                                                if identified {
                                                    if let Some(handle) = act.reidentify_handle {
                                                        ctx.cancel_future(handle);
                                                        act.reidentify_handle = None;
                                                    } else {
                                                        ctx.text("Identified");
                                                    }
                                                } else {
                                                    act.terminate(ctx);
                                                }
                                                act.identified = identified;
                                            }
                                            Err(e) => {
                                                error!("ChannelsClient Identify Error: {}", e);
                                                act.terminate(ctx);
                                            }
                                        }
                                        return fut::ready(());
                                    })
                                    .wait(ctx);
                            } else {
                                error!("This path should be unreachable!!!");
                                error!("This path should be unreachable!!!");
                                error!("This path should be unreachable!!!");
                                error!("This path should be unreachable!!!");
                                error!("This path should be unreachable!!!");
                                error!("This path should be unreachable!!!");
                                error!("This path should be unreachable!!!");
                                error!("This path should be unreachable!!!");
                                error!("This path should be unreachable!!!");
                                error!("This path should be unreachable!!!");
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
