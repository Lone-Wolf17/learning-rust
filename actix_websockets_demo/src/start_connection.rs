use actix::Addr;
use actix_web::web::{self, Data, Payload, Json};
use actix_web::{get, Error, HttpRequest, HttpResponse, post};
use actix_web_actors::ws;
use serde_json::json;
use uuid::Uuid;

use crate::messages::{BroadcastMessage, StatisticRecord};
use crate::{lobby::Lobby, web_socket_session::WebSocketSession};

#[get("/{topic_name}")]
pub async fn start_connection(
    req: HttpRequest,
    stream: Payload,
    topic_name: web::Path<String>,
    srv: Data<Addr<Lobby>>,
) -> Result<HttpResponse, Error> {
    println!("client");
    let topic_name = topic_name.into_inner();
    let ws = WebSocketSession::new(topic_name, srv.get_ref().clone());

    let resp = ws::start(ws, &req, stream)?;

    Ok(resp)
}

#[post("/dailyDashBoard")]
pub async fn send_statistics (websocket_srv: Data<Addr<Lobby>>, params: Json<Vec<StatisticRecord>>) -> Result<HttpResponse, Error> {
    let _msg = params.into_inner();

    let msg = BroadcastMessage::new(
        Uuid::parse_str("470bb217-ffa7-43d8-a0cc-b3d30421d1werfw").unwrap(),
        json!(_msg),
        "dailtNews".to_string()
    );

    websocket_srv.do_send(msg);

    return Ok(HttpResponse::Ok().json(()));
}
