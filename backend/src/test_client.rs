
use std::sync::Arc;
use std::sync::LazyLock;

use http::Uri;
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use tokio_websockets::{ClientBuilder, Message, WebSocketStream, MaybeTlsStream};
use futures_util::{SinkExt, StreamExt, stream::{SplitSink, SplitStream}};

use crate::server::UserMessage;
use crate::server::UserOk::*;
use crate::server::UserErr::*;

type Receiver = SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>;
type Sender = SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>;

struct Client {
    ident: String,
    sender: Sender,
    last_response: Arc<Mutex<Option<UserMessage>>>,
    just_sent: Arc<Mutex<bool>>,
}

impl Client {
    async fn new(ident: &str) -> Self {
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

        tokio::spawn(follow(ident.to_owned(), delay, just_sent.clone(), receiver, last_response.clone()));

        println!();
        println!("*** {} connected", ident);

        Client {
            ident: ident.to_owned(),
            sender,
            last_response,
            just_sent,
        }
    }

    async fn send(&mut self, text: &str) -> UserMessage {
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

    async fn register_unchecked(&mut self, name: &str) -> UserMessage {
        self.send(&format!(r#"{{"Register": {{"name": "{name}"}}}}"#)).await
    }

    async fn login_unchecked(&mut self, account_id: &str) -> UserMessage {
        self.send(&format!(r#"{{"Login": {{"auth": "{account_id}"}}}}"#)).await
    }

    async fn create_room_unchecked(&mut self) -> UserMessage {
        self.send(&format!(r#""CreateRoom""#)).await
    }

    async fn join_room_unchecked(&mut self, room_id: &str) -> UserMessage {
        self.send(&format!(r#"{{"JoinRoom": {{"room": "{room_id}"}}}}"#)).await
    }

    async fn debug_log_unchecked(&mut self) -> UserMessage {
        self.send(&format!(r#""DebugLog""#)).await
    }

    // Checked server interactions.

    async fn register(&mut self, name: &str) -> String {
        let response = self.register_unchecked(name).await;

        if let Ok(AccountRegistered {id}) = response {
            id
        } else {
            panic!("When registering account, response was: {response:?}")
        }
    }

    async fn login(&mut self, account_id: &str) {
        let response = self.login_unchecked(account_id).await;

        if response != Ok(Okay) {
            panic!("When logging in, response was: {response:?}");
        }
    }

    async fn create_room(&mut self) -> String {
        let response = self.create_room_unchecked().await;

        if let Ok(RoomCreated {id}) = response {
            id
        } else {
            panic!("When creating room, response was: {response:?}")
        }
    }

    async fn join_room(&mut self, room_id: &str) {
        let response = self.join_room_unchecked(room_id).await;

        if response != Ok(JoinedAsPlayer) && response != Ok(JoinedAsSpectator) {
            panic!("When joining room, response was: {response:?}");
        }
    }

    async fn debug_log(&mut self) {
        let response = self.debug_log_unchecked().await;

        if response != Err(InvalidJson) {
            panic!("When requesting debug log, response was: {response:?}");
        }
    }
}

async fn follow(ident: String, delay: usize, just_sent: Arc<Mutex<bool>>, mut receiver: Receiver, last: Arc<Mutex<Option<UserMessage>>>) {
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

    let mut lynn  = Client::new("Lynn").await;
    let mut evan  = Client::new("Evan").await;
    let mut lexi  = Client::new("Lexi").await;

    let evan_acct = evan.register("Evan is my name").await;
    let _         = lynn.register("Lynnnn").await;
    let _         = lexi.register("The LEX").await;
    let _         = evan.register_unchecked("second name").await;

    let mut evn2  = Client::new("Evan (second tab)").await;

    let _         = evn2.login(&evan_acct).await;
    let evan_room = evn2.create_room().await;
    let _         = lynn.join_room(&evan_room).await;
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
