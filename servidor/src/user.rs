use serde::{Deserialize, Serialize};
use tokio::io::{BufReader, BufWriter};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use std::sync::mpsc::Sender;

pub struct User {
    pub name: String,
    pub tx : Sender<Vec<u8>>,
    pub state: State,
}
impl User {
    pub fn new(name: String, tx: Sender<Vec<u8>>) -> User {
        let state = State::Active;
        User {
            name,
            tx,
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
