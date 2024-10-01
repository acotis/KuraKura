
use std::sync::Arc;
use std::sync::LazyLock;

use std::fmt::Debug;

use http::Uri;
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use tokio_websockets::{ClientBuilder, Message, WebSocketStream, MaybeTlsStream};
use futures_util::{SinkExt, StreamExt, stream::{SplitSink, SplitStream}};
use serde::de::DeserializeOwned;

use crate::server::message_types::UserMessage::{self, *};
use crate::server::message_types::UserOk::*;
use crate::server::message_types::UserError::*;
use crate::server::game::Game;
use crate::game::KuraKura;

type Receiver = SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>;
type Sender = SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>;

struct Client<G: Game> {
    ident: String,
    sender: Sender,
    last_response: Arc<Mutex<Option<UserMessage<G::TurnError>>>>,
    just_sent: Arc<Mutex<bool>>,
}

impl<G: Game> Client<G> {
    async fn new(ident: &str) -> Self where <G as Game>::TurnError: 'static {
        static NEXT_DELAY: LazyLock<Mutex<usize>> = LazyLock::new(|| Mutex::new(0));

        let delay = {
            let mut lock = NEXT_DELAY.lock().await;
            *lock += 100;
            *lock
        };

        let uri = Uri::from_static("ws://127.0.0.1:3000");
        let (websocket, _) = ClientBuilder::from_uri(uri).connect().await.unwrap();
        let (sender, receiver)  = websocket.split();
        let last_response = Arc::new(Mutex::new(None));
        let just_sent = Arc::new(Mutex::new(false));

        tokio::spawn(follow::<G::TurnError>(ident.to_owned(), delay, just_sent.clone(), receiver, last_response.clone()));

        println!();
        println!("*** {} connected", ident);

        Client {
            ident: ident.to_owned(),
            sender,
            last_response,
            just_sent,
        }
    }

    async fn send(&mut self, text: &str) -> UserMessage<G::TurnError> {
        println!();
        println!("<—— {}: {}", self.ident, text);

        *self.just_sent.lock().await = true;

        self.sender
            .send(Message::text(text.to_owned()))
            .await
            .expect(&format!("{} couldn't send this text: {}", self.ident, text));

        pause(1000).await;

        self.last_response
            .lock()
            .await
            .take()
            .expect("Request was not responded to at all.")
    }

    // Unchecked server interactions.

    async fn create_room_unchecked(&mut self, name: &str) -> UserMessage<G::TurnError> {
        self.send(&format!(r#"{{"CreateRoom": {{"name": "{name}"}}}}"#)).await
    }

    async fn join_room_unchecked(&mut self, name: &str, room_id: &str) -> UserMessage<G::TurnError> {
        self.send(&format!(r#"{{"JoinRoom": {{"name": "{name}", "room": "{room_id}"}}}}"#)).await
    }

    async fn debug_log_unchecked(&mut self) -> UserMessage<G::TurnError> {
        self.send(&format!(r#""DebugLog""#)).await
    }

    async fn take_turn_unchecked(&mut self, turn: G::Turn) -> UserMessage<G::TurnError> {
        self.send(&format!(r#"{{"TakeTurn": {}}}"#, serde_json::to_string(&turn).unwrap())).await
    }

    // Checked server interactions.

    async fn create_room(&mut self, name: &str) -> String {
        let response = self.create_room_unchecked(name).await;

        if let ResponseMessage(Ok(RoomCreated {id})) = response {
            id.to_string()
        } else {
            panic!("When creating room, response was: {response:?}")
        }
    }

    async fn join_room(&mut self, name: &str, room_id: &str) {
        let response = self.join_room_unchecked(name, room_id).await;

        if response != ResponseMessage(Ok(JoinedAsPlayer)) &&
           response != ResponseMessage(Ok(JoinedAsSpectator)) {
            panic!("When joining room, response was: {response:?}");
        }
    }

    async fn debug_log(&mut self) {
        let response = self.debug_log_unchecked().await;

        if response != ResponseMessage(Err(InvalidJson)) {
            panic!("When requesting debug log, response was: {response:?}");
        }
    }
}

async fn follow<E: DeserializeOwned>(ident: String, delay: usize, just_sent: Arc<Mutex<bool>>, mut receiver: Receiver, last: Arc<Mutex<Option<UserMessage<E>>>>) {
    while let Some(Ok(message)) = receiver.next().await {
        let text = message.as_text().unwrap();

        last.lock()
            .await
            .replace(serde_json::from_str(&text)
                                .expect(&format!("server's message was not valid JSON: {}", text)));

        let mut sent_lock = just_sent.lock().await;
        let del = if *sent_lock {
            *sent_lock = false; 0
        } else {
            delay
        };

        pause(del).await;

        println!("——> {ident}: {text}");
    }
}

async fn pause(millis: usize) {
    tokio::time::sleep(std::time::Duration::from_millis(millis as u64)).await;
}

pub async fn run_test_clients() {
    pause(1000).await;

    let mut lynn  = Client::<KuraKura>::new("Lynn").await;
    let mut evan  = Client::<KuraKura>::new("Evan").await;
    let mut lexi  = Client::<KuraKura>::new("Lexi").await;

    let evan_rm   = evan.create_room("Evan is my name").await;
    let _lynn_rm  = lynn.create_room("Lynnnn").await;
    let _         = lexi.join_room("The LEX", &evan_rm).await;

    let _         = evan.debug_log().await;

    println!();
}


/*


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
*/
