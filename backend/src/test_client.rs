
use futures_util::{SinkExt, StreamExt, stream::{SplitSink, SplitStream}};
use http::Uri;
use tokio_websockets::{ClientBuilder, Message, WebSocketStream, MaybeTlsStream};

use tokio::net::TcpStream;

static mut RESPONSES: Vec<String> = Vec::<String>::new();

fn get_symbol(id: usize) -> char {
    match id {
        1 => 'L',
        2 => 'E',
        _ => panic!(),
    }
}

async fn pause(millis: usize) {
    tokio::time::sleep(std::time::Duration::from_millis(millis as u64)).await;
}

async fn send(client_id: usize, client: &mut SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>, text: &'static str) {
    println!();
    println!(
        "<—— Client {}: {}",
        get_symbol(client_id),
        text
    );

    client.send(Message::text(text))
          .await
          .expect(&format!("couldn't send this text: {text}"));
}

async fn follow(client_id: usize, mut client: SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>) {
    loop {
        let message = client.next().await.unwrap().unwrap();
        let text = message.as_text().unwrap();

        println!(
            "——> Client {}: {}",
            get_symbol(client_id),
            text,
        );

        unsafe {
            while RESPONSES.len() < client_id {
                RESPONSES.push("".into());
            }

            RESPONSES[client_id-1] = text.to_owned();
        }
    }
}

fn get_response(client_id: usize) -> String {
    unsafe {RESPONSES[client_id-1].clone()}
}

pub async fn run_test_client(id: usize) {
    pause(100 * id).await;

    let uri = Uri::from_static("ws://127.0.0.1:3000");
    let (client, _) = ClientBuilder::from_uri(uri).connect().await.unwrap();
    let (mut sender, receiver) = client.split();

    println!("*** Client {} connected", get_symbol(id));

    tokio::spawn(follow(id, receiver));

    pause(1000).await;

    match id {
        1 => send(id, &mut sender, "Hello world").await,
        2 => send(id, &mut sender, "hi there").await,
        _ => panic!(),
    }
}

