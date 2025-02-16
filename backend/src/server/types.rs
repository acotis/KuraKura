use std::time::{Duration, Instant};

use crate::server::copy_id::CopyId;
use crate::server::game::Game;

// ID types.

pub type RoomId = CopyId<Room<()>>;
pub type SocketId = CopyId<Socket>;

// Socket type.

pub struct Socket {
    pub id:      SocketId,
    pub room_id: Option<RoomId>,
    pub name:    String,
    pub last_active: Instant,
}

impl Socket {
    pub fn new() -> Self {
        Socket {
            id:      SocketId::new(),
            room_id: None,
            name:    String::from("Guest"),
            last_active: Instant::now(),
        }
    }

    pub fn stale(&self) -> bool {
        Instant::now() - self.last_active > Duration::from_secs(3600)
    }
}

// Room type.

pub struct Room<G> {
    pub id:         RoomId,
    pub socket_ids: Vec<SocketId>, // must be a vec for multiplayer games (N > 2)
    pub game:       G,
}

impl<G: Game> Room<G> {
    pub fn new(parameters: G::Parameters) -> Self {
        Room {
            id:         RoomId::new(),
            socket_ids: vec![],
            game:       G::new(parameters),
        }
    }
}

