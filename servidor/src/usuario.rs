use std::net::TcpStream;
use serde::{Serialize, Deserialize};
use serde::ser::{SerializeMap, SerializeStruct, Serializer};


pub struct Usuario {
    name: String,
    socket: TcpStream,
    state: State,
}
impl Usuario {
    pub fn new(name: String, id: usize, socket: TcpStream) -> Usuario{
        let state = State::Active;
        Usuario {name, socket, state}
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub enum State {
    Busy, 
    Away,
    Active,
}

