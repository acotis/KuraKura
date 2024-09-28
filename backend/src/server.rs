
#![allow(unused)]

use crate::Game;
use crate::Turn;
use crate::TurnError;
use crate::server::UserRequest::*;
use crate::server::UserOk::*;
use crate::server::UserErr::*;
use crate::server::ServerError::*;
use crate::Player::Black;
use uuid::Uuid;
//use std::time::{Instant};
use std::collections::HashMap;
use std::fmt::{Display, Formatter, Error};
use serde::{Serialize, Deserialize};
use serde_json::from_str;
use axum::extract::ws::{WebSocket, Message::{self, Text}};
use futures_util::{SinkExt, stream::SplitSink};

// Public-facing types.

type AccountId = String;
type RoomId = String;
pub type SocketId = String;

// User inputs and outputs (i.e., things the user sends us and receives in response).

#[derive(Debug, Serialize, Deserialize)]
pub enum UserRequest {
    Register    {name: String},
    Login       {auth: AccountId},
    CreateRoom,
    JoinRoom    {room: RoomId},
    TakeTurn    {turn: Turn},

    DebugLog,   // Debugging only, turn this off in production.
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum UserOk {
    AccountRegistered   {id: AccountId},
    RoomCreated         {id: RoomId},
    JoinedAsPlayer,
    JoinedAsSpectator,
    Okay,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum UserErr {
    AccountNotFound,
    AlreadyLoggedIn,
    NotLoggedIn,
    RoomNotFound,
    AccountAlreadyHasRoom,     // Todo: add a paramater giving the room ID?
    RoomAlreadyHasGuest,    // (probably don't add such a parameter here for the player ID) (definitely not, that would reveal someone else's API key)
    NameTooLong,
    AccountDoesntHaveRoom,
    RoomDoesntHaveGuest,
    AccountPlayedWrongColor,
    InvalidTurn {error: TurnError},
    NotImplemented,
    InvalidJson,
}

pub type UserMessage = Result<UserOk, UserErr>;

// Server outputs (i.e., possible return values of the server's .handle_request() method).

#[derive(Debug)]
pub enum ServerError {SocketNotFound}
pub type ServerOk = Vec<(SocketId, UserMessage)>;
pub type ServerResult = Result<ServerOk, ServerError>;

// Basic entities recognized by the server.

struct Socket {
    id:         SocketId,
    account_id: Option<AccountId>,
}

struct Account {
    id:         AccountId,
    name:       String,
    room_id:    Option<RoomId>,
    socket_ids: Vec<SocketId>,
}

struct Room {
    id:                 RoomId,
    game:               Game,
    host_plays_black:   bool,
    player_ids:         Vec<AccountId>, // must be a vec for multiplayer games (N > 2)
    //creation_time:      Instant,
}

// Basic methods for those entities.

impl Socket {
    fn new() -> Self {
        Socket {
            id:         Uuid::new_v4().to_string(),
            account_id: None,
        }
    }
}

impl Account {
    fn new() -> Self {
        Account {
            id:         Uuid::new_v4().to_string(),
            name:       "".into(),
            room_id:    None,
            socket_ids: vec![],
        }
    }
}

impl Room {
    fn new(host_id: &str) -> Self {
        Room {
            id:                 Uuid::new_v4().to_string(),
            game:               Game::new(4, 2),
            host_plays_black:   true, // todo: make this random
            player_ids:         vec![],
            //creation_time:      Instant::now(),
        }
    }
}

// Server struct.

pub struct Server {
    sockets:    HashMap<SocketId, Socket>,
    accounts:   HashMap<AccountId, Account>,
    rooms:      HashMap<RoomId, Room>,
}

// Constructor.

impl Server {
    pub fn new() -> Self {
        Server {
            sockets: HashMap::new(),
            accounts: HashMap::new(),
            rooms: HashMap::new(),
        }
    }
}

// Public methods of Server.

impl Server {
    pub fn register_socket(&mut self) -> SocketId {
        let socket = Socket::new();
        let socket_id = socket.id.clone();
        self.sockets.insert(socket_id.clone(), socket);
        socket_id
    }

    pub fn handle_request(&mut self, socket_id: &SocketId, json: &str) -> ServerResult {
        if self.sockets.get(socket_id).is_none() {
            Err(SocketNotFound)
        } else {
            Ok(
                match from_str(&json) {
                    Ok(Register {name}) => {self.register   (socket_id.clone()      )},
                    Ok(Login    {auth}) => {self.login      (socket_id.clone(), auth)},
                    Ok(CreateRoom     ) => {self.create_room(socket_id.clone()      )},
                    Ok(JoinRoom {room}) => {self.join_room  (socket_id.clone(), room)},
                    Ok(TakeTurn {turn}) => {self.take_turn  (socket_id.clone(), turn)},
                    Ok(DebugLog       ) => {print!("{self}"); vec![(socket_id.clone(), Err(InvalidJson))]},
                    Err(_)              => {vec![(socket_id.clone(), Err(InvalidJson))]},
                }
            )
        }
    }
}

// Private methods directly corresponding to API calls.

impl Server {
    fn register(&mut self, socket_id: SocketId) -> ServerOk {
        let Some(socket) = self.sockets.get_mut(&socket_id) else {unreachable!()};

        if socket.account_id != None {
            vec![(socket_id, Err(AlreadyLoggedIn))]
        } else {
            let account = Account::new();
            let account_id = account.id.clone();
            self.accounts.insert(account_id.clone(), account);
            socket.account_id = Some(account_id.clone());

            vec![(socket_id, Ok(AccountRegistered {id: account_id}))]
        }
    }

    fn login(&mut self, socket_id: SocketId, account_id: AccountId) -> ServerOk {
        let Some(socket) = self.sockets.get_mut(&socket_id)   else {unreachable!()};
        let Some(_)      = self.accounts.get_mut(&account_id) else {return vec![(socket_id, Err(AccountNotFound))];};

        if socket.account_id != None {
            return vec![(socket_id, Err(AlreadyLoggedIn))];
        }

        socket.account_id = Some(account_id);
        vec![(socket_id, Ok(Okay))]
    }

    fn create_room(&mut self, socket_id: SocketId) -> ServerOk {
        let Some(socket)     = self.sockets.get_mut(&socket_id)   else {unreachable!()};
        let Some(account_id) = socket.account_id.clone()          else {return vec![(socket_id, Err(NotLoggedIn))];};
        let Some(account)    = self.accounts.get_mut(&account_id) else {return vec![(socket_id, Err(AccountNotFound))];};

        if account.room_id != None {
            return vec![(socket_id, Err(AccountAlreadyHasRoom))];
        }

        let room = Room::new(&account_id);
        let room_id = room.id.clone();
        self.rooms.insert(room_id.clone(), room);
        account.room_id = Some(room_id.clone());
        vec![(socket_id, Ok(RoomCreated {id: room_id}))]
    }

    fn join_room(&mut self, socket_id: SocketId, room_id: RoomId) -> ServerOk {
        let Some(socket)     = self.sockets.get_mut(&socket_id)   else {unreachable!()};
        let Some(account_id) = socket.account_id.clone()          else {return vec![(socket_id, Err(NotLoggedIn))];};
        let Some(account)    = self.accounts.get_mut(&account_id) else {return vec![(socket_id, Err(AccountNotFound))];};
        let Some(room)       = self.rooms.get_mut(&room_id)       else {return vec![(socket_id, Err(RoomNotFound))];};

        if account.room_id != None {
            return vec![(socket_id, Err(AccountAlreadyHasRoom))];
        }

        account.room_id = Some(room_id);
        room.player_ids.push(account_id);

        // todo: let the Game decide whether the new player is a player or a spectator.

        if room.player_ids.len() <= 2{
            vec![(socket_id, Ok(JoinedAsPlayer))]
        } else {
            vec![(socket_id, Ok(JoinedAsSpectator))]
        }
    }

    fn take_turn(&mut self, socket_id: SocketId, turn: Turn) -> ServerOk {
        let Some(socket)     = self.sockets.get_mut(&socket_id)   else {unreachable!()};
        let Some(account_id) = socket.account_id.clone()          else {return vec![(socket_id, Err(NotLoggedIn))];};
        let Some(account)    = self.accounts.get_mut(&account_id) else {return vec![(socket_id, Err(AccountNotFound))];};
        let Some(room_id)    = account.room_id.clone()            else {return vec![(socket_id, Err(AccountDoesntHaveRoom))];};
        let Some(room)       = self.rooms.get_mut(&room_id)       else {return vec![(socket_id, Err(RoomNotFound))];};

        // Todo: make sure that account really is that player!

        match room.game.turn(turn) {
            Ok(_)           => vec![(socket_id, Ok(Okay))],
            Err(turn_error) => vec![(socket_id, Err(InvalidTurn {error: turn_error}))],
        }
    }
}

// Display stuff.

impl Display for Account {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let bold = "\x1b[1m";
        let reset = "\x1b[0m";
        write!(f, "{bold}Account ID:{reset} {}... {bold}Name:{reset} {}", &self.id[0..4], self.name)
    }
}

impl Display for Room {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let bold = "\x1b[1m";
        let reset = "\x1b[0m";

        writeln!(f, "{bold}Room ID:{reset} {}...", &self.id[0..4])?;

        for player_id in &self.player_ids {
            writeln!(f, " ⮡ {bold}Player ID:{reset} {}...", &player_id[0..4])?;
        }

        for line in self.game.to_string().lines() {
            writeln!(f, "  {}", line)?;
        }

        Ok(())
    }
}

impl Display for Server {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let under = "\x1b[4m";
        let reset = "\x1b[0m";

        writeln!(f)?;
        writeln!(f, "{under}Accounts:{reset}")?;
        writeln!(f)?;

        for account in self.accounts.keys() {
            writeln!(f, "    {}", self.accounts.get(account).unwrap())?;
        }

        writeln!(f)?;
        writeln!(f, "{under}Rooms:{reset}")?;
        writeln!(f)?;

        for room in self.rooms.keys() {
            for line in self.rooms.get(room).unwrap().to_string().lines() {
                writeln!(f, "    {}", line)?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

