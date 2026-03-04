use crate::user::{State, User};
use serde::{Deserialize, Serialize};
use serde_json::Map;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum TypeReciveMesagges {
    Identify {
        #[serde(rename = "type")]
        type_msg: String,
        username: String,
    },
    NewUser {
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
