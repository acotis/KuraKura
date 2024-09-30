
#![allow(unused)]

use std::clone::Clone;
use crate::game::Game;
use crate::game::types::Turn;
use crate::game::types::TurnError;
use crate::server::message_types::UserRequest::*;
use crate::server::message_types::UserError::*;
use crate::server::message_types::UserOk::*;
use crate::server::message_types::UserInfo::*;
use crate::server::message_types::UserMessage::*;
use crate::server::message_types::ServerError::*;
use crate::game::types::Player::Black;
//use std::time::{Instant};
use std::collections::HashMap;
use std::fmt::{Display, Formatter, Error};
use serde::{Serialize, Deserialize};
use serde_json::from_str;
use axum::extract::ws::{WebSocket, Message::{self, Text}};
use futures_util::{SinkExt, stream::SplitSink};
use crate::server::types::{Socket, SocketId, Room, RoomId};
use crate::server::message_types::{*, UserBroadcast::*};

// Server struct.

pub struct Server {
    sockets:    HashMap<SocketId, Socket>,
    rooms:      HashMap<RoomId, Room>,
}

// Constructor.

impl Server {
    pub fn new() -> Self {
        Server {
            sockets: HashMap::new(),
            rooms: HashMap::new(),
        }
    }
}

// Public methods of Server.

impl Server {
    pub fn register_socket(&mut self) -> SocketId {
        let socket = Socket::new();
        let socket_id = socket.id;
        self.sockets.insert(socket_id, socket);
        socket_id
    }

    pub fn handle_request(&mut self, socket_id: SocketId, json: &str) -> ServerResult {
        if self.sockets.get(&socket_id).is_none() {
            Err(SocketNotFound)
        } else {
            let api_result = match from_str(&json) {
                Ok(CreateRoom {name}      ) => {self.create_room (socket_id, name      )},
                Ok(JoinRoom   {name, room}) => {self.join_room   (socket_id, name, room)},
                Ok(TakeTurn   {turn}      ) => {self.take_turn   (socket_id, turn      )},
                Ok(DebugLog               ) => {self.debug_log   (                     )},
                Err(_)                      => {Err(InvalidJson)},
            };

            // Construct the actual Vec of messages to send out.

            Ok(
                match api_result {
                    Err(error) => {vec![(socket_id, ResponseMessage(Err(error)))]},
                    Ok((okay, Silent)) => {vec![(socket_id, ResponseMessage(Ok(okay)))]},
                    Ok((okay, RoomBroadcast(room_id, info))) => todo!(),
                }
            )
        }
    }
}

//vec![(socket_id, Err(InvalidJson))]},


// Private methods directly corresponding to API calls.

impl Server {
    fn create_room(&mut self, socket_id: SocketId, name: String) -> ApiResult {
        let socket = self.sockets.get_mut(&socket_id).expect("socket lookup in create_room()");

        if socket.room_id != None {
            return Err(AlreadyInARoom);
        }

        /* todo: validate name*/

        let mut room = Room::new();
        let room_id = room.id;
        room.socket_ids.push(socket_id);
        self.rooms.insert(room_id, room);
        socket.room_id = Some(room_id);
        socket.name = name;

        Ok((RoomCreated {id: room_id}, Silent))
    }

    fn join_room(&mut self, socket_id: SocketId, name: String, room_id: RoomId) -> ApiResult {
        let socket = self.sockets.get_mut(&socket_id).expect("socket lookup in join_room()");
        let room = self.rooms.get_mut(&room_id).ok_or(RoomNotFound)?;

        /* todo: validate name */

        if socket.room_id != None {
            return Err(AlreadyInARoom);
        }

        socket.room_id = Some(room_id);
        room.socket_ids.push(socket_id);
        socket.name = name;

        // Todo: let the Game decide whether the new player is a player or
        // a spectator.
        // Todo: broadcast the fact that a player joined, as well as the new
        // state of the room, to all connected sockets.

        if room.socket_ids.len() <= 2 {
            Ok((JoinedAsPlayer, Silent))
        } else {
            Ok((JoinedAsSpectator, Silent))
        }
    }

    fn take_turn(&mut self, socket_id: SocketId, turn: Turn) -> ApiResult {
        let socket = self.sockets.get_mut(&socket_id).expect("socket lookup in take_turn()");
        let room_id = socket.room_id.ok_or(NotInARoom)?;
        let room = self.rooms.get_mut(&room_id).ok_or(RoomNotFound)?;

        // Todo: make it broadcast the turn that was taken, and the
        // new room state, to all players.

        match room.game.turn(turn) {
            Ok(_)           => Ok((TurnAccepted, Silent)),
            Err(turn_error) => Err(InvalidTurn(turn_error)),
        }
    }

    fn debug_log(&mut self) -> ApiResult {
        print!("{self}");

        // Always return InvalidJson so as to not reveal that the API call
        // did anything.
        Err(InvalidJson)
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

impl Display for Room {
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

impl Display for Server {
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

