
use std::marker::PhantomData;
use std::hash::{Hash, Hasher};
use serde::{Serialize, Serializer};
use std::ops::Deref;
use uuid::Uuid;
use crate::Game;
use crate::copy_id::CopyId;

pub type AccountId = CopyId<Account>;
pub type RoomId = CopyId<Room>;
pub type SocketId = CopyId<Socket>;

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

