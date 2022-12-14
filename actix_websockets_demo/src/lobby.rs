use std::collections::{HashMap, HashSet};

use actix::{Actor, Context, Handler, Recipient};
use serde_json::to_string;
use uuid::Uuid;

use crate::messages::{Connect, Disconnect, WsMessage, BroadcastMessage};

type Socket = Recipient<WsMessage>;

pub struct Lobby {
    sessions: HashMap<Uuid, Socket>,       // self id tp self
    rooms: HashMap<String, HashSet<Uuid>>, // room id to list of users id
}

impl Lobby {
    fn send_message(&self, message: &str, id_to: &Uuid) {
        if let Some(socket_recipient) = self.sessions.get(id_to) {
            let _ = socket_recipient.do_send(WsMessage(message.to_owned()));
        } else {
            println!("Attempting to send message but couldn't find user id.");
        }
    }
}

impl Default for Lobby {
    fn default() -> Self {
        Lobby {
            sessions: HashMap::new(),
            rooms: HashMap::new(),
        }
    }
}

impl Actor for Lobby {
    type Context = Context<Self>;
}

impl Handler<Disconnect> for Lobby {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) {
        if self.sessions.remove(&msg.id).is_some() {
            self.rooms
                .get(&msg.room_id)
                .unwrap()
                .iter()
                .filter(|conn_id| *conn_id.to_owned() != msg.id)
                .for_each(|user_id| {
                    self.send_message(&format!("{} disconnected.", &msg.id), user_id)
                });

            if let Some(lobby) = self.rooms.get_mut(&msg.room_id) {
                if lobby.len() > 1 {
                    lobby.remove(&msg.id);
                } else {
                    // Only one in the lobby, remove it entirely
                    self.rooms.remove(&msg.room_id);
                }
            }
        }
    }
}

impl Handler<Connect> for Lobby {
    type Result = ();

    fn handle(&mut self, msg: Connect, _: &mut Self::Context) -> Self::Result {
        self.rooms
            .entry(msg.lobby_id.clone())
            .or_insert_with(HashSet::new)
            .insert(msg.self_id);

        self.rooms
            .get(&msg.lobby_id)
            .unwrap()
            .iter()
            .filter(|conn_id| *conn_id.to_owned() != msg.self_id)
            .for_each(|conn_id| self.send_message(&format!("{} just joined!", msg.self_id), conn_id));

            self.sessions.insert(msg.self_id, msg.addr);

            self.send_message(&format!("your id is {}", msg.self_id), &msg.self_id);
    }
}

impl Handler<BroadcastMessage> for Lobby {
    type Result = ();

    fn handle(&mut self, msg: BroadcastMessage, _: &mut Self::Context) -> Self::Result {
        if let Some(_) = self.sessions.get(&msg.id) {
            self.rooms.get(&msg.room_id).unwrap().iter().for_each(|client| self.send_message(&to_string(&msg).unwrap(), client));
        } else {
            println!("Attempting to send message but couldn't find admin id")
        }
    }
}