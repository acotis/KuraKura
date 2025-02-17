
use std::sync::Arc;
use std::sync::LazyLock;

use std::fmt::Debug;

use http::Uri;
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use tokio_websockets::{ClientBuilder, Message, WebSocketStream, MaybeTlsStream};
use futures_util::{SinkExt, StreamExt, stream::{SplitSink, SplitStream}};
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::server::message_types::*;
use crate::server::message_types::UserMessage::{self, *};
use crate::server::message_types::UserOk::*;
use crate::server::message_types::UserError::*;
use crate::server::game::Game as GameTrait;
use crate::game::KuraKura;
use crate::game::types::*;
use crate::game::types::SpinDirection::*;

type Receiver = SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>;
type Sender = SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>;

async fn follow<Game>(
    ident: String,
    delay: usize,
    just_sent: Arc<Mutex<bool>>,
    mut receiver: Receiver,
    last: Arc<Mutex<Option<UserMessage<Game>>>>
) 
where Game: GameTrait + DeserializeOwned + Send,
      Game::TurnError: DeserializeOwned + Send,
      Game::Turn: Send,
      Game::PlayerRole: Send + DeserializeOwned,
      Game::Outcome: DeserializeOwned + Send + Debug,
{
    while let Some(Ok(message)) = receiver.next().await {
        let text = message.as_text().unwrap();
        let response =
            serde_json::from_str(&text)
                .expect(&format!("server's message was not valid JSON: {}", text));

        if matches!(response, ResponseMessage(_)) {
            last.lock()
                .await
                .replace(response);
        }

        let mut sent_lock = just_sent.lock().await;
        let del = if *sent_lock {
            *sent_lock = false; 0
        } else {
            delay
        };

        pause(del).await;

        let max_len = 9999;
        let shortened_text = if text.len() < max_len {
            String::from(text)
        } else {
            String::from(&text[..max_len-3]) + "..."
        };

        println!("——> {ident}: {shortened_text}");
    }
}

async fn pause(millis: usize) {
    tokio::time::sleep(std::time::Duration::from_millis(millis as u64)).await;
}

struct Client<Game: GameTrait> where Game::Turn: Serialize, Game::TurnError: DeserializeOwned + Debug {
    ident: String,
    sender: Sender,
    last_response: Arc<Mutex<Option<UserMessage<Game>>>>,
    just_sent: Arc<Mutex<bool>>,
}

impl<Game: GameTrait + Send> Client<Game>
where Game: DeserializeOwned,
      Game::Turn: Serialize + Send,
      Game::TurnError: DeserializeOwned + Debug + Send,
      Game::PlayerRole: Send + Debug + DeserializeOwned,
      Game: Debug,
      Game::Outcome: DeserializeOwned + Send + Debug,
{
    async fn new(ident: &str) -> Self where <Game as GameTrait>::TurnError: 'static, Game: 'static {
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

        tokio::spawn(follow::<Game>(ident.to_owned(), delay, just_sent.clone(), receiver, last_response.clone()));

        println!();
        println!("*** {} connected", ident);

        Client {
            ident: ident.to_owned(),
            sender,
            last_response,
            just_sent,
        }
    }

    async fn send(&mut self, text: &str) -> UserMessage<Game> {
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

    async fn create_room_unchecked(&mut self, name: &str, host_plays_black: bool, grid_size: usize, win_length: usize) -> UserMessage<Game> {
        self.send(&format!(r#"{{"CreateRoom": {{"name": "{name}", "parameters": {{"host_plays_black": {host_plays_black}, "grid_size": {grid_size}, "win_length": {win_length}}}}}}}"#)).await
    }

    async fn join_room_unchecked(&mut self, name: &str, room_id: &str) -> UserMessage<Game> {
        self.send(&format!(r#"{{"JoinRoom": {{"name": "{name}", "room": "{room_id}"}}}}"#)).await
    }

    async fn debug_log_unchecked(&mut self) -> UserMessage<Game> {
        self.send(&format!(r#""DebugLog""#)).await
    }

    async fn take_turn_unchecked(&mut self, turn: Game::Turn) -> UserMessage<Game> {
        self.send(&format!(r#"{{"TakeTurn": {{"turn": {}}}}}"#, serde_json::to_string(&turn).unwrap())).await
    }

    // Checked server interactions.

    async fn create_room(&mut self, name: &str, host_plays_black: bool, grid_size: usize, win_length: usize) -> String where Game::Turn : Debug {
        let response = self.create_room_unchecked(name, host_plays_black, grid_size, win_length).await;

        if let ResponseMessage(Ok(RoomCreated {room_state: RoomState {room_id, ..}, ..})) = response {
            room_id.to_string()
        } else {
            panic!("When creating room, response was: {response:?}")
        }
    }

    async fn join_room(&mut self, name: &str, room_id: &str) where Game::Turn : Debug {
        let response = self.join_room_unchecked(name, room_id).await;

        if !matches!(response, ResponseMessage(Ok(RoomJoined {..}))) {
            panic!("When joining room, response was: {response:?}");
        }
    }

    async fn debug_log(&mut self) where Game::Turn : Debug {
        let response = self.debug_log_unchecked().await;

        if !matches!(response, ResponseMessage(Err(InvalidJson(_)))) {
            panic!("When requesting debug log, response was: {response:?}");
        }
    }

    async fn take_turn(&mut self, turn: Game::Turn) where Game::Turn : Debug {
        let response = self.take_turn_unchecked(turn).await;

        if !matches!(response, ResponseMessage(Ok(TurnAccepted {..}))) {
            panic!("When taking turn, response was: {response:?}");
        }
    }
}

pub async fn run_test_clients() {
    pause(1000).await;

    let mut lynn  = Client::<KuraKura>::new("Lynn").await;
    let mut evan  = Client::<KuraKura>::new("Evan").await;
    let mut lexi  = Client::<KuraKura>::new("Lexi").await;

    let evan_rm   = evan.create_room("Evan is my name", true, 2, 2).await;
    let _lynn_rm  = lynn.create_room("Lynnnn", true, 2, 2).await;
    let _         = lexi.join_room("The LEX", &evan_rm).await;

    let _         = evan.debug_log().await;

    let _         = evan.take_turn(Turn {
        play_row:       0,
        play_col:       0,
        spin_ul_row:    0,
        spin_ul_col:    0,
        spin_size:      1,
        spin_dir:       CW,
    }).await;

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
