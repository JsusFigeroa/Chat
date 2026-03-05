use serde::{Deserialize, Serialize};
use tokio::io::{BufReader, BufWriter};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};

pub struct User {
    pub name: String,
    reader: BufReader<OwnedReadHalf>,
    writer: BufWriter<OwnedWriteHalf>,
    state: State,
}
impl User {
    pub fn new(name: String, reader: BufReader<OwnedReadHalf>, writer: BufWriter<OwnedWriteHalf>) -> User {
        let state = State::Active;
        User {
            name,
            reader,
            writer,
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
