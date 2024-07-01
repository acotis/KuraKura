
use crate::Game;
use crate::Turn;
use crate::TurnError;
use crate::server::KuraKuraRequest::*;
use crate::server::KuraKuraOk::*;
use crate::server::KuraKuraErr::*;
use uuid::Uuid;
use std::time::{Instant, Duration};
use std::collections::HashMap;
use std::fmt::{Display, Formatter, Error};
use std::process::ExitCode;
use std::process::Termination;
use serde::{Serialize, Deserialize};
use serde_json::from_str;

// Public-facing types.

type UserId = String;
type RoomId = String;

#[derive(Debug, Serialize, Deserialize)]
pub enum KuraKuraRequest {
    CreateUser  {},
    SetName     {auth: UserId, name: String},
    CreateRoom  {auth: UserId},
    JoinRoom    {auth: UserId, room: RoomId},
    TakeTurn    {auth: UserId, turn: Turn},
}

#[derive(Debug, Serialize, Deserialize)]
pub enum KuraKuraOk {
    UserCreated {id: UserId},
    NameSet     {},
    RoomCreated {id: RoomId},
    RoomJoined  {},
    TurnTaken   {},
}

impl Termination for KuraKuraOk {
    fn report(self) -> ExitCode {ExitCode::from(0)}
}

#[derive(Debug, Serialize, Deserialize)]
pub enum KuraKuraErr {
    UserNotFound,
    RoomNotFound,
    UserAlreadyHasRoom,     // Todo: add a paramater giving the room ID?
    RoomAlreadyHasGuest,    // (probably don't add such a parameter here for the player ID) (definitely not, that would reveal someone else's API key)
    NameTooLong,
    UserDoesntHaveRoom,
    RoomDoesntHaveGuest,
    InvalidTurn {error: TurnError},
    NotImplemented,
    InvalidJson,
}

pub type KuraKuraResponse = Result<KuraKuraOk, KuraKuraErr>;

// Implementation of server which publically deals in those types.

struct User {
    id:         UserId,
    name:       String,
    room_id:    Option<RoomId>,
}

struct Room {
    id:                 RoomId,
    host_user_id:       UserId,
    guest_user_id:      Option<UserId>,
    game:               Game,
    host_plays_black:   bool,
    creation_time:      Instant,
}

pub struct Server {
    users:  HashMap<UserId, User>,
    rooms:  HashMap<RoomId, Room>,
}

// Public method.

impl Server {
    pub fn handle_json(&mut self, json: &str) -> KuraKuraResponse {
        match from_str(&json) {
            Ok(CreateUser  {}             ) => {self.create_user()}
            Ok(SetName     {auth, name}   ) => {self.set_name(auth, name)}
            Ok(CreateRoom  {auth}         ) => {self.create_room(auth)}
            Ok(JoinRoom    {auth, room}   ) => {self.join_room(auth, room)}
            Ok(TakeTurn    {auth, turn}   ) => {self.take_turn(auth, turn)}
            Err(_)                          => {Err(InvalidJson {})}
        }
    }
}

// Private methods directly corresponding to API calls.

impl Server {
    fn create_user(&mut self) -> KuraKuraResponse {
        let user_id = Uuid::new_v4().to_string();

        self.users.insert(user_id.clone(), User {
            id:         user_id.clone(),
            name:       "".into(),
            room_id:    None,
        });

        Ok(UserCreated {id: user_id})
    }

    fn set_name(&mut self, auth: UserId, name: String) -> KuraKuraResponse {
        if name.len() > 250 {return Err(NameTooLong);}

        let user: &mut User = self.get_user(&auth)?;
        (*user).name = name;
        Ok(NameSet {})
    }

