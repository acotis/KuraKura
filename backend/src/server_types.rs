
use std::marker::PhantomData;
use std::hash::{Hash, Hasher};
use std::ops::Deref;
use uuid::Uuid;

pub struct Id {
    id: [u8; 36],
}

impl Copy for Id {}

impl Clone for Id {
    fn clone(&self) -> Self {
        *self
    }
}

impl Deref<str> for Id {
    fn deref(&self) -> &str {
        unsafe {&self.id}
    }
}

