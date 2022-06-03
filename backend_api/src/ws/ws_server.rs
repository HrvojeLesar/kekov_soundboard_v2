use std::{collections::HashMap, fmt::Display};

use actix::{Actor, Addr, Context, Handler, Message, Supervised, Supervisor};
use log::{debug, info};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

use crate::models::{
    guild_file::GuildFile,
    ids::{ChannelId, GuildId, SoundFileId},
    sound_file::SoundFile,
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
    display_name: String,
}

impl PlayControl {
    pub fn new(guild_file: GuildFile, voice_channel_id: Option<ChannelId>) -> Self {
        // WARN: expects sound_file field in GuildFile to be Some
        return Self {
            guild_id: guild_file.guild_id,
            file_id: guild_file.file_id,
            display_name: guild_file
                .sound_file
                .unwrap()
                .display_name
                .unwrap_or("".to_string()),
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
    PlayResponseQueued,
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
            OpCode::PlayResponseQueued => write!(f, "PlayResponseQueued"),
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
    #[serde(skip_serializing_if = "Option::is_none")]
    client_error: Option<ClientError>,
    #[serde(skip_serializing_if = "Option::is_none")]
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

    pub fn new_play(guild_file: GuildFile, voice_channel_id: Option<ChannelId>) -> Self {
        return Self {
            op: OpCode::Play,
            message_id: Uuid::new_v4().as_u128(),
            control: Some(Controls::Play(PlayControl::new(
                guild_file,
                voice_channel_id,
            ))),
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

#[cfg(test)]
mod tests {

    use chrono::{NaiveDate, NaiveDateTime};

    use crate::{
        models::{
            guild_file::GuildFile,
            ids::{GuildId, SoundFileId},
            sound_file::SoundFile,
        },
        ws::ws_server::{Controls, OpCode},
    };

    use super::ControlsServerMessage;

    const GUILD: GuildId = GuildId(1);
    const FILE: SoundFileId = SoundFileId(1);

    #[test]
    fn test_csm_new_play() {
        let guild_file: GuildFile = GuildFile {
            guild_id: GUILD,
            file_id: FILE,
            time_added: NaiveDateTime::from_timestamp(0, 0),
            is_deleted: false,
            sound_file: Some(SoundFile {
                id: FILE,
                display_name: Some("TestFile".to_string()),
                is_deleted: false,
                time_added: Some(NaiveDateTime::from_timestamp(0, 0)),
                is_public: false,
                owner: None,
            }),
        };
        let play = ControlsServerMessage::new_play(guild_file, None);
        let control = play.control.unwrap();
        assert!(match play.op {
            OpCode::Play => true,
            _ => false,
        });
        assert!(match control {
            Controls::Play(..) => true,
            _ => false,
        });
    }

    #[test]
    fn test_csm_new_stop() {
        let stop = ControlsServerMessage::new_stop(GUILD);
        let control = stop.control.unwrap();
        assert!(match stop.op {
            OpCode::Stop => true,
            _ => false,
        });
        assert!(match control {
            Controls::Stop(..) => true,
            _ => false,
        });
    }

    #[test]
    fn test_csm_new_skip() {
        let skip = ControlsServerMessage::new_skip(GUILD);
        let control = skip.control.unwrap();
        assert!(match skip.op {
            OpCode::Skip => true,
            _ => false,
        });
        assert!(match control {
            Controls::Skip(..) => true,
            _ => false,
        });
    }

    #[test]
    fn test_csm_new_queue() {
        let queue = ControlsServerMessage::new_queue(GUILD);
        let control = queue.control.unwrap();
        assert!(match queue.op {
            OpCode::GetQueue => true,
            _ => false,
        });
        assert!(match control {
            Controls::GetQueue(..) => true,
            _ => false,
        });
    }
}
