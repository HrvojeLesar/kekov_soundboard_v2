use std::{collections::HashMap, sync::Arc};

use actix::{
    Actor, Addr, AsyncContext, Context, ContextFutureSpawner, Handler, Message, Supervised,
    Supervisor, WrapFuture,
};

use futures::FutureExt;
use log::{debug, error, info};

use tokio::sync::RwLock;

use crate::models::ids::GuildId;

use super::{
    channels_client::{ChannelsClient, ChannelsMessage, SubscribeResponse},
    ws_sync::{AddGuild, SyncSession},
    GuildVoiceChannels,
};

type ChannelsServerCache =
    Arc<RwLock<HashMap<GuildId, (HashMap<u128, Addr<ChannelsClient>>, GuildVoiceChannels)>>>;

#[derive(Message)]
#[rtype(result = "()")]
pub struct ConnectSyncSession {
    pub id: u128,
    pub address: Addr<SyncSession>,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct DisconnectSyncSession {
    pub id: u128,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Subscribe {
    pub id: u128,
    pub guild: GuildId,
    pub old_guild: Arc<Option<GuildId>>,
    pub client: Addr<ChannelsClient>,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Unsubscribe {
    pub id: u128,
    pub guild: Arc<Option<GuildId>>,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Update {
    pub guild: GuildId,
    pub msg: GuildVoiceChannels,
}

pub struct ChannelsServer {
    channels_cache: ChannelsServerCache,
    sync_sessions: HashMap<u128, Addr<SyncSession>>,
}

impl ChannelsServer {
    pub fn new() -> Addr<Self> {
        debug!("New Channels Server");
        let server = Self {
            channels_cache: Arc::new(RwLock::new(HashMap::new())),
            sync_sessions: HashMap::new(),
        };

        return server.start_supervisor();
    }

    fn start_supervisor(self) -> Addr<Self> {
        return Supervisor::start(|_| self);
    }
}

impl Supervised for ChannelsServer {
    fn restarting(&mut self, _ctx: &mut <Self as Actor>::Context) {
        debug!("Superviser: Restarting ChannelsServer");
    }
}

impl Actor for ChannelsServer {
    type Context = Context<Self>;

    fn stopping(&mut self, _: &mut Self::Context) -> actix::Running {
        info!("Stopping ChannelsServer websocket");
        return actix::Running::Stop;
    }
}

impl Handler<ConnectSyncSession> for ChannelsServer {
    type Result = ();

    fn handle(&mut self, msg: ConnectSyncSession, ctx: &mut Self::Context) -> Self::Result {
        self.sync_sessions.insert(msg.id, msg.address);
    }
}

impl Handler<DisconnectSyncSession> for ChannelsServer {
    type Result = ();

    fn handle(&mut self, msg: DisconnectSyncSession, ctx: &mut Self::Context) -> Self::Result {
        self.sync_sessions.remove(&msg.id);
        async {
            // Disconnect all clients and notfy them about this
            // let mut channels_cache = self.channels_cache.write().await;
        }
        .into_actor(self)
        .wait(ctx);
    }
}

impl Handler<Subscribe> for ChannelsServer {
    type Result = ();

    fn handle(&mut self, msg: Subscribe, ctx: &mut Self::Context) -> Self::Result {
        // Add to another guild
        let cache = self.channels_cache.clone();
        if self.sync_sessions.len() == 0 {
            return;
        }

        let sync = self
            .sync_sessions
            .values()
            .map(|addr| addr.clone())
            .collect::<Vec<Addr<SyncSession>>>();

        async move {
            let mut channels = cache.write().await;
            // Remove from cache if in cache
            // Insert into new guild
            if let Some(old) = msg.old_guild.as_ref() {
                match channels.get_mut(old) {
                    Some(o) => {
                        o.0.remove(&msg.id);
                    }
                    None => {}
                }
            }

            let mut channels_message = None;
            if let Some(cache) = channels.get_mut(&msg.guild) {
                // Subscribes new client
                cache.0.insert(msg.id, msg.client.clone());
                channels_message = Some(cache.1.clone());
            } else {
                let mut new_client_map = HashMap::new();
                new_client_map.insert(msg.id, msg.client.clone());
                // Subscribes new client
                channels.insert(
                    msg.guild.clone(),
                    (new_client_map, GuildVoiceChannels::empty()),
                );
                // Notify bot to fetch this guild and send back
                // ...^^^^
                // WARN: Temporary bandage fix ??
                if let Some(s) = sync.first() {
                    s.do_send(AddGuild {
                        guild_id: msg.guild.clone(),
                    });
                }
            }
            drop(channels);

            msg.client.do_send(SubscribeResponse {
                new_guild: msg.guild,
            });

            if let Some(cm) = channels_message {
                match serde_json::to_string(&cm) {
                    Ok(cm) => {
                        msg.client.do_send(ChannelsMessage { channels: cm });
                    }
                    Err(e) => {
                        error!("{}", e);
                    }
                }
            }
        }
        .into_actor(self)
        .spawn(ctx);
    }
}

impl Handler<Unsubscribe> for ChannelsServer {
    type Result = ();

    fn handle(&mut self, msg: Unsubscribe, ctx: &mut Self::Context) -> Self::Result {
        let cache = self.channels_cache.clone();
        async move {
            let mut channels = cache.write().await;
            // Remove from cache if in cache
            // Insert into new guild
            if let Some(old) = msg.guild.as_ref() {
                match channels.get_mut(old) {
                    Some(o) => {
                        o.0.remove(&msg.id);
                    }
                    None => {}
                }
            }
        }
        .into_actor(self)
        .spawn(ctx);
    }
}

impl Handler<Update> for ChannelsServer {
    type Result = ();

    fn handle(&mut self, msg: Update, ctx: &mut Self::Context) -> Self::Result {
        let cache = self.channels_cache.clone();
        async move {
            let mut channels = cache.write().await;
            match channels.get_mut(&msg.guild) {
                Some(gc) => {
                    gc.1 = msg.msg;
                    match serde_json::to_string(&gc.1) {
                        Ok(cm) => {
                            for client in gc.0.values() {
                                client.do_send(ChannelsMessage {
                                    channels: cm.clone(),
                                });
                            }
                        }
                        Err(e) => {
                            error!("{}", e);
                        }
                    }
                }
                None => {
                    error!("Tried to update a guild that is not cached");
                    return;
                }
            }
        }
        .into_actor(self)
        .spawn(ctx);
    }
}
