
use std::fmt::{Debug, Display};
use serde::Serialize;
use serde::de::DeserializeOwned;

pub trait Game : Debug + Display + Clone {
    type PlayerRole : Serialize;
    type Turn : Serialize + DeserializeOwned + Clone;
    type TurnError : Clone + PartialEq + Eq + Serialize + Send;
    type Outcome;

    fn new() -> Self;
    fn add_player(&mut self) -> Self::PlayerRole;
    fn turn(&mut self, turn: Self::Turn) ->
        Result<Option<Self::Outcome>, Self::TurnError>;
}

