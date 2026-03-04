use crate::user::{State, User};
use serde::{Deserialize, Serialize};
use serde_json::Map;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum TypeSendMesagges {
    Response {
        #[serde(rename = "type")]
        type_msg: String,
        operation: String,
        result: String,
        extra: String,
    },
    NewStatus {
        #[serde(rename = "type")]
        type_msg: String,
        username: String,
        status: State,
    },
    GiveUsers {
        #[serde(rename = "type")]
        type_msg: String,
        users: HashMap<String, State>,
    },
    TextFrom {
        #[serde(rename = "type")]
        type_msg: String,
        username: String,
        text: String,
    },
    Invitation {
        #[serde(rename = "type")]
        type_msg: String,
        usernamme: String,
        roomname: String,
    },
    RoomOperations {
        #[serde(rename = "type")]
        type_msg: String,
        roomname: String,
        username: String,
    },
    RoomUsers {
        #[serde(rename = "type")]
        type_msg: String,
        roomname: String,
        users: HashMap<String, State>,
    },
    IdentifyOrDisconect {
        #[serde(rename = "type")]
        type_msg: String,
        username: String,
    },
    UsersList {
        #[serde(rename = "type")]
        type_msg: String,
        users: HashMap<String, State>
    },
    RoomText {
        #[serde(rename = "type")]
        type_msg: String,
        roomname: String,
        username: String,
        text: String
    },
    Invalid {
        #[serde(rename = "type")]
        type_msg: String,
        operation: String,
        result: String
    }
}

impl TypeSendMesagges {
    
}