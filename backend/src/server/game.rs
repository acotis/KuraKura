
use std::fmt::{Debug, Display};
use serde::Serialize;
use serde::de::DeserializeOwned;

// Todo: remove as many trait bounds as possible.

pub trait Game : Debug + Display {
    type Turn : DeserializeOwned;
    type TurnError : Debug + Clone + PartialEq + Eq + Serialize + Send;
    type Outcome;

    // Todo: in .new(), add a parameter for Parameters.

    fn new() -> Self;
    fn turn(&mut self, turn: Self::Turn) ->
        Result<Option<Self::Outcome>, Self::TurnError>;
}

