use std::sync::Mutex;
// use std::{collections::HashMap, process::Child};

/// A simple in-memory DB to store logging state
pub type Db = Mutex<ServerState>;
#[derive(Debug)]
pub struct ServerState {
    pub call_count: u32,
}

impl ServerState {
    pub fn new() -> ServerState {
        ServerState { call_count: 0 }
    }
}
