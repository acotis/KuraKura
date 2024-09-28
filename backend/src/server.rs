
#![allow(unused)]

use std::clone::Clone;
use std::ops::Deref;

pub struct Id {
    pub id: u32,
}

impl Clone for Id {
    fn clone(&self) -> Self {
        Id {
            id: self.id
        }
    }
}

impl Deref<str> for Id {
    fn deref(&self) -> &str {
        "Hello world"
    }
}

pub fn clone_it() -> Id {
    let id = Id {id: 1};
    id.clone()
}

