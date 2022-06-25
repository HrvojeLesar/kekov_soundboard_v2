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
    ws_sync::{AddGuild, RemoveGuild, SyncSession},
    GuildVoiceChannels,
};

// WARN: Wrap GuildVoiceChannels in Option
type ChannelsServerCache =
    HashMap<GuildId, (HashMap<u128, Addr<ChannelsClient>>, GuildVoiceChannels)>;

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
    pub old_guild: Option<GuildId>,
    pub client: Addr<ChannelsClient>,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Unsubscribe {
    pub id: u128,
    pub guild: Option<GuildId>,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Update {
    pub guild: GuildId,
    pub msg: GuildVoiceChannels,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct CacheClearing {}

pub struct ChannelsServer {
    channels_cache: ChannelsServerCache,
    sync_sessions: HashMap<u128, Addr<SyncSession>>,
}

impl ChannelsServer {
    pub fn new() -> Addr<Self> {
        debug!("New Channels Server");
        let server = Self {
            channels_cache: HashMap::new(),
            sync_sessions: HashMap::new(),
        };

        return server.start_supervisor();
    }

    fn start_supervisor(self) -> Addr<Self> {
        return Supervisor::start(|_| self);
    }

    fn remove_client(&mut self, id: &u128, guild_id: &GuildId) {
        match self.channels_cache.get_mut(guild_id) {
            Some(o) => {
                o.0.remove(id);
                if o.0.len() == 0 {
                    // WARN: Also notify bot of this
                    self.channels_cache.remove(guild_id);
                    for sync_client in self.sync_sessions.values() {
                        sync_client.do_send(RemoveGuild {
                            guild_id: guild_id.clone(),
                        });
                    }
                }
            }
            None => {}
        }
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

    fn handle(&mut self, msg: ConnectSyncSession, _ctx: &mut Self::Context) -> Self::Result {
        debug!("ConnectSyncSession");
        self.sync_sessions.insert(msg.id, msg.address);
    }
}

impl Handler<DisconnectSyncSession> for ChannelsServer {
    type Result = ();

    fn handle(&mut self, msg: DisconnectSyncSession, _ctx: &mut Self::Context) -> Self::Result {
        debug!("DisconnectSyncSession");
        self.sync_sessions.remove(&msg.id);
        if self.sync_sessions.len() == 0 {
            for guild in self.channels_cache.values() {
                for client in guild.0.iter() {
                    client.1.do_send(CacheClearing {});
                }
            }
            self.channels_cache.clear();
            self.channels_cache.shrink_to(100);
        }
    }
}

impl Handler<Subscribe> for ChannelsServer {
    type Result = ();

    fn handle(&mut self, msg: Subscribe, _ctx: &mut Self::Context) -> Self::Result {
        if self.sync_sessions.len() == 0 {
            return;
        }

        debug!("Subscribe");
        let sync = self
            .sync_sessions
            .values()
            .map(|addr| addr.clone())
            .collect::<Vec<Addr<SyncSession>>>();

        // Remove from cache if in cache
        // Insert into new guild
        if let Some(old) = &msg.old_guild {
            self.remove_client(&msg.id, &old);
        }

        let mut channels_message = None;
        if let Some(cache) = self.channels_cache.get_mut(&msg.guild) {
            // Subscribes new client
            cache.0.insert(msg.id, msg.client.clone());
            channels_message = Some(cache.1.clone());
        } else {
            let mut new_client_map = HashMap::new();
            new_client_map.insert(msg.id, msg.client.clone());
            // Subscribes new client
            self.channels_cache.insert(
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
}

impl Handler<Unsubscribe> for ChannelsServer {
    type Result = ();

    fn handle(&mut self, msg: Unsubscribe, _ctx: &mut Self::Context) -> Self::Result {
        debug!("Unsubscribe");
        if let Some(old) = &msg.guild {
            self.remove_client(&msg.id, &old);
        }
    }
}

impl Handler<Update> for ChannelsServer {
    type Result = ();

    fn handle(&mut self, msg: Update, _ctx: &mut Self::Context) -> Self::Result {
        debug!("Update");
        match self.channels_cache.get_mut(&msg.guild) {
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
}
