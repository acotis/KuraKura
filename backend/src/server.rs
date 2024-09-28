
#![allow(unused)]

use std::clone::Clone;
pub use crate::server_types::Id;

// Server struct.

pub struct Server {
    dummy: Id
}

// Constructor.

impl Server {
    pub fn get_dummy(&self) -> Id {
        self.dummy.clone()
    }
}

