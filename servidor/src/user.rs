use tokio::sync::Mutex;
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::Sender;


pub struct User {
    pub name: String,
    pub tx : Sender<Vec<u8>>,
    pub state: Mutex<State>,
    pub rooms_keys: Mutex<Vec<String>>,
    pub invitations_room_keys: Mutex<Vec<String>>
}
impl User {
    pub(crate) fn new(name: String, tx: Sender<Vec<u8>>) -> User {
        let state = Mutex::new(State::Active);
        let rooms_keys = Mutex::new(Vec::new());
        let invitations_room_keys = Mutex::new(Vec::new());
        User {
            name,
            tx,
            state,
            rooms_keys,
            invitations_room_keys
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

