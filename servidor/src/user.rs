use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::Sender;

#[derive(Clone)]
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

