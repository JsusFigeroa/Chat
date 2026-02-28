use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum TypeReciveMesagge {
    Identify {
        #[serde(rename = "type")]
        typ_msg: String,
        username: String,
    },
    Status {
        #[serde(rename = "type")]
        typ_msg: String,
        username: String,
        status: String,
    },
    UsersList {
        #[serde(rename = "type")]
        typ_msg: String,
        users: HashMap<String, String>,
    },
    TextFrom {
        #[serde(rename = "type")]
        typ_msg: String,
        username: String,
        text: String,
    },
    PublicTextFrom {
        #[serde(rename = "type")]
        typ_msg: String,
        username: String,
        text: String,
    },
    JoinedRoom {
        #[serde(rename = "type")]
        typ_msg: String,
        roomname: String,
        username: String,
    },
    RoomUserList {
        #[serde(rename = "type")]
        typ_msg: String,
        roomname: String,
        users: HashMap<String, String>,
    },
    RoomTextFrom {
        #[serde(rename = "type")]
        typ_msg: String,
        roomname: String,
        username: String,
        text: String,
    },
    LeftRoom {
        #[serde(rename = "type")]
        typ_msg: String,
        roomname: String,
        username: String,
    },
    Disconected {
        #[serde(rename = "type")]
        typ_msg: String,
        username: String,
    }

}