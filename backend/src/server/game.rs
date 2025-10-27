
use std::fmt::Display;
use serde::Serialize;
use serde::de::DeserializeOwned;

pub trait Game : Serialize + Display + Clone {
    type Parameters : DeserializeOwned;
    type Turn : Serialize + DeserializeOwned + Clone;
    type Outcome: Serialize + Clone;
    type PlayerRole : Serialize + Clone;
    type TurnError : Serialize;

    fn new(parameters: Self::Parameters) -> Self;
    fn add_player(&mut self) -> Self::PlayerRole;
    fn turn(&mut self, player: usize, turn: Self::Turn) ->
        Result<Option<Self::Outcome>, Self::TurnError>;
}

/* Hypothetical awesome version of this trait:
 *
 * pub trait Game {
 *     type GameState,          // Depicts the state of the game.
 *     type PlayerRole,         // What roles each player can have.
 *     type Parameters,         // Used to create the game.
 *     type ParametersError,    // Error when creating the game.
 *     type Turn,               // Take a turn.
 *     type TurnError,          // Error when taking your turn.
 *
 *     fn new(Parameters) -> Result<Self, ParametersError>;
 *     fn turn(usize, Turn) -> Result<(), TurnError>;
 *     fn add_player() -> PlayerRole;
 *     fn current_state() -> GameState;
 * }
 *
 * // Note: current_state() is a separate method because calls
 * // add_player() can modify the game state in ways that can't
 * // naturally be captured by the assigned PlayerRole. For
 * // example, a player might spawn into a game world and be
 * // given a starting location.
 * //
 * // You could argue that we should 
 *
 */

