
use kurakura::Game;
use std::time::{Instant, Duration};
use std::collections::HashMap;
use uuid::Uuid;

type UserId = String;
type RoomId = String;

// "API calls" for the server to implement:
//
//      - [       ] createUser()     -> Result<UserId, ServerError>
//      - [auth: U] setName(N)       -> Result<(), ServerError>
//      - [auth: U] createRoom()     -> Result<RoomId, ServerError>
//      - [auth: U] joinRoom(R)      -> Result<(), ServerError>
//      - [auth: U] play(move)       -> Result<(), ServerError>

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

enum KuraKuraRequest {
    CreateUser  {},
    SetName     {auth: UserId, name: String},
    CreateRoom  {auth: UserId},
    JoinRoom    {auth: UserId, room: RoomId},
    Play        {auth: UserId, play: //todo 
}

enum KuraKuraResponse {
    UserCreated {id: UserId},



struct Server {
    users:  HashMap<UserId, User>;
    rooms:  HashMap<RoomId, Room>;
}

enum ServerError {
    UserNotFound,
    UserAlreadyHasRoom,
}

impl Server {

    // createUser() can't fail right now, but I'm defining the return type as
    // a Result just in case we ever decide there's a case where it should.

    fn createUser(self, name: String) -> Result<UserId, ServerError> {
        let user_id = Uuid::new_v4().to_string();

        self.users.insert(user_id, User {
            name: name,
            room: None,
        }

        user_id
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

