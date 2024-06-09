
use kurakura::Game;
use std::time::{Instant, Duration};
use std::collections::HashMap;

struct Player {
    id:     String,
    name:   String,
}

struct Room {
    host:               Option<Player>,
    guest:              Option<Player>,
    game:               Game,
    host_plays_black:   bool,
    creation_time:      Instant,
}

struct Lobby {
    rooms:  HashMap<String, Room>;
}

impl Lobby {
    makeRoom() -> String {
    }

    joinRoom(room_id: String, player_name: String) -> String {
    }

    play(room_id: String, player_id) {
    }
}

