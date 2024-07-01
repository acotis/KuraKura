
#[allow(unused)]

use axum::{
    extract::ws::{WebSocketUpgrade, WebSocket},
    routing::get,
    response::{IntoResponse, Response},
    Router,
    Json,
};

use serde::Serialize;
use serde_json::to_string;
use std::collections::HashMap;

use kurakura::server::{
    Server,
    KuraKuraRequest::{self, *},
    KuraKuraOk::{self, *},
    KuraKuraResponse,
};

fn main() -> KuraKuraResponse {
    let mut server = Server::new();

    let cu  = &format!(r#"{{"CreateUser": {{}}}}"#);

    let UserCreated {id: evan } = server.handle_json(cu)?  else {panic!();};
    let UserCreated {id: lynn } = server.handle_json(cu)?  else {panic!();};
    let UserCreated {id: lexi } = server.handle_json(cu)?  else {panic!();};

    let sn1 = &format!(r#"{{"SetName": {{"auth": "{evan}", "name": "Evan is my name"}}}}"#);
    let sn2 = &format!(r#"{{"SetName": {{"auth": "{lynn}", "name": "Laqme"}}}}"#);
    let sn3 = &format!(r#"{{"SetName": {{"auth": "{lexi}", "name": "The Lex"}}}}"#);

    let NameSet     {         } = server.handle_json(sn1)? else {panic!();};
    let NameSet     {         } = server.handle_json(sn2)? else {panic!();};
    let NameSet     {         } = server.handle_json(sn3)? else {panic!();};

    let cr1 = &format!(r#"{{"CreateRoom": {{"auth": "{lynn}"}}}}"#);
    let cr2 = &format!(r#"{{"CreateRoom": {{"auth": "{lexi}"}}}}"#);

    let RoomCreated {id: room1} = server.handle_json(cr1)? else {panic!();};
    let RoomCreated {id: room2} = server.handle_json(cr2)? else {panic!();};

    let jr  = &format!(r#"{{"JoinRoom": {{"auth": "{evan}", "room": "{room1}"}}}}"#);

    let RoomJoined  {         } = server.handle_json(jr )? else {panic!();};

    let tt1 = &format!(r#"{{"TakeTurn": {{"auth": "{lynn}", "turn": {{"player": "Black", "play_row": 0, "play_col": 0, "spin_ul_row": 0, "spin_ul_col": 0, "spin_size": 3, "spin_dir": "CW"}}}}}}"#);
    let tt2 = &format!(r#"{{"TakeTurn": {{"auth": "{evan}", "turn": {{"player": "White", "play_row": 1, "play_col": 2, "spin_ul_row": 1, "spin_ul_col": 2, "spin_size": 1, "spin_dir": "CCW"}}}}}}"#);

    let TurnTaken   {         } = server.handle_json(tt1)? else {panic!();};
    let TurnTaken   {         } = server.handle_json(tt2)? else {panic!();};

    print!("{server}");
    println!("{}", cu);
    println!("{}", sn1);
    println!("{}", sn2);
    println!("{}", sn3);
    println!("{}", cr1);
    println!("{}", cr2);
    println!("{}", jr);
    println!("{}", tt1);
    println!("{}", tt2);

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
