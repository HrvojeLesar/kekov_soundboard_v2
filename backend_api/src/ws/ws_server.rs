use std::fmt::Display;

use actix::{
    fut, Actor, ActorContext, ActorFutureExt, Addr, AsyncContext, Context, ContextFutureSpawner,
    Handler, Message, Recipient, Supervised, Supervisor, WrapFuture,
};
use actix_web_actors::ws::WebsocketContext;
use log::{debug, warn};
use serde::{Deserialize, Serialize};

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
    guild_id: i64,
    file_id: i64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum OpCode {
    Connection,
    Play,
    Stop,
}

#[derive(Clone, Debug, Message, Serialize, Deserialize)]
#[rtype(result = "()")]
pub struct ControlsServerMessage2 {
    op: OpCode,
    #[serde(flatten)]
    control: Option<Controls>,
}

impl ControlsServerMessage2 {
    pub fn get_op_code(&self) -> &OpCode {
        return &self.op;
    }

    pub fn new_connect() -> Self {
        return Self {
            op: OpCode::Connection,
            control: None,
        };
    }

    pub fn new_play(guild_id: i64, file_id: i64) -> Self {
        return Self {
            op: OpCode::Play,
            control: Some(Controls::Play(PlayControl { guild_id, file_id })),
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
