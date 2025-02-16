
use serde::{Serialize, Deserialize};
use derivative::Derivative;
use std::fmt::Debug;
use crate::server::types::*;
use crate::server::game::Game;

/*

// Stuff to derive:
// #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]

// Note about types: I know it sucks that the API call methods have to start with
// a redundant socket lookup which also must be unwrapped instead of .ok_or()'d.
// But it is simply not appropriate for the API call methods to ever return a
// SocketNotFound error, because control is not delegated to the methods until the
// JSON in the request is parsed, and this parsing itself can yield an error, and
// SocketNotFound should be checked for first because it is the more fundamental
// error between the two.

// Below are the two top-level container types for server-internal message-passing.
// ServerResult is the return type of Server.handle_request(), which interfaces
// with the threads that handle WebSockets. ApiResult is used inside the Server
// class as the type returned by its API call handling methods.
*/

pub type ServerResult<G> = Result<ServerOk<G>, ServerError>;
pub type ApiResult<G> = Result<(UserOk<G>, UserBroadcast<G>), UserError<G>>;

// ServerResult can be our one type of error, or it can be a Vec of socket ID's
// and UserMessages, which is an instruction to the WebSocket thread to send
// all the messages to those socket ID's.

#[derive(Debug)]
pub enum ServerError {SocketNotFound}
pub type ServerOk<G> = Vec<(SocketId, UserMessage<G>)>;

// UserMessage is the umbrella type for any message we might send to the user.
// It has variants "Info" (for things we might send them autonomously) and
// "ResponseMessage" (for things we send in response to API calls).

#[derive(Serialize, Deserialize)]
#[derive(Derivative)]
#[derivative(Debug(bound="G: Debug, G::Turn: Debug, G::PlayerRole: Debug, G::TurnError: Debug"))]
#[derivative(PartialEq(bound="UserInfo<G>: PartialEq, UserResponse<G>: PartialEq"))]
#[derivative(Eq(bound="UserInfo<G>: Eq, UserResponse<G>: Eq"))]
#[derivative(Clone(bound="G::PlayerRole: Clone, UserResponse<G>: Clone"))]
#[serde(bound(
    deserialize = "G::PlayerRole: Deserialize<'de>, G: Deserialize<'de>, G::TurnError: Deserialize<'de>"
))]
pub enum UserMessage<G: Game> {
    Info(UserInfo<G>),
    #[serde(untagged)] ResponseMessage(UserResponse<G>),
}

// Here is the content of the Info variant:

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum UserInfo<G: Game> {
    TurnTaken {room_state: RoomState<G>, turn: G::Turn},
    PlayerJoined {player_id: usize, player_role: G::PlayerRole, room_state: RoomState<G>, player_name: String},
}

// Here is the content of the UserResponse variant:

pub type UserResponse<G> = Result<UserOk<G>, UserError<G>>;

#[derive(Serialize, Deserialize)]
#[derive(Derivative)]
#[serde(bound(
    deserialize = "G::PlayerRole: Deserialize<'de>, G: Deserialize<'de>"
))]
#[derivative(Debug(bound="G::PlayerRole: Debug, G: Debug"))]
#[derivative(Clone(bound="G::PlayerRole: Clone"))]
#[derivative(PartialEq(bound="G::PlayerRole: PartialEq, G: PartialEq"))]
#[derivative(Eq(bound="G::PlayerRole: Eq, G: Eq"))]
pub enum UserOk<G: Game> {
    RoomCreated {player_id: usize, player_role: G::PlayerRole, room_state: RoomState<G>},
    RoomJoined  {player_id: usize, player_role: G::PlayerRole, room_state: RoomState<G>},
    TurnAccepted {room_state: RoomState<G>},
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum UserError<G: Game> {
    AccountNotFound,
    AlreadyLoggedIn,
    AlreadyInARoom,
    NotInARoom,
    NotLoggedIn,
    RoomNotFound,
    AccountAlreadyHasRoom,
    RoomAlreadyHasGuest,    // (don't add a parameter giving the player ID, that would reveal someone else's API key)
    NameTooLong,
    AccountDoesntHaveRoom,
    RoomDoesntHaveGuest,
    AccountPlayedWrongColor,
    InvalidTurn(G::TurnError),
    NotImplemented,
    InvalidJson(String),
    InternalFailure,
}

// Many types above include a RoomState field.

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RoomState<G> {
    pub room_id: RoomId,
    pub player_names: Vec<String>,
    pub game_state: G,
}

// We also have a type UserBroadcast which is used only internally
// as part of the API call delegation architecture.

#[derive(Derivative)]
#[derive(Clone, Serialize, Deserialize)]
#[derivative(Debug(bound="UserInfo<G> : Debug"))]
#[derivative(PartialEq(bound="UserInfo<G> : PartialEq"))]
#[derivative(Eq(bound="UserInfo<G> : Eq"))]
#[serde(bound(
    deserialize = "G::PlayerRole: Deserialize<'de>, G: Deserialize<'de>"
))]
pub enum UserBroadcast<G: Game> {
    Silent,
    RoomBroadcast(RoomId, UserInfo<G>),
}


// Finally, there is the top-level type of a User Request.

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum UserRequest<G: Game> {
    CreateRoom {name: String, parameters: G::Parameters},
    JoinRoom   {name: String, room: RoomId},
    TakeTurn   {turn: G::Turn},

    DebugLog,   // Debugging only, turn this off in production.
}

