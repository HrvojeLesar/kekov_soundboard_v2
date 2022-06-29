use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use actix::{Actor, Addr, Context, Handler, Message, ResponseFuture, Supervised, Supervisor};

use actix_web::web::Data;
use log::{debug, error, info};

use crate::{
    middleware::{authorize_user, cache_authorized_user_guilds},
    models::ids::{GuildId, UserId},
    utils::{
        auth::{AccessToken, AuthorizedUser},
        cache::{
            AuthMiddlewareQueueCache, AuthorizedUsersCache, UserGuildsCache,
            UserGuildsMiddlwareQueueCache,
        },
        validation::Validation,
    },
};

use super::{
    channels_client::{ChannelsClient, ChannelsMessage, Removed, SubscribeResponse},
    ws_sync::{AddGuild, RemoveGuild, SyncSession},
    GuildVoiceChannels,
};

// WARN: Wrap GuildVoiceChannels in Option
type ChannelsServerCache = HashMap<
    GuildId,
    (
        HashMap<u128, (Addr<ChannelsClient>, Arc<AuthorizedUser>)>,
        GuildVoiceChannels,
    ),
>;

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
    pub access_token: Option<Arc<AccessToken>>,
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

#[derive(Message)]
#[rtype(result = "bool")]
pub struct Identify {
    pub client: Addr<ChannelsClient>,
    pub access_token: Arc<AccessToken>,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct InvalidateClient {
    pub user_id: UserId,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct IdentifyResponse {
    pub success: bool,
}

pub struct ChannelsServer {
    channels_cache: ChannelsServerCache,
    sync_sessions: HashMap<u128, Addr<SyncSession>>,
    authorized_users_cache: Data<AuthorizedUsersCache>,
    authorized_users_queue_cache: Data<Mutex<AuthMiddlewareQueueCache>>,
    user_guilds_cache: Data<UserGuildsCache>,
    user_guilds_queue_cache: Data<Mutex<UserGuildsMiddlwareQueueCache>>,
}

impl ChannelsServer {
    pub fn new(
        authorized_users_cache: Data<AuthorizedUsersCache>,
        authorized_users_queue_cache: Data<Mutex<AuthMiddlewareQueueCache>>,
        user_guilds_cache: Data<UserGuildsCache>,
        user_guilds_queue_cache: Data<Mutex<UserGuildsMiddlwareQueueCache>>,
    ) -> Addr<Self> {
        debug!("New Channels Server");
        let server = Self {
            channels_cache: HashMap::new(),
            sync_sessions: HashMap::new(),
            authorized_users_cache,
            authorized_users_queue_cache,
            user_guilds_cache,
            user_guilds_queue_cache,
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

    fn remove_guild(&mut self, msg: RemoveGuild) {
        match self.channels_cache.remove(&msg.guild_id) {
            Some(entry) => {
                let clients = entry.0;
                for client in clients {
                    (client.1).0.do_send(CacheClearing {});
                }
                for sync_client in self.sync_sessions.values() {
                    sync_client.do_send(RemoveGuild {
                        guild_id: msg.guild_id.clone(),
                    });
                }
            }
            None => {}
        }
    }

    fn invalidate_client(&mut self, msg: InvalidateClient) {
        let mut empty_guild_ids = Vec::new();
        for (guild_id, guilds) in self.channels_cache.iter_mut() {
            guilds.0.retain(|_, (c, authorized_user)| {
                if authorized_user.discord_user.id == msg.user_id {
                    c.do_send(Removed {});
                    return false;
                }
                return true;
            });
            if guilds.0.len() == 0 {
                empty_guild_ids.push(guild_id.clone());
            }
        }
        for id in empty_guild_ids {
            for sync_client in self.sync_sessions.iter() {
                sync_client.1.do_send(RemoveGuild {
                    guild_id: id.clone(),
                });
            }
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
                    (client.1).0.do_send(CacheClearing {});
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

        if let Some(old) = &msg.old_guild {
            self.remove_client(&msg.id, &old);
        }

        let access_token = match msg.access_token {
            Some(at) => at,
            None => return,
        };

        let authorized_user = match self.authorized_users_cache.get(&access_token) {
            Some(au) => au,
            None => return,
        };

        if let Err(e) =
            Validation::is_user_in_guild(&authorized_user, &msg.guild, &self.user_guilds_cache)
        {
            error!("WsSession Error: {}", e);
            return;
        }

        debug!("Subscribe");
        let sync = self
            .sync_sessions
            .values()
            .map(|addr| addr.clone())
            .collect::<Vec<Addr<SyncSession>>>();

        let mut channels_message = None;
        if let Some(cache) = self.channels_cache.get_mut(&msg.guild) {
            // Subscribes new client
            cache
                .0
                .insert(msg.id, (msg.client.clone(), authorized_user));
            channels_message = Some(cache.1.clone());
        } else {
            let mut new_client_map = HashMap::new();
            new_client_map.insert(msg.id, (msg.client.clone(), authorized_user));
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
                            client.0.do_send(ChannelsMessage {
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

impl Handler<Identify> for ChannelsServer {
    type Result = ResponseFuture<bool>;

    fn handle(&mut self, msg: Identify, _ctx: &mut Self::Context) -> Self::Result {
        debug!("Identify");
        let authorized_users_cache = self.authorized_users_cache.clone();
        let authorized_users_queue_cache = self.authorized_users_queue_cache.clone();
        let user_guilds_cache = self.user_guilds_cache.clone();
        let user_guilds_queue_cache = self.user_guilds_queue_cache.clone();
        return Box::pin(async move {
            let access_token = msg.access_token;
            let authorized_user = match authorize_user(
                access_token,
                authorized_users_cache,
                authorized_users_queue_cache,
            )
            .await
            {
                Ok(au) => au,
                Err(e) => {
                    error!("ChannelsServer Identify Error (authorize_user): {}", e);
                    return false;
                }
            };

            match cache_authorized_user_guilds(
                &authorized_user,
                user_guilds_cache,
                user_guilds_queue_cache,
            )
            .await
            {
                Ok(_) => return true,
                Err(e) => {
                    error!(
                        "ChannelsServer Identify Error (cache_authorized_user_guilds): {}",
                        e
                    );
                    return false;
                }
            }
        });
    }
}

impl Handler<RemoveGuild> for ChannelsServer {
    type Result = ();

    fn handle(&mut self, msg: RemoveGuild, _ctx: &mut Self::Context) -> Self::Result {
        debug!("RemoveGuild ChannelsServer");
        self.remove_guild(msg);
    }
}

impl Handler<InvalidateClient> for ChannelsServer {
    type Result = ();

    fn handle(&mut self, msg: InvalidateClient, _ctx: &mut Self::Context) -> Self::Result {
        debug!("InvalidateClient");
        self.invalidate_client(msg);
    }
}
