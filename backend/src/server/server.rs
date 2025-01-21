
use crate::server::message_types::UserRequest::*;
use crate::server::message_types::UserError::*;
use crate::server::message_types::UserOk::*;
use crate::server::message_types::UserInfo::*;
use crate::server::message_types::UserMessage::*;
use crate::server::message_types::ServerError::*;
use crate::server::game::Game as GameTrait;
//use std::time::{Instant};
use std::collections::HashMap;
use std::fmt::{Display, Formatter, Error};
use serde_json::from_str;
use crate::server::types::{Socket, SocketId, Room, RoomId};
use crate::server::message_types::{*, UserBroadcast::*};

// Server struct.

pub struct Server<Game> {
    sockets:    HashMap<SocketId, Socket>,
    rooms:      HashMap<RoomId, Room<Game>>,
}

// Constructor.

impl<Game: GameTrait> Server<Game> {
    pub fn new() -> Self {
        Server {
            sockets: HashMap::new(),
            rooms: HashMap::new(),
        }
    }
}

// Public methods of Server.

impl<Game: GameTrait> Server<Game> {
    pub fn register_socket(&mut self) -> SocketId {
        let socket = Socket::new();
        let socket_id = socket.id;
        self.sockets.insert(socket_id, socket);
        socket_id
    }

    pub fn handle_request(&mut self, socket_id: SocketId, json: &str) -> ServerResult<Game> {
        if self.sockets.get(&socket_id).is_none() {
            Err(SocketNotFound)
        } else {
            let api_result = match from_str::<UserRequest<Game::Turn>>(&json) {
                Ok(CreateRoom {name}      ) => {self.create_room (socket_id, name      )},
                Ok(JoinRoom   {name, room}) => {self.join_room   (socket_id, name, room)},
                Ok(TakeTurn   {turn}      ) => {self.take_turn   (socket_id, turn      )},
                Ok(DebugLog               ) => {self.debug_log   (                     )},
                Err(error)                  => {Err(InvalidJson(error.to_string()))},
            };

            // Construct the actual Vec of messages to send out.

            Ok(
                match api_result {
                    Err(error) => {vec![(socket_id, ResponseMessage(Err(error)))]},
                    Ok((okay, broadcast)) => {
                        let mut ret = match broadcast {
                            Silent => vec![],
                            RoomBroadcast(room_id, info) =>
                                self.rooms
                                    .get(&room_id)
                                    .unwrap()
                                    .socket_ids
                                    .iter()
                                    .map(|&id| (id, Info(info.clone())))
                                    .collect()
                        };
                        ret.insert(0, (socket_id, ResponseMessage(Ok(okay))));
                        ret
                    }
                }
            )
        }
    }
}

//vec![(socket_id, Err(InvalidJson))]},


// Private methods directly corresponding to API calls.

impl<Game: GameTrait> Server<Game> {
    fn create_room(&mut self, socket_id: SocketId, name: String) -> ApiResult<Game> {
        let socket = self.sockets.get_mut(&socket_id).expect("socket lookup in create_room()");

        if socket.room_id != None {
            return Err(AlreadyInARoom);
        }

        let mut room = Room::new();
        let room_id = room.id;
        room.socket_ids.push(socket_id);
        self.rooms.insert(room_id, room);
        socket.room_id = Some(room_id);
        socket.name = name;

        Ok((RoomCreated {id: room_id}, Silent))
    }

    fn join_room(&mut self, socket_id: SocketId, name: String, room_id: RoomId) -> ApiResult<Game> {
        let socket = self.sockets.get_mut(&socket_id).expect("socket lookup in join_room()");
        let room = self.rooms.get_mut(&room_id).ok_or(RoomNotFound)?;

        if socket.room_id != None {
            return Err(AlreadyInARoom);
        }

        socket.room_id = Some(room_id);
        room.socket_ids.push(socket_id);
        socket.name = name;

        if room.socket_ids.len() <= 2 {
            Ok((JoinedAsPlayer, Silent))
        } else {
            Ok((JoinedAsSpectator, Silent))
        }
    }

    fn take_turn(&mut self, socket_id: SocketId, turn: Game::Turn) -> ApiResult<Game> {
        let socket = self.sockets.get_mut(&socket_id).expect("socket lookup in take_turn()");
        let room_id = socket.room_id.ok_or(NotInARoom)?;
        let room = self.rooms.get_mut(&room_id).ok_or(RoomNotFound)?;

        match room.game.turn(turn.clone()) {
            Ok(_)           => Ok((
                TurnAccepted,
                RoomBroadcast(room_id, TurnTaken {
                    turn: turn,
                    new_game_state: room.game.clone(),
                })
            )),
            Err(turn_error) => Err(InvalidTurn(turn_error)),
        }
    }

    fn debug_log(&mut self) -> ApiResult<Game> {
        print!("{self}");

        // Always return InvalidJson so as to not reveal that the API call
        // did anything.
        Err(InvalidJson(String::from("")))
    }
}

// Display stuff.

impl Display for Socket {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let bold = "\x1b[1m";
        let reset = "\x1b[0m";
        write!(f, "{bold}Socket ID:{reset} {}... {bold}Name:{reset} {}", &self.id[0..4], self.name)
    }
}

impl<Game: GameTrait> Display for Room<Game> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let bold = "\x1b[1m";
        let reset = "\x1b[0m";

        writeln!(f, "{bold}Room ID:{reset} {}...", &self.id[0..4])?;

        for socket_id in &self.socket_ids {
            writeln!(f, " ⮡ {bold}SocketId ID:{reset} {}...", &socket_id[0..4])?;
        }

        for line in self.game.to_string().lines() {
            writeln!(f, "  {}", line)?;
        }

        Ok(())
    }
}

impl<Game: GameTrait> Display for Server<Game> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let under = "\x1b[4m";
        let reset = "\x1b[0m";

        writeln!(f)?;
        writeln!(f, "{under}Sockets:{reset}")?;
        writeln!(f)?;

        for socket_id in self.sockets.keys() {
            writeln!(f, "    {}", self.sockets.get(socket_id).unwrap())?;
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

