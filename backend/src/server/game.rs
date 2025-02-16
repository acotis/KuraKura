
use std::fmt::Display;
use serde::Serialize;
use serde::de::DeserializeOwned;

pub trait Game : Serialize + Display + Clone {
    type Parameters : DeserializeOwned;
    type Turn : Serialize + DeserializeOwned + Clone;
    type Outcome: Serialize;
    type PlayerRole : Serialize + Clone;
    type TurnError : Serialize;

    fn new(parameters: Self::Parameters) -> Self;
    fn add_player(&mut self) -> Self::PlayerRole;
    fn turn(&mut self, player: usize, turn: Self::Turn) ->
        Result<Option<Self::Outcome>, Self::TurnError>;
}

