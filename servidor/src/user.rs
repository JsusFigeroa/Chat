use serde::ser::{SerializeMap, SerializeStruct, Serializer};
use serde::{Deserialize, Serialize};
use tokio::io::BufReader;
use tokio::net::TcpStream;

pub struct User {
    pub name: String,
    socket: BufReader<TcpStream>,
    state: State,
}
impl User {
    pub fn new(name: String, socket: BufReader<TcpStream>) -> User {
        let state = State::Active;
        User {
            name,
            socket,
            state,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub enum State {
    Busy,
    Away,
    Active,
}
