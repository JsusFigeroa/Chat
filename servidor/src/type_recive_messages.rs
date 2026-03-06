use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum TypeReciveMessages {
    Identify {
        #[serde(rename = "type")]
        type_msg: String,
        username: String,
    },
    Status {
        #[serde(rename = "type")]
        type_msg: String,
        status: String,
    },
    Users {
        #[serde(rename = "type")]
        type_msg: String,
    },
    TextFrom {
        #[serde(rename = "type")]
        type_msg: String,
        username: String,
        text: String,
    },
    PublicText {
        #[serde(rename = "type")]
        type_msg: String,
        text: String,
    },
    RoomMesagge {
        #[serde(rename = "type")]
        type_msg: String,
        roomname: String,
    },
    Invitation {
        #[serde(rename = "type")]
        type_msg: String,
        roomname: String,
        usernames: String,
    },
    JoinRoom {
        #[serde(rename = "type")]
        type_msg: String,
        roomname: String,
    },
    RoomText {
        #[serde(rename = "type")]
        type_msg: String,
        roomname: String,
        text: String,
    },
    Disconect {
        #[serde(rename = "type")]
        type_msg: String,
    }
}
