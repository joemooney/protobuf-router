//use quick_protobuf::Reader;
use quick_protobuf::deserialize_from_slice;
use quick_protobuf::serialize_into_vec;

mod error;
mod example;
mod requests;
mod routes;
pub mod udp; // example.rs protobuf generated file we generated
use crate::db::*;
pub use error::*;
pub use requests::*;
pub use routes::*;

use example::Person;
//use quick_protobuf::{
//    deserialize_from_slice, serialize_into_vec, BytesReader, MessageRead, Writer,
//};
use std::borrow::Cow;

impl Person<'static> {
    fn new() -> Person<'static> {
        Person {
            name: Cow::Borrowed("John"),
            id: 123,
            email: Some(Cow::Borrowed("john.doe@email.com")),
        }
    }
}

pub fn send_test_message() -> ProtoResult<()> {
    let message = Person::new();
    let out =
        serialize_into_vec(&message).map_err(|source| ProtoError::SerializeError { source })?;
    println!("Read serialized ok");
    udp::send_message(out)?;
    println!("Message written successfully!");
    Ok(())
}

pub fn receive_test_message(msg: &[u8]) -> ProtoResult<()> {
    let person = Person::new();
    deserialize_from_slice::<Person>(&msg)
        .map_err(|source| ProtoError::DeserializeError { source })?;
    println!("Message written deserialized! {:?}", person);
    Ok(())
}
