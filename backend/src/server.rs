
use crate::Game;
use crate::Turn;
use crate::TurnError;
use crate::server::KuraKuraRequest::*;
use crate::server::KuraKuraOk::*;
use crate::server::KuraKuraErr::*;
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

#[derive(Debug, Serialize, Deserialize)]
pub enum KuraKuraRequest {
    Register,
    Login       {auth: AccountId},
    
    CreateRoom,
    JoinRoom    {room: RoomId},
    SetName     {name: String},

    TakeTurn    {turn: Turn},
}

#[derive(Debug, Serialize, Deserialize)]
pub enum KuraKuraOk {
    AccountRegistered   {id: AccountId},
    RoomCreated         {id: RoomId},
    KuraKuraOk,
}

impl Termination for KuraKuraOk {
    fn report(self) -> ExitCode {ExitCode::from(0)}
}

#[derive(Debug, Serialize, Deserialize)]
pub enum KuraKuraErr {
    AccountNotFound,
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

pub type KuraKuraResponse = Result<KuraKuraOk, KuraKuraErr>;

// Implementation of server which publically deals in those types.

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

pub struct Server {
    accounts:  HashMap<AccountId, Account>,
    rooms:  HashMap<RoomId, Room>,
}

// Public method.

impl Server {
    pub fn handle_json(&mut self, json: &str) -> KuraKuraResponse {
        match from_str(&json) {
            Ok(Register       ) => {self.register()}
            Ok(Login    {auth}) => {Err(NotImplemented)} //self.login(auth)}
            Ok(SetName  {name}) => {self.set_name(name)}
            Ok(CreateRoom     ) => {self.create_room()}
            Ok(JoinRoom {room}) => {self.join_room(room)}
            Ok(TakeTurn {turn}) => {self.take_turn(turn)}
            Err(_)              => {Err(InvalidJson {})}
        }
    }
}

// Private methods directly corresponding to API calls.

impl Server {
    fn register(&mut self) -> KuraKuraResponse {
        let account_id = Uuid::new_v4().to_string();

        self.accounts.insert(account_id.clone(), Account {
            id:         account_id.clone(),
            name:       "".into(),
            room_id:    None,
        });

        Ok(AccountRegistered {id: account_id})
    }

    fn set_name(&mut self, auth: AccountId, name: String) -> KuraKuraResponse {
        if name.len() > 250 {return Err(NameTooLong);}

        let account: &mut Account = self.get_account(&auth)?;
        (*account).name = name;
        Ok(KuraKuraOk)
    }

    fn create_room(&mut self, auth: AccountId) -> KuraKuraResponse {
        let account: &mut Account = self.get_account(&auth)?;

        if account.room_id != None {
            return Err(AccountAlreadyHasRoom);
        }

        let room_id = Uuid::new_v4().to_string();
        account.room_id = Some(room_id.clone());

        self.rooms.insert(room_id.clone(), Room {
            id:                 room_id.clone(),
            host_account_id:    auth,
            guest_account_id:   None,
            game:               Game::new(4, 2),
            host_plays_black:   true, // todo: make this random
            //creation_time:      Instant::now(),
        });

        Ok(RoomCreated {id: room_id})
    }

    fn join_room(&mut self, auth: AccountId, room_id: RoomId) -> KuraKuraResponse {

        // Todo: these two stanzas of code are copied from the methods
        // .get_account() and .get_room(), which I can't just call directly
        // I think because the mutable reference to a Account or Room that
        // each one returns also keeps the mutable reference to self alive,
        // meaning that there would be two mutable references to self alive
        // at the same time, which is disallowed. So, I need to figure
        // out how to resolve this without violating DRY. Whatever solution
        // I come to, I should apply it to any other usage of this DRY
        // violation.

        let account = match self.accounts.get_mut(&auth) {
            Some(u) => Ok(u),
            None => Err(AccountNotFound),
        }?;

        let room = match self.rooms.get_mut(&room_id) {
            Some(u) => Ok(u),
            None => Err(RoomNotFound),
        }?;

        //let account: &mut Account = self.get_account(&auth)?;
        //let room: &mut Room = self.get_room(&room_id)?;

        if account.room_id != None {return Err(AccountAlreadyHasRoom);}
        if room.guest_account_id != None {return Err(RoomAlreadyHasGuest);}

        account.room_id = Some(room_id);
        room.guest_account_id = Some(auth);

        Ok(KuraKuraOk)
    }

    fn take_turn(&mut self, auth: AccountId, turn: Turn) -> KuraKuraResponse {
        let Some(account) = self.accounts.get_mut(&auth)    else {return Err(AccountNotFound);};
        let Some(room_id) = account.room_id.clone()         else {return Err(AccountDoesntHaveRoom);};
        let Some(room)    = self.rooms.get_mut(&room_id)    else {return Err(RoomNotFound);};
        let Some(_)       = room.guest_account_id.clone()   else {return Err(RoomDoesntHaveGuest);};
        let host          = room.host_account_id.clone();

        if (room.host_plays_black == (turn.player == Black)) != (auth == host) {
            return Err(AccountPlayedWrongColor);
        }

        // Todo: make sure that account really is that player!

        match room.game.turn(turn) {
            Ok(_) => Ok(KuraKuraOk),
            Err(turn_error) => Err(InvalidTurn {error: turn_error}),
        }
    }
}

// Constructor.

impl Server {
    pub fn new() -> Self {
        Server {
            accounts: HashMap::new(),
            rooms: HashMap::new(),
        }
    }
}

// Private utility methods.

impl Server {
    fn get_account(&mut self, account_id: &AccountId) -> Result<&mut Account, KuraKuraErr> {
        match self.accounts.get_mut(account_id) {
            Some(u) => Ok(u),
            None => Err(AccountNotFound),
        }
    }

    //fn get_room(&mut self, room_id: &RoomId) -> Result<&mut Room, KuraKuraErr> {
        //match self.rooms.get_mut(room_id) {
            //Some(u) => Ok(u),
            //None => Err(RoomNotFound),
        //}
    //}
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
               &self.host_account_id[0..4],
               match &self.guest_account_id {
                   None => "None",
                   Some(id) => &id[0..4],
               },
               match &self.guest_account_id {
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

