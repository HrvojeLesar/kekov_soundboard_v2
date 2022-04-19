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

impl PlayControl {
    pub fn new(guild_id: i64, file_id: i64) -> Self {
        return Self { file_id, guild_id };
    }
}

#[derive(Debug, Message, Clone)]
#[rtype(result = "u64")]
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
            Some(addr) => addr.do_send(ControlsServerMessage("Success".to_string())),
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
