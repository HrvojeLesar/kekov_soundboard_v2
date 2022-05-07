use std::{collections::HashMap, fmt::Display};

use actix::{
    fut, Actor, ActorContext, ActorFutureExt, Addr, AsyncContext, Context, ContextFutureSpawner,
    Handler, Message, Recipient, Supervised, Supervisor, WrapFuture,
};
use actix_web_actors::ws::WebsocketContext;
use log::{debug, info, warn};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

use crate::{
    error::errors::KekServerError,
    models::{ids::{ChannelId, GuildId, SoundFileId}, sound_file::SoundFile},
};

use super::ws_session::ControlsSession;

#[derive(Message)]
#[rtype(result = "()")]
pub struct Connect {
    id: u128,
    address: Addr<ControlsSession>,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub id: u128,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PlayControl {
    // submitted_by: u64,
    guild_id: GuildId,
    file_id: SoundFileId,
    voice_channel_id: Option<ChannelId>,
}

impl PlayControl {
    pub fn new(
        guild_id: GuildId,
        file_id: SoundFileId,
        voice_channel_id: Option<ChannelId>,
    ) -> Self {
        return Self {
            guild_id,
            file_id,
            voice_channel_id,
        };
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StopControl {
    guild_id: GuildId,
}

impl StopControl {
    pub fn new(guild_id: GuildId) -> Self {
        return Self { guild_id };
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SkipControl {
    guild_id: GuildId,
}

impl SkipControl {
    pub fn new(guild_id: GuildId) -> Self {
        return Self { guild_id };
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct QueueControl {
    guild_id: GuildId,
}

impl QueueControl {
    pub fn new(guild_id: GuildId) -> Self {
        return Self { guild_id };
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum OpCode {
    Connection,
    Play,
    Stop,
    Skip,
    GetQueue,
    PlayResponse,
    StopResponse,
    SkipResponse,
    GetQueueResponse,
    Error,
}

impl Display for OpCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OpCode::Connection => write!(f, "Connection"),
            OpCode::Play => write!(f, "Play"),
            OpCode::Stop => write!(f, "Stop"),
            OpCode::Skip => write!(f, "Skip"),
            OpCode::GetQueue => write!(f, "GetQueue"),
            OpCode::PlayResponse => write!(f, "PlayResponse"),
            OpCode::StopResponse => write!(f, "StopResponse"),
            OpCode::SkipResponse => write!(f, "SkipResponse"),
            OpCode::GetQueueResponse => write!(f, "GetQueueResponse"),
            OpCode::Error => write!(f, "Error"),
        }
    }
}

#[derive(Clone, Debug, Error, Serialize, Deserialize)]
pub enum ClientError {
    #[error("Invalid guild id error")]
    InvalidGuildId,
    #[error("Guild not found error")]
    GuildNotFound,
    #[error("Channnel not found error")]
    ChannelNotFound,
    #[error("Channels empty error")]
    ChannelsEmpty,
    #[error("Connection not established error")]
    LavalinkConnectionNotEstablished,
    #[error("Invalid voice channel error")]
    InvalidVoiceChannel,
    #[error("Failed to load file error")]
    FileLoadingFailed,
    #[error("Invalid file id error")]
    InvalidFileId,
    #[error("Nothing playing")]
    NotPlaying,
    #[serde(other)]
    #[error("Unknown error")]
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Controls {
    Play(PlayControl),
    Stop(StopControl),
    Skip(SkipControl),
    GetQueue(QueueControl),
}

#[derive(Clone, Debug, Message, Serialize, Deserialize)]
#[rtype(result = "()")]
pub struct ControlsServerMessage {
    op: OpCode,
    message_id: u128,
    #[serde(flatten)]
    control: Option<Controls>,
    #[serde(skip_serializing)]
    client_error: Option<ClientError>,
    #[serde(skip_serializing)]
    pub queue: Option<Vec<SoundFile>>,
}

impl ControlsServerMessage {
    pub fn get_op_code(&self) -> &OpCode {
        return &self.op;
    }

    pub fn get_id(&self) -> u128 {
        return self.message_id.clone();
    }

    pub fn get_error(self) -> ClientError {
        match self.client_error {
            Some(e) => return e,
            None => return ClientError::Unknown,
        }
    }

    pub fn new_connect() -> Self {
        return Self {
            op: OpCode::Connection,
            message_id: Uuid::new_v4().as_u128(),
            control: None,
            client_error: None,
            queue: None,
        };
    }

    pub fn new_play(guild_id: GuildId, file_id: SoundFileId) -> Self {
        return Self {
            op: OpCode::Play,
            message_id: Uuid::new_v4().as_u128(),
            control: Some(Controls::Play(PlayControl::new(guild_id, file_id, None))),
            client_error: None,
            queue: None,
        };
    }

    pub fn new_stop(guild_id: GuildId) -> Self {
        return Self {
            op: OpCode::Stop,
            message_id: Uuid::new_v4().as_u128(),
            control: Some(Controls::Stop(StopControl::new(guild_id))),
            client_error: None,
            queue: None,
        };
    }

    pub fn new_skip(guild_id: GuildId) -> Self {
        return Self {
            op: OpCode::Skip,
            message_id: Uuid::new_v4().as_u128(),
            control: Some(Controls::Skip(SkipControl::new(guild_id))),
            client_error: None,
            queue: None,
        };
    }

    pub fn new_queue(guild_id: GuildId) -> Self {
        return Self {
            op: OpCode::GetQueue,
            message_id: Uuid::new_v4().as_u128(),
            control: Some(Controls::GetQueue(QueueControl::new(guild_id))),
            client_error: None,
            queue: None,
        };
    }
}

impl Connect {
    pub fn new(address: Addr<ControlsSession>, id: u128) -> Self {
        return Self { id, address };
    }
}

pub struct ControlsServer {
    clients: HashMap<u128, Addr<ControlsSession>>,
}

impl ControlsServer {
    pub fn new() -> Addr<Self> {
        debug!("New Controls server");
        let server = Self {
            clients: HashMap::new(),
        };

        return server.start_supervisor();
    }

    fn start_supervisor(self) -> Addr<Self> {
        return Supervisor::start(|_| self);
    }

    pub fn send_command(&self, command: ControlsServerMessage) {
        for (_, addr) in self.clients.iter() {
            addr.do_send(command.clone());
        }
    }
}

impl Supervised for ControlsServer {
    fn restarting(&mut self, _ctx: &mut <Self as Actor>::Context) {
        debug!("Superviser: Restarting ControlsServer");
    }
}

impl Actor for ControlsServer {
    type Context = Context<Self>;

    fn stopping(&mut self, _: &mut Self::Context) -> actix::Running {
        info!("Stopping sessions server websocket");
        return actix::Running::Stop;
    }
}

impl Handler<Connect> for ControlsServer {
    type Result = ();

    fn handle(&mut self, msg: Connect, _ctx: &mut Self::Context) -> Self::Result {
        self.clients.insert(msg.id, msg.address);
        let last_address = self.clients.get(&msg.id);
        match last_address {
            Some(addr) => addr.do_send(ControlsServerMessage::new_connect()),
            None => (),
        }
    }
}

impl Handler<Disconnect> for ControlsServer {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _ctx: &mut Self::Context) -> Self::Result {
        self.clients.remove(&msg.id);
    }
}

impl Handler<ControlsServerMessage> for ControlsServer {
    type Result = ();

    // TODO: If there are no active ws connections return error
    fn handle(&mut self, msg: ControlsServerMessage, _ctx: &mut Self::Context) -> Self::Result {
        self.send_command(msg);
    }
}
