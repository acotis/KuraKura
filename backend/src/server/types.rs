
use crate::game::Game;
use crate::server::copy_id::CopyId;

pub type RoomId = CopyId<Room>;
pub type SocketId = CopyId<Socket>;

// Socket type.

pub struct Socket {
    pub id:      SocketId,
    pub room_id: Option<RoomId>,
    pub name:    String,
}

impl Socket {
    pub fn new() -> Self {
        Socket {
            id:      SocketId::new(),
            room_id: None,
            name:    String::from("Guest"),
        }
    }
}

// Room type.

pub struct Room {
    pub id:         RoomId,
    pub socket_ids: Vec<SocketId>, // must be a vec for multiplayer games (N > 2)
    pub game:       Game,
}

impl Room {
    pub fn new() -> Self {
        Room {
            id:         RoomId::new(),
            socket_ids: vec![],
            game:       Game::new(4, 2),
        }
    }
}

