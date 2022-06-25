use std::{
    sync::Arc,
    time::{Duration, Instant},
};

use actix::{
    fut, Actor, ActorContext, ActorFutureExt, Addr, AsyncContext, ContextFutureSpawner, Handler,
    Message, StreamHandler, WrapFuture,
};
use actix_http::ws;
use actix_web::web::Data;
use actix_web_actors::ws::WebsocketContext;
use log::{error, info, warn, debug};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    models::ids::{GuildId, UserId},
    utils::cache::UserGuildsCache,
    ws::channels_server::DisconnectSyncSession,
};

use super::{
    channels_server::{ChannelsServer, ConnectSyncSession, Update},
    GuildVoiceChannels,
};

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(HEARTBEAT_INTERVAL.as_secs() * 2);

#[derive(Clone, Debug, Serialize, Deserialize)]
enum SyncOpCode {
    UpdateUserCache,
    InvalidateGuildsCache,
    UpdateGuildChannels,
    AddGuild,
    RemoveGuild,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct SyncMessage {
    op: SyncOpCode,
    user_id: Option<UserId>,
    guild_id: Option<GuildId>,
    guild_voice_channels: Option<GuildVoiceChannels>,
}

#[derive(Message, Serialize)]
#[rtype(result = "()")]
pub struct AddGuild {
    pub guild_id: GuildId,
}

#[derive(Message, Serialize)]
#[rtype(result = "()")]
pub struct RemoveGuild {
    pub guild_id: GuildId,
}

pub struct SyncSession {
    id: u128,
    heartbeat: Instant,
    user_guilds_cache: Data<UserGuildsCache>,
    channels_server: Data<Addr<ChannelsServer>>,
    db_pool: Data<PgPool>,
}

impl SyncSession {
    pub fn new(
        user_guilds_cache: Data<UserGuildsCache>,
        db_pool: Data<PgPool>,
        channels_server: Data<Addr<ChannelsServer>>,
    ) -> Self {
        return Self {
            id: Uuid::new_v4().as_u128(),
            heartbeat: Instant::now(),
            user_guilds_cache,
            channels_server,
            db_pool,
        };
    }

    fn heartbeat(&self, ctx: &mut <Self as Actor>::Context) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |actor, context| {
            if Instant::now().duration_since(actor.heartbeat) > CLIENT_TIMEOUT {
                warn!("SyncSession heartbeat failed, disconnecting client!");
                context.stop();
            } else {
                context.ping(b"");
            }
        });
    }
}

impl Actor for SyncSession {
    type Context = WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.user_guilds_cache.invalidate_all();
        self.heartbeat(ctx);

        let address = ctx.address();
        self.channels_server
            .send(ConnectSyncSession {
                id: self.id,
                address,
            })
            .into_actor(self)
            .then(|_, _, _| {
                return fut::ready(());
            })
            .wait(ctx);

        // ctx.notify(AddGuild {
        //     guild_id: GuildId(679094912179765271),
        // });
    }

    fn stopping(&mut self, ctx: &mut Self::Context) -> actix::Running {
        info!("Stopping sync websocket");
        self.user_guilds_cache.invalidate_all();
        self.channels_server
            .do_send(DisconnectSyncSession { id: self.id });
        return actix::Running::Stop;
    }
}

impl Handler<AddGuild> for SyncSession {
    type Result = ();

    fn handle(&mut self, msg: AddGuild, ctx: &mut Self::Context) -> Self::Result {
        debug!("AddGuild");
        match serde_json::to_string(&SyncMessage {
            op: SyncOpCode::AddGuild,
            guild_id: Some(msg.guild_id),
            user_id: None,
            guild_voice_channels: None,
        }) {
            Ok(m) => {
                ctx.text(m);
            }
            Err(e) => {
                error!("{}", e);
            }
        }
    }
}

impl Handler<RemoveGuild> for SyncSession {
    type Result = ();

    fn handle(&mut self, msg: RemoveGuild, ctx: &mut Self::Context) -> Self::Result {
        debug!("RemoveGuild");
        match serde_json::to_string(&SyncMessage {
            op: SyncOpCode::RemoveGuild,
            guild_id: Some(msg.guild_id),
            user_id: None,
            guild_voice_channels: None,
        }) {
            Ok(m) => {
                ctx.text(m);
            }
            Err(e) => {
                error!("{}", e);
            }
        }
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for SyncSession {
    fn handle(&mut self, item: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        let msg = match item {
            Ok(msg) => msg,
            Err(err) => {
                error!("SyncSession actor error: {}", err);
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
                let user_guilds_cache = Arc::clone(&self.user_guilds_cache);
                let channels_server = self.channels_server.clone();
                async move {
                    // TODO: remove user from cache on guild join/leave/kick/ban
                    // TODO: do something similar when bot gets joined/kicked/banned
                    let message: SyncMessage = match serde_json::from_str(&msg) {
                        Ok(m) => m,
                        Err(e) => return error!("WsSync message error: {}", e),
                    };

                    match &message.op {
                        SyncOpCode::UpdateUserCache => {
                            if let Some(id) = &message.user_id {
                                info!("Trying to invalidate user with id: {}", &id.0);
                                user_guilds_cache.invalidate(id).await;
                            }
                        }
                        SyncOpCode::InvalidateGuildsCache => {
                            if let Some(id) = &message.guild_id {
                                info!("Invalidating guilds cache");
                                info!("Bot joined/left guild with id: {:?}", &id.0);
                                user_guilds_cache.invalidate_all();
                            }
                        }
                        SyncOpCode::UpdateGuildChannels => {
                            if let Some(gvc) = message.guild_voice_channels {
                                if let Some(guild_id) = message.guild_id {
                                    channels_server.do_send(Update {
                                        guild: guild_id,
                                        msg: gvc,
                                    });
                                }
                            }
                        }
                        SyncOpCode::AddGuild | SyncOpCode::RemoveGuild => {}
                    }
                }
                .into_actor(self)
                .spawn(ctx);
            }
            _ => (),
        }
    }
}
