
#![allow(unused)]

use std::clone::Clone;
pub use crate::server_types::SocketId;

// Server struct.

pub struct Server {
    dummy: SocketId
}

// Constructor.

impl Server {
    pub fn new() -> Self {
        Server {
            dummy: SocketId::new();
        }
    }
}

// Public methods of Server.

impl Server {
    pub 
}

