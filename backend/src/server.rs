
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
use std::process::ExitCode;
use std::process::Termination;
use serde::{Serialize, Deserialize};
use serde_json::from_str;

// Public-facing types.

type AccountId = String;
type RoomId = String;
type SocketId = String;

// User inputs and outputs (i.e., things the user sends us and receives in response).

#[derive(Debug, Serialize, Deserialize)]
pub enum UserRequest {
    Register,
    Login       {auth: AccountId},
    CreateRoom,
    JoinRoom    {room: RoomId},
    SetName     {name: String},
    TakeTurn    {turn: Turn},
}

#[derive(Debug, Serialize, Deserialize)]
pub enum UserOk {
    AccountRegistered   {id: AccountId},
    RoomCreated         {id: RoomId},
    Okay,
}

impl Termination for UserOk {
    fn report(self) -> ExitCode {ExitCode::from(0)}
}

#[derive(Debug, Serialize, Deserialize)]
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

pub type UserResponse = Result<UserOk, UserErr>;

// Server outputs (i.e., possible return values of the server's .handle_json() method).

pub enum ServerError {SocketNotFound}
pub type ServerResult = Result<String, ServerError>;

// Basic entities recognized by the server.

struct Socket {
    id:         SocketId,
    account_id: Option<AccountId>,
}

struct Account {
    id:         AccountId,
    name:       String,
    room_id:    Option<RoomId>,
}

struct Room {
    id:                 RoomId,
    host_id:            AccountId,
    guest_id:           Option<AccountId>,
    game:               Game,
    host_plays_black:   bool,
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
            id:      Uuid::new_v4().to_string(),
            name:    "".into(),
            room_id: None,
        }
    }
}

impl Room {
    fn new(host_id: &str) -> Self {
        Room {
            id:               Uuid::new_v4().to_string(),
            host_id:          host_id.to_owned(),
            guest_id:         None,
            game:             Game::new(4, 2),
            host_plays_black: true, // todo: make this random
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

// Public methods of Server.

impl Server {
    pub fn new_socket(&mut self) -> SocketId {
        let socket = Socket::new();
        self.sockets.insert(socket.id, socket);
        socket.id
    }

    pub fn handle_json(&mut self, socket_id: SocketId, json: &str) -> ServerResult {
        let Some(_) = self.sockets.get(&socket_id) else {return Err(SocketNotFound);};

        Ok(
            serde_json::to_string(
                &match from_str(&json) {
                    Ok(Register       ) => {self.register   (socket_id      )},
                    Ok(Login    {auth}) => {self.login      (socket_id, auth)},
                    Ok(SetName  {name}) => {self.set_name   (socket_id, name)},
                    Ok(CreateRoom     ) => {self.create_room(socket_id      )},
                    Ok(JoinRoom {room}) => {self.join_room  (socket_id, room)},
                    Ok(TakeTurn {turn}) => {self.take_turn  (socket_id, turn)},
                    Err(_)              => {Err(InvalidJson)},
                }
            ).expect("serde JSON error")
        )
    }
}

// Private methods directly corresponding to API calls.

impl Server {
    fn register(&mut self, socket_id: SocketId) -> UserResponse {
        let Some(socket) = self.sockets.get_mut(&socket_id) else {unreachable!()};

        if socket.account_id != None {
            return Err(AlreadyLoggedIn);
        }
        
        let account = Account::new();
        self.accounts.insert(account.id, account);
        Ok(AccountRegistered {id: account.id})
    }

    fn login(&mut self, socket_id: SocketId, account_id: AccountId) -> UserResponse {
        let Some(socket)  = self.sockets.get_mut(&socket_id)   else {unreachable!()};
        let Some(account) = self.accounts.get_mut(&account_id) else {return Err(AccountNotFound);};

        if socket.account_id != None {
            return Err(AlreadyLoggedIn);
        }

        socket.account_id = Some(account_id);
        Ok(Okay)
    }

    fn set_name(&mut self, socket_id: SocketId, name: String) -> UserResponse {
        let Some(socket)     = self.sockets.get_mut(&socket_id)   else {unreachable!()};
        let Some(account_id) = socket.account_id                  else {return Err(NotLoggedIn);};
        let Some(account)    = self.accounts.get_mut(&account_id) else {return Err(AccountNotFound);};

        if name.len() > 250 {
            return Err(NameTooLong);
        }

        account.name = name;
        Ok(Okay)
    }

    fn create_room(&mut self, socket_id: SocketId) -> UserResponse {
        let Some(socket)     = self.sockets.get_mut(&socket_id)   else {unreachable!()};
        let Some(account_id) = socket.account_id                  else {return Err(NotLoggedIn);};
        let Some(account)    = self.accounts.get_mut(&account_id) else {return Err(AccountNotFound);};

        if account.room_id != None {
            return Err(AccountAlreadyHasRoom);
        }

        let room = Room::new(&account_id);
        self.rooms.insert(room.id, room);
        account.room_id = Some(room.id);
        Ok(RoomCreated {id: room.id})
    }

    fn join_room(&mut self, socket_id: SocketId, room_id: RoomId) -> UserResponse {
        let Some(socket)     = self.sockets.get_mut(&socket_id)   else {unreachable!()};
        let Some(account_id) = socket.account_id                  else {return Err(NotLoggedIn);};
        let Some(account)    = self.accounts.get_mut(&account_id) else {return Err(AccountNotFound);};
        let Some(room)       = self.rooms.get_mut(&room_id)       else {return Err(RoomNotFound);};

        if account.room_id != None {
            return Err(AccountAlreadyHasRoom);
        }

        if room.guest_id != None {
            return Err(RoomAlreadyHasGuest);
        }

        account.room_id = Some(room_id);
        room.guest_id = Some(account_id);
        Ok(Okay)
    }

    fn take_turn(&mut self, socket_id: SocketId, turn: Turn) -> UserResponse {
        let Some(socket)     = self.sockets.get_mut(&socket_id)   else {unreachable!()};
        let Some(account_id) = socket.account_id                  else {return Err(NotLoggedIn);};
        let Some(account)    = self.accounts.get_mut(&account_id) else {return Err(AccountNotFound);};
        let Some(room_id)    = account.room_id.clone()            else {return Err(AccountDoesntHaveRoom);};
        let Some(room)       = self.rooms.get_mut(&room_id)       else {return Err(RoomNotFound);};
        let Some(_)          = room.guest_id.clone()              else {return Err(RoomDoesntHaveGuest);};
        let host             = room.host_id.clone();

        if (room.host_plays_black == (turn.player == Black)) != (account_id == host) {
            return Err(AccountPlayedWrongColor);
        }

        // Todo: make sure that account really is that player!

        match room.game.turn(turn) {
            Ok(_) => Ok(Okay),
            Err(turn_error) => Err(InvalidTurn {error: turn_error}),
        }
    }
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

        writeln!(f, "{bold}Room ID:{reset} {}... {bold}Host ID:{reset} {}... {bold}Guest ID:{reset} {}{}",
               &self.id[0..4],
               &self.host_id[0..4],
               match &self.guest_id {
                   None => "None",
                   Some(id) => &id[0..4],
               },
               match &self.guest_id {
                   None => "",
                   Some(_) => "..."
               })?;
        
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

