
use std::clone::Clone;
use std::ops::Deref;

pub struct Foo {}

impl Clone for Foo {
    fn clone(&self) -> Self {
        Foo {}
    }
}

impl Deref for Foo {}

pub fn main() {
    let f = Foo {};
    let _ = f.clone();
}

