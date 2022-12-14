use actix::{fut, ActorContext, ActorFuture, ContextFutureSpawner, Handler, WrapFuture};
use actix::{Actor, Addr, AsyncContext, Running, StreamHandler};
use actix_web_actors::ws;
use actix_web_actors::ws::{Message::Text, ProtocolError, WebsocketContext};
use std::time::{Duration, Instant};
use uuid::Uuid;

use crate::lobby::{Lobby};
use crate::messages::{BroadcastMessage, Connect, Disconnect, WsMessage};

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

pub struct WebSocketSession {
    room: String,
    lobby_addr: Addr<Lobby>,
    hb: Instant,
    id: Uuid,
}

impl WebSocketSession {
    pub fn new (room: String, lobby: Addr<Lobby>) -> WebSocketSession {
        WebSocketSession { room, lobby_addr: lobby, hb: Instant::now(), id: Uuid::new_v4() }
    }
    
    fn hb(&self, ctx: &mut WebsocketContext<Self>) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                println!("Disconnecting failed heartbeat");
                act.lobby_addr.do_send(Disconnect {
                    id: act.id,
                    room_id: act.room.clone(),
                });

                ctx.stop();

                return;
            }

            ctx.ping(b"hi");
        });
    }
}

impl Actor for WebSocketSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);
        let addr = ctx.address();
        self.lobby_addr
            .send(Connect {
                addr: addr.recipient(),
                lobby_id: self.room.clone(),
                self_id: self.id,
            })
            .into_actor(self)
            .then(|res, _, ctx| {
                match res {
                    Ok(_res) => (),
                    _ => ctx.stop(),
                }

                fut::ready(())
            })
            .wait(ctx);
    }

    fn stopping(&mut self, _: &mut Self::Context) -> Running{
        self.lobby_addr.do_send(Disconnect {
            id: self.id,
            room_id: self.room.clone(),
        });

        Running::Stop
    }
}

impl StreamHandler<Result<ws::Message, ProtocolError>> for WebSocketSession {
    fn handle(&mut self, msg: Result<ws::Message, ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }

            Ok(ws::Message::Pong(_)) => {
                self.hb = Instant::now();
            }

            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),

            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }

            Ok(ws::Message::Continuation(_)) => {
                ctx.stop();
            }

            Ok(ws::Message::Nop) => (),

            Ok(Text(s)) => self.lobby_addr.do_send(BroadcastMessage {
                id: self.id,
                msg: serde_json::Value::String(s),
                room_id: self.room.clone(),
            }),

            Err(e) => panic!("{e}"),
        }
    }
}

impl Handler<WsMessage> for WebSocketSession {
    type Result = ();

    fn handle(&mut self, msg: WsMessage, ctx: &mut Self::Context) -> Self::Result {
        ctx.text(msg.0);
    }
}
