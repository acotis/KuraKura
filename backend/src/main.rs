
use std::clone::Clone;
use std::ops::Deref;

pub struct Foo {
    pub field: u32,
}

impl Clone for Foo {
    fn clone(&self) -> Self {
        Foo {
            field: self.field
        }
    }
}

impl Deref<str> for Foo {
    fn deref(&self) -> &str {
        "Hello world"
    }
}

pub fn main() {
    let f = Foo {field: 1};
    let _ = f.clone();
}

