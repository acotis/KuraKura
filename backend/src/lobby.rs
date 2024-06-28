
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

enum KuraKuraOk {
    UserCreated {id: UserId},
    NameSet     {},
    RoomCreated {id: RoomId},
    RoomJoined  {},
    TurnTaken   {},
}

enum KuraKuraErr {
    UserNotFound,
    UserAlreadyHasRoom,     // Todo: add a paramater giving the room ID?
    RoomAlreadyHasGuest,    // (probably don't add such a parameter here for the player ID) (definitely not, that would reveal someone else's API key)
}

type KuraKuraResponse = Result<KuraKuraOk, KuraKuraErr>;

// Implementation of server which publically deals in those types.

struct User {
    name:       String,
    room_id:    Option<RoomId>,
}

struct Room {
    host_user_id:       UserId,
    guest_user_id:      Option<UserId>,
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

    fn create_user(&mut self) -> KuraKuraResponse {
        let user_id = Uuid::new_v4().to_string();

        self.users.insert(user_id, User {
            name: "".into(),
            room: None,
        }

        UserCreated(user_id)
    }

    fn set_name(&mut self, auth: UserId, name: String) -> KuraKuraResponse {
        let user: &mut User = self.get_user(auth)?;
        *user.name = name;
        NameSet
    }

    fn create_room(&self, auth: UserId) -> KuraKuraResponse {
        let user: &mut User = self.get_user(auth)?;

        if user.room != None {
            return UserAlreadyHasRoom;
        }

        let room_id = Uuid::new_v4().to_string();
        *user.room_id = room_id;

        self.rooms.insert(room_id, Room {
            host_user_id:       auth,
            guest_user_id:      None,
            game:               Game::new(9, 5),
            host_plays_black:   true, // todo: make this random
            creation_time:      Instant::now(),
        });

        RoomCreated(room_id)
    }

    fn join_room(&self, auth: UserId, room_id: RoomId) {
        let user: &mut User = self.get_user(auth)?;
        let room: &mut Room = self.get_room(room_id)?;

        if user.room != None {return UserAlreadyHasRoom;}
        if room.guest_user_id != None {return RoomAlreadyHasGuest;}

        *user.room = Some(room_id);
        room.guest_user_id = Some(auth);
    }

    fn play(self, auth: UserId, details: TurnDetails) {
        // todo
    }
}

impl Server {
    fn get_user(&self, user_id: UserId) -> Result<&mut User, KuraKuraErr> {
        match self.users.get_mut(&user_id) {
            Some(u) => Ok(u),
            None => Err(PlayerNotFound),
        }
    }

    fn get_room(&self, room_id: RoomId) -> Result<&mut Room, KuraKuraErr> {
        match self.rooms.get_mut(&room_id) {
            Some(u) => Ok(u),
            None => Err(RoomNotFound),
        }
    }
}

