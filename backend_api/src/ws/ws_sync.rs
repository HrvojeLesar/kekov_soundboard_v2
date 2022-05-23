use std::{
    sync::Arc,
    time::{Duration, Instant},
};

use actix::{Actor, ActorContext, AsyncContext, ContextFutureSpawner, StreamHandler, WrapFuture};
use actix_http::ws;
use actix_web::web::Data;
use actix_web_actors::ws::WebsocketContext;
use log::{error, info, warn};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::{
    models::ids::{GuildId, UserId},
    utils::cache::UserGuildsCache,
};

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(HEARTBEAT_INTERVAL.as_secs() * 2);

#[derive(Clone, Debug, Serialize, Deserialize)]
enum SyncOpCode {
    UpdateUserCache,
    InvalidateGuildsCache,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct SyncMessage {
    op: SyncOpCode,
    user_id: Option<UserId>,
    guild_id: Option<GuildId>,
}

pub struct SyncSession {
    heartbeat: Instant,
    user_guilds_cache: Data<UserGuildsCache>,
    db_pool: Data<PgPool>,
}

impl SyncSession {
    pub fn new(user_guilds_cache: Data<UserGuildsCache>, db_pool: Data<PgPool>) -> Self {
        return Self {
            heartbeat: Instant::now(),
            user_guilds_cache,
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
    }

    fn stopping(&mut self, _: &mut Self::Context) -> actix::Running {
        info!("Stopping sync websocket");
        self.user_guilds_cache.invalidate_all();
        return actix::Running::Stop;
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
                    }
                }
                .into_actor(self)
                .wait(ctx);
            }
            _ => (),
        }
    }
}
