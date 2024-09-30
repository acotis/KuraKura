
use serde::{Serialize, Deserialize};
use crate::server::types::*;
use crate::server::game::Game;

// Note about types: I know it sucks that the API call methods have to start with
// a redundant socket lookup which also must be unwrapped instead of .ok_or()'d.
// But it is simply not appropriate for the API call methods to ever return a
// SocketNotFound error, because control is not delegated to the methods until the
// JSON in the request is parsed, and this parsing itself can yield an error, and
// SocketNotFound should be checked for first because it is the more fundamental
// error between the two.

// USER REQUESTS: Things the user can send us.

#[derive(Debug, Serialize, Deserialize)]
pub enum UserRequest<T> {
    CreateRoom {name: String},
    JoinRoom   {name: String, room: RoomId},
    TakeTurn   {turn: T},

    DebugLog,   // Debugging only, turn this off in production.
}

// USER RESPONSES: Things we can reply to a user request (OKs and Errors).

pub type UserResponse<E> = Result<UserOk, UserError<E>>;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum UserOk {
    RoomCreated {id: RoomId},
    JoinedAsPlayer,
    JoinedAsSpectator,
    TurnAccepted,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum UserError<E> {
    AccountNotFound,
    AlreadyLoggedIn,
    AlreadyInARoom,
    NotInARoom,
    NotLoggedIn,
    RoomNotFound,
    AccountAlreadyHasRoom,     // Todo: add a paramater giving the room ID?
    RoomAlreadyHasGuest,    // (probably don't add such a parameter here for the player ID) (definitely not, that would reveal someone else's API key)
    NameTooLong,
    AccountDoesntHaveRoom,
    RoomDoesntHaveGuest,
    AccountPlayedWrongColor,
    InvalidTurn(E),
    NotImplemented,
    InvalidJson,
}

// USER INFOS: Things we can send the user autonomously.

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum UserInfo {
    // nothing yet, will be used for turns being taken by the other player I guess
}

// User message: an enum for the two types of things we can send to a user.

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum UserMessage<E> {
    ResponseMessage(UserResponse<E>),
    InfoMessage(UserInfo),
}

// User broadcast: a broadcast into a paritcular room, or silence.

pub enum UserBroadcast {
    Silent,
    RoomBroadcast(RoomId, UserInfo),
}

// API Result: the outcome of calling an endpoint-specific server method.

pub type ApiResult<E> = Result<(UserOk, UserBroadcast), UserError<E>>;

// Server outputs (i.e., possible return values of the server's .handle_request() method).

#[derive(Debug)]
pub enum ServerError {SocketNotFound}
pub type ServerOk<E> = Vec<(SocketId, UserMessage<E>)>;
pub type ServerResult<E> = Result<ServerOk<E>, ServerError>;

