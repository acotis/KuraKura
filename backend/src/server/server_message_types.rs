
use serde::{Serialize, Deserialize};
use crate::server_types::*;
use crate::{Turn, TurnError};

// Note about types: I know it sucks that the API call methods have to start with
// a redundant socket lookup which also must be unwrapped instead of .ok_or()'d.
// But it is simply not appropriate for the API call methods to ever return a
// SocketNotFound error, because control is not delegated to the methods until the
// JSON in the request is parsed, and this parsing itself can yield an error, and
// SocketNotFound should be checked for first because it is the more fundamental
// error between the two.

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
pub enum UserInfo {
    AccountRegistered   {id: AccountId},
    RoomCreated         {id: RoomId},
    JoinedAsPlayer      {id: RoomId},
    JoinedAsSpectator   {id: RoomId},
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum UserError {
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

pub type UserMessage = enum {
    Okay,
    Error(UserError),
    Info(UserInfo),
}

pub type UserBroadcast = enum {
    Silent,
    SocketBroadcast(SocketId, UserMessage),
    AccountBroadcast(AccountId, UserMessage),
    RoomBroadcast(RoomId, UserMessage),
}

pub type ApiResult = Result<UserBroadcast, UserError>;

// Server outputs (i.e., possible return values of the server's .handle_request() method).

#[derive(Debug)]
pub enum ServerError {SocketNotFound}
pub type ServerOk = Vec<(SocketId, UserMessage)>;
pub type ServerResult = Result<ServerOk, ServerError>;

