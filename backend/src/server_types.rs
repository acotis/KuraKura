
use std::marker::PhantomData;
use uuid::Uuid;
use crate::Game;

pub type AccountId = String;
pub type RoomId = String;
pub type SocketId = String;

// The "ID" struct. Used to create the AccountId, RoomId, and SocketId types. These
// three types are identical, but we want the compiler to recognize them as distinct
// and not let us use them interchangeably. To do that, we give the Id struct a
// phantom type parameter <T>. The AccountId type is Id<Account>, the RoomId type
// is Id<Room> and the SocketId type is Id<Socket>.
//
// The Id simply holds an array of thirt-six u8's which is guaranteed to comprise
// a valid str (i.e., a valid UTF-8 string). When an Id is created, it fills out
// its str using a UUID generator that generates 36-character ascii-hexadecimal
// strings (32 meaningful chars and 4 hyphens). We hold the data in a field instead
// of behind a String (for example) so that the type can be plain-old-data and
// implement Copy guilt-free. This is nice because otherwise we end up invoking
// .clone() on the Id type five thousand times in the Server implementation.

pub struct Id<T> {
    id: [u8; 36],
    dummy: PhantomData<T>,
}

impl<T> Id<T> {
    pub fn new() -> Self {
        Id {
            id: [0; 36],
            dummy: PhantomData::<T>::default(),
        }
    }
}

// Socket type.

pub struct Socket {
    pub id:         SocketId,
    pub account_id: Option<AccountId>,
}

impl Socket {
    pub fn new() -> Self {
        Socket {
            id:         Uuid::new_v4().to_string(),
            account_id: None,
        }
    }
}

// Account type.

pub struct Account {
    pub id:         AccountId,
    pub room_id:    Option<RoomId>,
    pub socket_ids: Vec<SocketId>,
    pub name:       String,
}

impl Account {
    pub fn new() -> Self {
        Account {
            id:         Uuid::new_v4().to_string(),
            name:       "".into(),
            room_id:    None,
            socket_ids: vec![],
        }
    }
}

// Room type.

pub struct Room {
    pub id:          RoomId,
    pub account_ids: Vec<AccountId>, // must be a vec for multiplayer games (N > 2)
    pub game:        Game,
}

impl Room {
    pub fn new() -> Self {
        Room {
            id:          Uuid::new_v4().to_string(),
            game:        Game::new(4, 2),
            account_ids: vec![],
        }
    }
}

