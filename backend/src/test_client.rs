
use std::task::Poll::*;

use futures_util::{SinkExt, StreamExt, stream::{SplitSink, SplitStream}, poll};
use http::Uri;
use tokio_websockets::{ClientBuilder, Message, WebSocketStream, MaybeTlsStream};
use tokio::net::TcpStream;

use crate::server::UserResponse;

type Receiver = SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>;
type Sender = SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>;

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

async fn send(client_id: usize, sender: &mut Sender, text: &str) {
    println!();
    println!(
        "<—— Client {}: {}",
        get_symbol(client_id),
        text
    );

    sender.send(Message::text(text.to_owned()))
          .await
          .expect(&format!("couldn't send this text: {text}"));
}

//fn get_response(client_id: usize, receiver: &mut Receiver) -> Option<UserResponse> {
async fn get_response(client_id: usize, receiver: &mut Receiver) -> Option<String> {
    match poll!(receiver.next()) {
        Pending => None,
        Ready(msg) => {
            let message = msg.unwrap().unwrap();
            let text = message.as_text().unwrap();
            println!("——> Client {}: {}", get_symbol(client_id), text);
            //serde_json::from_str(&text).expect("server's response was not valid JSON") // parse
            Some(text.to_owned())
        }
    }
}

async fn call_and_response(target_id: usize, actual_id: usize, sender: &mut Sender, receiver: &mut Receiver, text: &str) -> Option<String> {
    if actual_id == target_id {
        send(actual_id, sender, text).await;
    }

    if actual_id == target_id {
        pause(100).await;
    } else {
        pause(200 + 100 * actual_id).await;
    }

    let response = get_response(actual_id, receiver).await;

    if actual_id == target_id {
        pause(900).await;
    } else {
        pause(800 - 100 * actual_id).await;
    }

    response
}

pub async fn run_test_client(id: usize) {

    // Wait for server to be set up.

    pause(1000).await;

    // Get into numerical order, connect to the server, and sync up again.

    pause(100 * id).await;

    let uri = Uri::from_static("ws://127.0.0.1:3000");
    let (client, _) = ClientBuilder::from_uri(uri).connect().await.unwrap();
    let (mut sender, mut receiver) = client.split();

    println!("*** Client {} connected", get_symbol(id));

    pause(1000 - 100 * id).await;

    // Client 1 sends first message.

    let cu = &format!(r#""Register""#);

    let _resp = call_and_response(1, id, &mut sender, &mut receiver, cu).await;



    /*
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