    fn create_room(&mut self, auth: UserId) -> KuraKuraResponse {
        let user: &mut User = self.get_user(&auth)?;

        if user.room_id != None {
            return Err(UserAlreadyHasRoom);
        }

        let room_id = Uuid::new_v4().to_string();
        user.room_id = Some(room_id.clone());

        self.rooms.insert(room_id.clone(), Room {
            id:                 room_id.clone(),
            host_user_id:       auth,
            guest_user_id:      None,
            game:               Game::new(4, 2),
            host_plays_black:   true, // todo: make this random
            creation_time:      Instant::now(),
        });

        Ok(RoomCreated {id: room_id})
    }

    fn join_room(&mut self, auth: UserId, room_id: RoomId) -> KuraKuraResponse {

        // Todo: these two stanzas of code are copied from the methods
        // .get_user() and .get_room(), which I can't just call directly
        // I think because the mutable reference to a User or Room that
        // each one returns also keeps the mutable reference to self alive,
        // meaning that there would be two mutable references to self alive
        // at the same time, which is disallowed. So, I need to figure
        // out how to resolve this without violating DRY. Whatever solution
        // I come to, I should apply it to any other usage of this DRY
        // violation.

        let user = match self.users.get_mut(&auth) {
            Some(u) => Ok(u),
            None => Err(UserNotFound),
        }?;

        let room = match self.rooms.get_mut(&room_id) {
            Some(u) => Ok(u),
            None => Err(RoomNotFound),
        }?;

        //let user: &mut User = self.get_user(&auth)?;
        //let room: &mut Room = self.get_room(&room_id)?;

        if user.room_id != None {return Err(UserAlreadyHasRoom);}
        if room.guest_user_id != None {return Err(RoomAlreadyHasGuest);}

        user.room_id = Some(room_id);
        room.guest_user_id = Some(auth);

        Ok(RoomJoined {})
    }

    fn take_turn(&mut self, auth: UserId, turn: Turn) -> KuraKuraResponse {
        let Some(user)    = self.users.get_mut(&auth)    else {return Err(UserNotFound);};
        let Some(room_id) = user.room_id.clone()         else {return Err(UserDoesntHaveRoom);};
        let Some(room)    = self.rooms.get_mut(&room_id) else {return Err(RoomNotFound);};
        let Some(_)       = room.guest_user_id.clone()   else {return Err(RoomDoesntHaveGuest);};

        // Todo: make sure that user really is that player!

        match room.game.turn(turn) {
            Ok(_) => Ok(TurnTaken {}),
            Err(turn_error) => Err(InvalidTurn {error: turn_error}),
        }
    }
}

// Constructor.

impl Server {
    pub fn new() -> Self {
        Server {
            users: HashMap::new(),
            rooms: HashMap::new(),
        }
    }
}

// Private utility methods.

impl Server {
    fn get_user(&mut self, user_id: &UserId) -> Result<&mut User, KuraKuraErr> {
        match self.users.get_mut(user_id) {
            Some(u) => Ok(u),
            None => Err(UserNotFound),
        }
    }

    fn get_room(&mut self, room_id: &RoomId) -> Result<&mut Room, KuraKuraErr> {
        match self.rooms.get_mut(room_id) {
            Some(u) => Ok(u),
            None => Err(RoomNotFound),
        }
    }
}

// Display stuff.

impl Display for User {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let bold = "\x1b[1m";
        let reset = "\x1b[0m";
        write!(f, "{bold}User ID:{reset} {}... {bold}Name:{reset} {}", &self.id[0..4], self.name)
    }
}

impl Display for Room {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let bold = "\x1b[1m";
        let reset = "\x1b[0m";

        writeln!(f, "{bold}Room ID:{reset} {}... {bold}Host ID:{reset} {}... {bold}Guest ID:{reset} {}{}",
               &self.id[0..4],
               &self.host_user_id[0..4],
               match &self.guest_user_id {
                   None => "None",
                   Some(id) => &id[0..4],
               },
               match &self.guest_user_id {
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
        writeln!(f, "{under}Users:{reset}")?;
        writeln!(f)?;

        for user in self.users.keys() {
            writeln!(f, "    {}", self.users.get(user).unwrap())?;
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

