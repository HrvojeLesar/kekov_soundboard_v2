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
    utils::{auth::AuthorizedUser, cache::UserGuildsCache},
    ws::channels_server::Unsubscribe,
};

use super::channels_server::{ChannelsServer, Subscribe};

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(10);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(20);

#[derive(Debug, Serialize, Deserialize)]
struct ListenToGuild {
    guild_id: GuildId,
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
pub struct ChannelClientData {
    pub server_address: Addr<ChannelsServer>,
    // pub user: Arc<AuthorizedUser>,
    // pub user_guilds_cache: Data<UserGuildsCache>,
    pub current_guild: Arc<Option<GuildId>>,
}

pub struct ChannelsClient {
    pub id: u128,
    pub heartbeat: Instant,
    pub data: ChannelClientData,
}

impl ChannelsClient {
    pub fn new(
        server_address: Addr<ChannelsServer>,
        // user: Arc<AuthorizedUser>,
        // user_guilds_cache: Data<UserGuildsCache>,
    ) -> Self {
        return Self {
            id: Uuid::new_v4().as_u128(),
            heartbeat: Instant::now(),
            data: ChannelClientData {
                server_address,
                // user,
                // user_guilds_cache,
                current_guild: Arc::new(None),
            },
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

    async fn handle_message(
        id: u128,
        msg: ListenToGuild,
        data: ChannelClientData,
        addr: Addr<Self>,
    ) {
        // validate that user is allowed in guild
        // subscribe to that guild with server
        data.server_address
            .send(Subscribe {
                id,
                guild: msg.guild_id,
                client: addr,
                old_guild: data.current_guild.clone(),
            })
            .await;
    }
}

impl Actor for ChannelsClient {
    type Context = WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.heartbeat(ctx);
    }

    fn stopping(&mut self, _: &mut Self::Context) -> actix::Running {
        info!("Stopping channels client!");
        self.data.server_address.do_send(Unsubscribe {
            id: self.id,
            guild: self.data.current_guild.clone(),
        });
        return actix::Running::Stop;
    }
}

impl Handler<SubscribeResponse> for ChannelsClient {
    type Result = ();

    fn handle(&mut self, msg: SubscribeResponse, _ctx: &mut Self::Context) -> Self::Result {
        self.data.current_guild = Arc::new(Some(msg.new_guild));
    }
}

impl Handler<ChannelsMessage> for ChannelsClient {
    type Result = ();

    fn handle(&mut self, msg: ChannelsMessage, ctx: &mut Self::Context) -> Self::Result {
        ctx.text(msg.channels);
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
                let data = self.data.clone();
                let id = self.id.clone();
                let addr = ctx.address();
                async move {
                    let message: ListenToGuild = match serde_json::from_str(&msg) {
                        Ok(c) => c,
                        Err(e) => {
                            error!("WsSession Error: {}", e);
                            debug!("{:#?}", &msg);
                            return;
                        }
                    };
                    Self::handle_message(id, message, data, addr).await;
                }
                .into_actor(self)
                .wait(ctx);
            }
            _ => (),
        }
    }
}
