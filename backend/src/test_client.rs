
use futures_util::{SinkExt, StreamExt, stream::{SplitSink, SplitStream}};
use http::Uri;
use tokio_websockets::{ClientBuilder, Message, WebSocketStream, MaybeTlsStream};

use tokio::net::TcpStream;

async fn pause(millis: u64) {
    tokio::time::sleep(std::time::Duration::from_millis(millis)).await;
}

async fn send(client_id: u64, client: &mut SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>, text: &'static str) {
    println!(
        "<== Client {}: {}",
        client_id,
        text
    );

    client.send(Message::text(text))
          .await
          .expect(&format!("couldn't send this text: {text}"));
}

async fn follow(client_id: u64, mut client: SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>) {
    loop {
        println!(
            "==> Client {}: {}",
            client_id,
            client.next().await.unwrap().unwrap().as_text().unwrap()
        );
    }
}

pub async fn run_test_client(client_id: u64) {
    pause(1000).await;

    let uri = Uri::from_static("ws://127.0.0.1:3000");
    let (client, _) = ClientBuilder::from_uri(uri).connect().await.unwrap();
    let (mut sender, receiver) = client.split();

    tokio::spawn(follow(client_id, receiver));

    send(client_id, &mut sender, "Hello world").await;

    pause(1000).await;

}

