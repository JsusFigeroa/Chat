use futures::future::ok;
use serde::{Deserialize, Serialize};
use tokio::io::{BufReader, BufWriter};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use std::sync::mpsc::Sender;
use serde_json;

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

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "UPPERCASE")]
pub enum State {
    Busy,
    Away,
    Active,
}

impl State {
    pub(crate) fn get_from_str(state: &str) -> Result<State, ()> {
        if state == "AWAY" {
            Ok(State::Away)
        }
        else if state == "BUSY" {
            Ok(State::Busy)
        }
        else if state == "ACTIVE" {
            Ok(State::Active)
        }
        else {
            Err(())
        }
    }
}
