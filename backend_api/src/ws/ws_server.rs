use std::fmt::Display;

use actix::{
    fut, Actor, ActorContext, ActorFutureExt, Addr, AsyncContext, Context, ContextFutureSpawner,
    Handler, Message, Recipient, Supervised, Supervisor, WrapFuture,
};
use actix_web_actors::ws::WebsocketContext;
use log::{debug, warn};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

use crate::{error::errors::KekServerError, models::ids::{GuildId, SoundFileId, ChannelId}};

use super::ws_session::ControlsSession;

#[derive(Message)]
#[rtype(result = "()")]
pub struct ControlsServerMessage(pub String);

#[derive(Message)]
#[rtype(result = "()")]
pub struct Connect {
    address: Addr<ControlsSession>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PlayControl {
    // submitted_by: u64,
    guild_id: GuildId,
    file_id: SoundFileId,
    voice_channel_id: Option<ChannelId>,
}

impl PlayControl {
    pub fn new(guild_id: GuildId, file_id: SoundFileId, voice_channel_id: Option<ChannelId>) -> Self {
        return Self {
            guild_id,
            file_id,
            voice_channel_id,
        };
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum OpCode {
    Connection,
    Play,
    Stop,
    PlayResponse,
    StopResponse,
    Error,
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
    #[error("Unknown error")]
    Unknown,
}

#[derive(Clone, Debug, Message, Serialize, Deserialize)]
#[rtype(result = "()")]
pub struct ControlsServerMessage2 {
    op: OpCode,
    message_id: u128,
    #[serde(flatten)]
    control: Option<Controls>,
    client_error: Option<ClientError>,
}

impl ControlsServerMessage2 {
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
        };
    }

    pub fn new_play(guild_id: GuildId, file_id: SoundFileId) -> Self {
        return Self {
            op: OpCode::Play,
            message_id: Uuid::new_v4().as_u128(),
            control: Some(Controls::Play(PlayControl::new(guild_id, file_id, None))),
            client_error: None,
        };
    }

    pub fn new_stop() -> Self {
        return Self {
            op: OpCode::Play,
            message_id: Uuid::new_v4().as_u128(),
            control: Some(Controls::Stop),
            client_error: None,
        };
    }
}

#[derive(Debug, Message, Clone, Serialize, Deserialize)]
#[rtype(result = "u64")]
#[serde(untagged)]
pub enum Controls {
    Play(PlayControl),
    Stop,
    GetQueue,
}

impl Connect {
    pub fn new(address: Addr<ControlsSession>) -> Self {
        return Self { address };
    }
}

pub struct ControlsServer {
    sessions: Vec<Addr<ControlsSession>>,
}

impl ControlsServer {
    pub fn new() -> Addr<Self> {
        debug!("New Controls server");
        let server = Self {
            sessions: Vec::new(),
        };

        return server.start_supervisor();
    }

    fn start_supervisor(self) -> Addr<Self> {
        return Supervisor::start(|_| self);
    }

    pub fn send_command(&self, command: Controls) -> u64 {
        let mut commands_sent = 0;
        for session in &self.sessions {
            session.do_send(command.clone());
            commands_sent += 1;
        }
        return commands_sent;
    }

    pub fn send_command2(&self, command: ControlsServerMessage2) -> u64 {
        let mut commands_sent = 0;
        for session in &self.sessions {
            session.do_send(command.clone());
            commands_sent += 1;
        }
        return commands_sent;
    }
}

impl Supervised for ControlsServer {
    fn restarting(&mut self, _ctx: &mut <Self as Actor>::Context) {
        debug!("Superviser: Restarting ControlsServer");
    }
}

impl Actor for ControlsServer {
    type Context = Context<Self>;
}

impl Handler<Connect> for ControlsServer {
    type Result = ();

    fn handle(&mut self, msg: Connect, _ctx: &mut Self::Context) -> Self::Result {
        self.sessions.push(msg.address);
        let last_address = self.sessions.last();
        match last_address {
            Some(addr) => addr.do_send(ControlsServerMessage2::new_connect()),
            None => (),
        }
    }
}

impl Handler<Controls> for ControlsServer {
    type Result = u64;

    fn handle(&mut self, msg: Controls, _ctx: &mut Self::Context) -> Self::Result {
        return self.send_command(msg);
    }
}

impl Handler<ControlsServerMessage2> for ControlsServer {
    type Result = ();

    fn handle(&mut self, msg: ControlsServerMessage2, _ctx: &mut Self::Context) -> Self::Result {
        self.send_command2(msg);
    }
}
