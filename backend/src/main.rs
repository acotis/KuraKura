
#[allow(unused)]

use axum::{
    extract::ws::{WebSocketUpgrade, WebSocket},
    routing::get,
    response::{IntoResponse, Response},
    Router,
    Json,
};

use serde::Serialize;
use std::collections::HashMap;

use kurakura::server::{
    Server,
    KuraKuraRequest::{self, *},
    KuraKuraOk::{self, *},
    KuraKuraResponse,
};

fn main() -> KuraKuraResponse {
    let mut server = Server::new();

    let UserCreated {id: evan} = server.handle_request(CreateUser {})? else {panic!();};
    let UserCreated {id: lynn} = server.handle_request(CreateUser {})? else {panic!();};
    let UserCreated {id: lexi} = server.handle_request(CreateUser {})? else {panic!();};

    server.handle_request(SetName {auth: evan.clone(), name: "Evan is my name".into()})?;
    server.handle_request(SetName {auth: lynn.clone(), name: "Laqme".into()})?;
    server.handle_request(SetName {auth: lexi.clone(), name: "The Lex".into()})?;

    let RoomCreated {id: room1} = server.handle_request(CreateRoom {auth: lynn.clone()})? else {panic!();};
    let RoomCreated {id: room2} = server.handle_request(CreateRoom {auth: lexi.clone()})? else {panic!();};
    
    server.handle_request(JoinRoom {auth: evan.clone(), room: room1.clone()});

    print!("{server}");

    Ok(TurnTaken {})
}


/*
#[tokio::main]
async fn main() {
    //let app = Router::new().route("/", get(|| async { "Secret string for Lynn" }));
    let app = Router::new().route("/", get(handler));
    //let app = Router::new().route("/", get(send_json));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[derive(Serialize)]
struct MyStruct {
    first_field: u32,
    second_field: String,
}

async fn send_json() -> Json<MyStruct> {
    Json(MyStruct { first_field: 1, second_field: "hello".into() })
}


async fn handler(ws: WebSocketUpgrade) -> Response {
    println!("handler");
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    println!("handle socket");
    socket.send("The password is fire".into());

    while let Some(msg) = socket.recv().await {
        let msg = if let Ok(msg) = msg {
            msg
        } else {
            // client disconnected
            return;
        };

        println!("{msg:?}");

        if socket.send(msg).await.is_err() {
            // client disconnected
            return;
        }
    }
}
*/
