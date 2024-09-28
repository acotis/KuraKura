
use std::clone::Clone;
use std::ops::Deref;

pub struct Foo {}

impl Clone for Foo {
    fn clone(&self) -> Self {
        Foo {}
    }
}

impl Deref<str> for Foo {
    fn deref(&self) -> &str {
        "Hello world"
    }
}

pub fn main() {
    let f = Foo {};
    let _ = f.clone();
}

