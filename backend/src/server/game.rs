
use std::fmt::{Debug, Display};
use serde::Serialize;
use serde::de::DeserializeOwned;

pub trait Game : Debug + Display {
    type Turn : DeserializeOwned;
    type TurnError : Clone + PartialEq + Eq + Serialize + Send;
    type Outcome;

    fn new() -> Self;
    fn turn(&mut self, turn: Self::Turn) ->
        Result<Option<Self::Outcome>, Self::TurnError>;
}

