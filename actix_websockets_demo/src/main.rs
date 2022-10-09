mod lobby;
mod messages;
mod start_connection;
mod web_socket_session;

use actix::Actor;
use actix_web::{App, HttpServer};
use lobby::Lobby;
use start_connection::{send_statistics, start_connection as start_connection_route};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let chat_server = Lobby::default().start(); // create  and spin up a lobby

    HttpServer::new(move || {
        App::new()
            .service(start_connection_route)
            .service(send_statistics) // register our route
            .data(chat_server.clone()) // rehister the lobby
    })
    .bind("127.0.0.1:8081")?
    .run()
    .await
}
