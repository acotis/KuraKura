
use serde::{Serialize, Deserialize};
use crate::server::types::*;
use crate::server::game::Game as GameTrait;

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

pub type ServerResult<Game> = Result<ServerOk<Game>, ServerError>;
pub type ApiResult<Game> = Result<(UserOk<<Game as GameTrait>::PlayerRole>, UserBroadcast<Game>), UserError<<Game as GameTrait>::TurnError>>;

// ServerResult can be our one type of error, or it can be a Vec of socket ID's
// and UserMessages, which is an instruction to the WebSocket thread to send
// all the messages to those socket ID's.

#[derive(Debug)]
pub enum ServerError {SocketNotFound}
pub type ServerOk<Game> = Vec<(SocketId, UserMessage<Game>)>;

// UserMessage is the umbrella type for any message we might send to the user.
// It has variants "Info" (for things we might send them autonomously) and
// "ResponseMessage" (for things we send in response to API calls).

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(bound(
        deserialize = "Game::TurnError: Deserialize<'de>, Game: Deserialize<'de>, Game::PlayerRole: Deserialize<'de>"
))]
pub enum UserMessage<Game: GameTrait> {
    Info(UserInfo<Game, Game::Turn>),
    #[serde(untagged)] ResponseMessage(UserResponse<Game::PlayerRole, Game::TurnError>),
}

// Here is the content of the Info variant:

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum UserInfo<Game, Turn> {
    TurnTaken {new_game_state: Game, turn: Turn},
    PlayerJoined {new_game_state: Game, player_name: String},
}

// Here is the content of the UserResponse variant:

pub type UserResponse<PlayerRole, TurnError> = Result<UserOk<PlayerRole>, UserError<TurnError>>;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum UserOk<PlayerRole> {
    RoomCreated {id: RoomId},
    RoomJoined {player_role: PlayerRole},
    TurnAccepted,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum UserError<TurnError> {
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
    InvalidTurn(TurnError),
    NotImplemented,
    InvalidJson(String),
    InternalFailure,
}

// We also have a type UserBroadcast which is used only internally
// as part of the API call delegation architecture.

pub enum UserBroadcast<Game: GameTrait> {
    Silent,
    RoomBroadcast(RoomId, UserInfo<Game, Game::Turn>),
}

// Finally, there is the top-level type of a User Request.

#[derive(Debug, Serialize, Deserialize)]
pub enum UserRequest<Turn> {
    CreateRoom {name: String},
    JoinRoom   {name: String, room: RoomId},
    TakeTurn   {turn: Turn},

    DebugLog,   // Debugging only, turn this off in production.
}

