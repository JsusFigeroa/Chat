use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum TypeSendMesagges {
    Identify {
        #[serde(rename = "type")]
        typ_msg: String,
        username: String,
    },
    Status {
        #[serde(rename = "type")]
        typ_msg: String,
        status: String,
    },
    Users {
        #[serde(rename = "type")]
        typ_msg: String,
    },
    Text {
        #[serde(rename = "type")]
        typ_msg: String,
        username: String,
        text: String,
    },
    PublicText {
        #[serde(rename = "type")]
        typ_msg: String,
        text: String,
    },
    NewRoom {
        #[serde(rename = "type")]
        typ_msg: String,
        roomname: String,
    },
    Invite {
        #[serde(rename = "type")]
        typ_msg: String,
        roomname: String,
        usernames: Vec<String>,
    },
    JoinRoom {
        #[serde(rename = "type")]
        typ_msg: String,
        roomname: String,
    },
    RoomUsers {
        #[serde(rename = "type")]
        typ_msg: String,
        roomname: String,
    },
    RoomText {
        #[serde(rename = "type")]
        typ_msg: String,
        roomname: String,
        text: String,
    },
    LeaveRoom {
        #[serde(rename = "type")]
        typ_msg: String,
        roomname: String,
    },
    Disconect {
        #[serde(rename = "type")]
        typ_msg: String,
    }
}