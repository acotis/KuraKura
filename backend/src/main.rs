
#[allow(unused)]

use std::sync::Mutex;
use axum::{extract::ws::{WebSocketUpgrade, WebSocket}, routing::get, response::{Response}, Router};
//use axum::{extract::ws::{WebSocketUpgrade, WebSocket}, routing::get, response::{IntoResponse, Response}, Router, Json};
//use serde::Serialize;
//use kurakura::server::{Server, UserOk::*, UserResponse, SocketId};

//static mut X: Mutex<u32> = Mutex::new(0);

static mut X: u32 = 0;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(handler));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn handler(ws: WebSocketUpgrade) -> Response {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    while let Some(msg) = socket.recv().await {
        let msg = if let Ok(msg) = msg {
            msg
        } else {
            // client disconnected
            return;
        };

        unsafe {
            X += 1;
            println!("X = {X}");
        }

        if socket.send(msg).await.is_err() {
            // client disconnected

            println!("couldn't send response because client disconnected");
            return;
        }
    }
}







/*

fn call(server: &mut Server, socket: &SocketId, json: &str) -> UserResponse {
    match server.handle_json(socket, json) {
        Err(_) => panic!("server's response was an error"),
        Ok(response) => {
            let value = serde_json::from_str(&response)
                        .expect("server's response was not JSON representing a valid value");
            println!("Server's response was: {value:?}");
            value
        }
    }
}

fn main() {
    let mut server = Server::new();

    let evan1 = server.new_socket();
    let lynn1 = server.new_socket();
    let lexi1 = server.new_socket();

    let cu  = &format!(r#""Register""#);

    let Ok(AccountRegistered {id: _evan}) = call(&mut server, &evan1, cu) else {panic!();};
    let Ok(AccountRegistered {id: _lynn}) = call(&mut server, &lynn1, cu) else {panic!();};
    let Ok(AccountRegistered {id: _lexi}) = call(&mut server, &lexi1, cu) else {panic!();};

    let sn1 = &format!(r#"{{"SetName": {{"name": "Evan is my name"}}}}"#);
    let sn2 = &format!(r#"{{"SetName": {{"name": "Laqme"}}}}"#);
    let sn3 = &format!(r#"{{"SetName": {{"name": "The Lex"}}}}"#);

    let Ok(Okay) = call(&mut server, &evan1, sn1) else {panic!();};
    let Ok(Okay) = call(&mut server, &lynn1, sn2) else {panic!();};
    let Ok(Okay) = call(&mut server, &lexi1, sn3) else {panic!();};

    let cr = &format!(r#""CreateRoom""#);

    let Ok(RoomCreated {id:  room1}) = call(&mut server, &lynn1, cr) else {panic!();};
    let Ok(RoomCreated {id: _room2}) = call(&mut server, &lexi1, cr) else {panic!();};

    let jr  = &format!(r#"{{"JoinRoom": {{"room": "{room1}"}}}}"#);

    let Ok(Okay) = call(&mut server, &evan1, jr) else {panic!();};

    let tt1 = &format!(r#"{{"TakeTurn": {{"turn": {{"player": "Black", "play_row": 0, "play_col": 0, "spin_ul_row": 0, "spin_ul_col": 0, "spin_size": 3, "spin_dir": "CW"}}}}}}"#);
    let tt2 = &format!(r#"{{"TakeTurn": {{"turn": {{"player": "White", "play_row": 1, "play_col": 2, "spin_ul_row": 1, "spin_ul_col": 2, "spin_size": 1, "spin_dir": "CCW"}}}}}}"#);

    let Ok(Okay) = call(&mut server, &lynn1, tt1) else {panic!();};
    let Ok(Okay) = call(&mut server, &evan1, tt2) else {panic!();};

    print!("{server}");

    /*
    println!("{}", cu);
    println!("{}", sn1);
    println!("{}", sn2);
    println!("{}", sn3);
    println!("{}", cr1);
    println!("{}", cr2);
    println!("{}", jr);
    println!("{}", tt1);
    println!("{}", tt2);
    */
}
*/


