
use std::sync::Arc;
use std::collections::HashMap;

use futures_util::{StreamExt, SinkExt, stream::{SplitSink}};
use tokio::sync::Mutex;
use axum::{
    extract::{State, ws::{WebSocketUpgrade, WebSocket, Message::{self, Text}}},
    routing::get,
    response::Response,
    Router
};

use kurakura::server::Server;
use kurakura::server::SocketId;
use kurakura::test_client::run_test_clients;

type Sender   = SplitSink<WebSocket, Message>;
type Senders  = HashMap<SocketId, Sender>;
type AppState = (Arc<Mutex<Server>>, Arc<Mutex<Senders>>);

#[tokio::main]
async fn main() {
    tokio::spawn(run_test_clients());

    let server  = Arc::new(Mutex::new(Server::new()));
    let senders = Arc::new(Mutex::new(Senders::new()));

    let app = Router::new().route("/", get(handler)).with_state((server, senders));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

#[axum::debug_handler]
async fn handler(State(state): State<AppState>, ws: WebSocketUpgrade) -> Response {
    ws.on_upgrade(|socket| async {handle_socket(state, socket).await})
}

async fn handle_socket((server, senders): AppState, socket: WebSocket) {
    let (sender, mut receiver) = socket.split();

    let handle = server.lock().await.register_socket();
    senders.lock().await.insert(handle.clone(), sender);

    while let Some(Ok(Text(msg))) = receiver.next().await {
        let messages = 
            server.lock()
                  .await
                  .handle_request(&handle, &msg)
                  .expect(&format!("server gave error, socket ID was {handle}"));

        let mut senders_lock = senders.lock().await;

        for (socket_id, message) in messages {
            senders_lock
                .get_mut(&socket_id)
                .unwrap()
                .send(Text(serde_json::to_string(&message).unwrap()))
                .await
                .unwrap();
        }
    }
}

