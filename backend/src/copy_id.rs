
use std::marker::PhantomData;
use std::hash::{Hash, Hasher};
use std::fmt;
use std::fmt::Debug;
use std::fmt::Formatter;
use serde::{Serialize, Serializer, Deserialize, Deserializer};
use serde::de::Visitor;
use serde::de;
use std::ops::Deref;
use uuid::Uuid;
use crate::Game;

// The "ID" struct. Used to create the AccountId, RoomId, and SocketId types. These
// three types are identical, but we want the compiler to recognize them as distinct
// and not let us use them interchangeably. To do that, we give the Id struct a
// phantom type parameter <T>. The AccountId type is Id<Account>, the RoomId type
// is Id<Room> and the SocketId type is Id<Socket>. The phantom type parameter
// requires us to implement the trais of this type automatically, because of this
// bug (!) in the #[derive] macro: https://github.com/rust-lang/rust/issues/26925
//
// The Id simply holds an array of thirt-six u8's which is guaranteed to comprise
// a valid str (i.e., a valid UTF-8 string). When an Id is created, it fills out
// its str using a UUID generator that generates 36-character ascii-hexadecimal
// strings (32 meaningful chars and 4 hyphens). We hold the data in a field instead
// of behind a String (for example) so that the type can be plain-old-data and
// implement Copy guilt-free. This is nice because otherwise we end up invoking
// .clone() on the Id type five thousand times in the Server implementation.

pub struct CopyId<T> {
    id: [u8; 36],
    dummy: PhantomData<T>,
}

impl<T> PartialEq for CopyId<T> {
    fn eq(&self, other: &CopyId<T>) -> bool {
        self.id == other.id
    }
}

impl<T> Eq for CopyId<T> {}

impl<T> Hash for CopyId<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl<T> Copy for CopyId<T> {}

impl<T> Clone for CopyId<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Deref for CopyId<T> {
    type Target = str;
    fn deref(&self) -> &str {
        std::str::from_utf8(&self.id).unwrap()
    }
}

// Serde stuff.

struct CopyIdVisitor<T> {
    dummy: PhantomData<T>
}

impl<'de, T> Visitor<'de> for CopyIdVisitor<T> {
    type Value = CopyId<T>;

    fn visit_string<E>(self, string: String) -> Result<Self::Value, E> where E: de::Error {
        if string.as_bytes().len() != 36 {
            Err(E::custom(format!("IDs must be 36 characters long (actual length: {}): {}", string.len(), string)))
        } else {
            Ok(CopyId {
                id: string.as_bytes().try_into().expect("unreachable"),
                dummy: PhantomData::<T>::default(),
            })
        }
    }

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a UUID like 0fe68b4f-7346-4f8b-90c4-4766770b63bd")
    }
}


impl<T> Serialize for CopyId<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        serializer.serialize_str(self.as_ref())
    }
}

impl<'de, T> Deserialize<'de> for CopyId<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        deserializer.deserialize_string(CopyIdVisitor {dummy: PhantomData::<T>::default()})
    }
}

// Formatting stuff.

impl<T> Debug for CopyId<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", self.deref())
    }
}

// Method to create a new CopyId.
// 
// TODO: make it not be all zeros.

impl<T> CopyId<T> {
    pub fn new() -> Self {
        CopyId {
            id: [0; 36],
            dummy: PhantomData::<T>::default(),
        }
    }
}

