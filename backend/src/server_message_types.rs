
use serde::{Serialize, Deserialize};
use crate::server_types::*;
use crate::{Turn, TurnError};

// User inputs and outputs (i.e., things the user sends us and receives in response).

#[derive(Debug, Serialize, Deserialize)]
pub enum UserRequest {
    Register    {name: String},
    Login       {auth: AccountId},
    CreateRoom,
    JoinRoom    {room: RoomId},
    TakeTurn    {turn: Turn},

    DebugLog,   // Debugging only, turn this off in production.
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum UserOk {
    AccountRegistered   {id: AccountId},
    RoomCreated         {id: RoomId},
    JoinedAsPlayer,
    JoinedAsSpectator,
    Okay,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum UserErr {
    AccountNotFound,
    AlreadyLoggedIn,
    NotLoggedIn,
    RoomNotFound,
    AccountAlreadyHasRoom,     // Todo: add a paramater giving the room ID?
    RoomAlreadyHasGuest,    // (probably don't add such a parameter here for the player ID) (definitely not, that would reveal someone else's API key)
    NameTooLong,
    AccountDoesntHaveRoom,
    RoomDoesntHaveGuest,
    AccountPlayedWrongColor,
    InvalidTurn {error: TurnError},
    NotImplemented,
    InvalidJson,
}

pub type UserMessage = Result<UserOk, UserErr>;

// Server outputs (i.e., possible return values of the server's .handle_request() method).

#[derive(Debug)]
pub enum ServerError {SocketNotFound}
pub type ServerOk = Vec<(SocketId, UserMessage)>;
pub type ServerResult = Result<ServerOk, ServerError>;

