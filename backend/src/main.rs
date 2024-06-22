
#[allow(unused)]

use axum::{
    extract::ws::{WebSocketUpgrade, WebSocket},
    routing::get,
    response::{IntoResponse, Response},
    Router,
};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(|| async { "Secret string for Lynn" }));
    //let app = Router::new().route("/ws", get(handler));
    //let app = Router::new().route("/ws", get(send_json));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

//struct MyStruct {
    //first_field: u32,
    //second_field: String,
//}

//async fn send_json() -> Json<MyStruct> {
//
//}


async fn handler(ws: WebSocketUpgrade) -> Response {
    println!("handler");
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    println!("handle socket");
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

