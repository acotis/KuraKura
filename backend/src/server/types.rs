
use crate::server::copy_id::CopyId;
use crate::server::game::Game;

pub type RoomId = CopyId<Room<bool>>;
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

