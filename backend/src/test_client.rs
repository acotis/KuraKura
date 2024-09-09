
use std::sync::Arc;
use std::sync::LazyLock;

use http::Uri;
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use tokio_websockets::{ClientBuilder, Message, WebSocketStream, MaybeTlsStream};
use futures_util::{SinkExt, StreamExt, stream::{SplitSink, SplitStream}};

use crate::server::UserResponse;
use crate::server::UserOk::*;
use crate::server::UserErr::*;

type Receiver = SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>;
type Sender = SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>;

struct Client {
    ident: String,
    sender: Sender,
    response_history: Arc<Mutex<Vec<UserResponse>>>,
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
        let response_history = Arc::new(Mutex::new(vec![]));
        let just_sent = Arc::new(Mutex::new(false));

        tokio::spawn(follow(ident.to_owned(), delay, just_sent.clone(), receiver, response_history.clone()));

        println!("*** Client {} connected", ident);

        Client {
            ident: ident.to_owned(),
            sender,
            response_history,
            just_sent,
        }
    }

    async fn send(&mut self, text: &str) -> UserResponse {
        println!();
        println!("<—— Client {}: {}", self.ident, text);

        *self.just_sent.lock().await = true;

        self.sender
            .send(Message::text(text.to_owned()))
            .await
            .expect(&format!("{} couldn't send this text: {}", self.ident, text));

        pause(1000).await;

        let lock = self.response_history.lock().await;
        lock[lock.len() - 1].clone()
    }

    async fn register(&mut self) -> UserResponse {
        self.send(&format!(r#""Register""#)).await
    }

    async fn login(&mut self, account_id: &str) -> UserResponse {
        self.send(&format!(r#"{{"Login": {{"auth": "{account_id}"}}}}"#)).await
    }

    async fn create_room(&mut self) -> UserResponse {
        self.send(&format!(r#""Register""#)).await
    }

    async fn join_room(&mut self, room_id: &str) -> UserResponse {
        self.send(&format!(r#"{{"JoinRoom": {{"room": "{room_id}"}}}}"#)).await
    }

    async fn set_name(&mut self, name: &str) -> UserResponse {
        self.send(&format!(r#"{{"SetName": {{"name": "{name}"}}}}"#)).await
    }
}

async fn follow(ident: String, delay: usize, just_sent: Arc<Mutex<bool>>, mut receiver: Receiver, accum: Arc<Mutex<Vec<UserResponse>>>) {
    while let Some(Ok(message)) = receiver.next().await {
        let text = message.as_text().unwrap();

        accum.lock()
             .await
             .push(serde_json::from_str(&text)
                               .expect("server's response was not valid JSON"));

        let mut sent_lock = just_sent.lock().await;
        let del = if *sent_lock {
            *sent_lock = false; 0
        } else {
            delay
        };

        pause(del).await;

        println!("——> Client {ident}: {text}");
    }
}

async fn pause(millis: usize) {
    tokio::time::sleep(std::time::Duration::from_millis(millis as u64)).await;
}

pub async fn run_test_clients() {
    pause(1000).await;

    println!();
    let mut lynn = Client::new("Lynn").await;
    let mut evan = Client::new("Evan").await;
    let mut lexi = Client::new("Lexi").await;

    let Ok(AccountRegistered {id: lynn_acct}) = lynn.register().await else {panic!()};
    let Ok(AccountRegistered {id: evan_acct}) = evan.register().await else {panic!()};
    let Ok(AccountRegistered {id: lexi_acct}) = lexi.register().await else {panic!()};
}


/*

async fn call_and_response(sender: &mut Sender, receiver: &mut Receiver, id: usize, text: Option<&str>) -> Option<UserResponse> {
    if let Some(t) = text {
        send(id, sender, t).await;
        pause(100).await;
    } else {
        pause(200 + 100 * id).await;
    }

    let response = get_response(id, receiver).await;

    if let Some(_) = text {
        pause(900).await;
    } else {
        pause(800 - 100 * id).await;
    }

    response
}

// Client struct and fundamental methods.

struct Client {
    sender: Sender,
    receiver: Receiver,
    client_id: usize,

    account_id: Option<String>,
    room_id: Option<String>,
}

impl Client {
    async fn car(&mut self, text: &str) -> UserResponse {
        call_and_response(
            &mut self.sender,
            &mut self.receiver,
            self.client_id,
            Some(text)
        ).await.unwrap()
    }

    async fn poll(&mut self) {
        call_and_response(
            &mut self.sender,
            &mut self.receiver,
            self.client_id,
            None
        ).await;
    }

    fn new(sender: Sender, receiver: Receiver, client_id: usize) -> Self {
        Client {
            sender,
            receiver,
            client_id,
            account_id: None,
            room_id: None,
        }
    }
}

// API call methods.

impl Client {
    async fn register(&mut self) {
        if let Ok(AccountRegistered {id}) = self.car().await {
            self.account_id = Some(id);
        }
    }

    async fn login(&mut self) {
        let _ = self.car(&format!(r#"{{"Login": {{"auth": "{}"}}}}"#, self.account_id.as_ref().unwrap())).await;
    }

    async fn create_room(&mut self) {
        if let Ok(RoomCreated {id}) = self.car(r#""CreateRoom""#).await {
            self.room_id = Some(id);
        }
    }

    async fn join_room(&mut self) {
        let _ = self.car(&format!(r#"{{"JoinRoom": {{"room": "{}"}}}}"#, self.room_id.as_ref().unwrap())).await;
    }

    async fn set_name(&mut self, name: &str) {
        let _ = self.car(&format!(r#"{{"SetName": {{"name": "{name}"}}}}"#)).await;
    }
}

pub async fn run_test_client(id: usize) {

    // Wait for server to be set up, then get into numerical order.

    pause(1000 + 100 * id).await;

    // Connect to server.


    // Sync up again.

    pause(1000 - 100 * id).await;

    // Run scenario.

    let mut last: UserResponse = Err(NotImplemented);

    if id == 1 {client.register().await} else {client.poll().await;}
    if id == 2 {client.register().await} else {client.poll().await;}
    if id == 2 {client.login().await} else {client.poll().await;}



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
