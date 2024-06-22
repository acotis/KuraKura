
use kurakura::Game;
use std::time::{Instant, Duration};
use std::collections::HashMap;
use uuid::Uuid;

// Public-facing types.

type UserId = String;
type RoomId = String;

enum KuraKuraRequest {
    CreateUser  {},
    SetName     {auth: UserId, name: String},
    CreateRoom  {auth: UserId},
    JoinRoom    {auth: UserId, room: RoomId},
    TakeTurn    {auth: UserId, details: TurnDetails},
}

enum KuraKuraResponse {
    UserCreated {id: UserId},
    NameSet     {},
    RoomCreated {id: RoomId},
    RoomJoined  {},
    TurnTaken   {},

    UserNotFound,
    UserAlreadyHasRoom,
}

// Implementation of server which publically deals in those types.

struct User {
    name:   String,
    room:   Option<RoomId>,
}

struct Room {
    host:               UserId,
    guest:              Option<UserId>,
    game:               Game,
    host_plays_black:   bool,
    creation_time:      Instant,
}

struct Server {
    users:  HashMap<UserId, User>;
    rooms:  HashMap<RoomId, Room>;
}

impl Server {
    pub fn handle_request(&mut self, request: KuraKuraRequest) -> KuraKuraResponse {
        match request {
            CreateUser  {}              => {create_user()}
            SetName     {auth, name}    => {set_name(auth, name)}
            CreateRoom  {auth}          => {create_room(auth)}
            JoinRoom    {auth, room}    => {join_room(auth, room)}
            Taketurn    {auth, details} => {take_turn(auth, details)}
        }
    }

    fn create_user() {
        let user_id = Uuid::new_v4().to_string();

        self.users.insert(user_id, User {
            name: "".into(),
            room: None,
        }

        UserCreated(user_id)
    }




    fn createRoom(self, user_id: UserId) -> Result<RoomId, ServerError> {
        let user = self.get_user(user_id)?;

        if user.room != None {
            return Err(UserAlreadyHasRoom);
        }

        let room_id = Uuid::new_v4().to_string();

        self.rooms.insert(user_id, Room {

    }

    fn joinRoom(self, user_id: UserId, room_id: RoomId) {

    }

    fn play(self, user_id: UserId, 
}

impl Server {
    fn get_user(self, user_id) -> Result<UserId, ServerError> {
        match self.users.get(user_id) {
            Some(u) => Ok(u),
            None => Err(PlayerNotFound),
        }
    }
}

