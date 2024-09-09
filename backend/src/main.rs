
use std::sync::Arc;

use futures_util::{StreamExt};
use tokio::sync::Mutex;
use axum::{
    extract::{State, ws::{WebSocketUpgrade, WebSocket, Message::Text}},
    routing::get,
    response::Response,
    Router
};

use kurakura::server::Server;
use kurakura::test_client::run_test_clients;

#[tokio::main]
async fn main() {
    tokio::spawn(run_test_clients());

    let server = Arc::new(Mutex::new(Server::new()));
    let app = Router::new().route("/", get(handler)).with_state(server);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

#[axum::debug_handler]
async fn handler(State(state): State<Arc::<Mutex::<Server>>>, ws: WebSocketUpgrade) -> Response {
    ws.on_upgrade(|socket| async {handle_socket(state, socket).await})
}

async fn handle_socket(state: Arc::<Mutex::<Server>>, socket: WebSocket) {
    let (sender, mut receiver) = socket.split();

    let handle = {
        state.lock()
             .await
             .register_socket(sender)
    };

    while let Some(Ok(Text(msg))) = receiver.next().await {
        state.lock()
             .await
             .handle_request(&handle, &msg)
             .await
             .expect("server gave error");
    }
}

