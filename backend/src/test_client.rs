
use futures_util::{SinkExt, StreamExt, stream::{SplitSink, SplitStream}};
use http::Uri;
use tokio_websockets::{ClientBuilder, Message, WebSocketStream, MaybeTlsStream};
use tokio::net::TcpStream;

use crate::server::UserResponse;

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

fn get_response(client_id: usize) -> UserResponse {
    let string = unsafe {RESPONSES[client_id-1].clone()};

    serde_json::from_str(&string)
                .expect("server's response was not JSON representing a valid value")
}

pub async fn run_test_client(id: usize) {
    pause(100 * id).await;

    let uri = Uri::from_static("ws://127.0.0.1:3000");
    let (client, _) = ClientBuilder::from_uri(uri).connect().await.unwrap();
    let (mut sender, receiver) = client.split();

    println!("*** Client {} connected", get_symbol(id));

    tokio::spawn(follow(id, receiver));

    pause(1000).await;

    //let cu  = format!(r#""Register""#);

    let cu = "hi";

    match id {
        1 => send(id, &mut sender, cu).await,
        2 => send(id, &mut sender, cu).await,
        2 => send(id, &mut sender, cu).await,
        _ => panic!(),
    }

    //let response = get_response(id);



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

