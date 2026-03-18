use crate::user::{State};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum TypeSendMessages {
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

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "operation")]
pub(crate) enum Operations {
    #[serde(rename = "INVITE")]
    Invite,
    #[serde(rename = "JOIN_ROOM")]
    JoinRoom,
    #[serde(rename = "ROOM_USERS")]
    RoomUsers,
    #[serde(rename = "ROOM_TEXT")]
    RoomText,
    #[serde(rename = "LEAVE:_ROOM")]
    LeaveRoom,
    #[serde(rename = "IDENTIFY")]
    Identify,
    #[serde(rename = "TEXT")]
    Text,
    #[serde(rename = "NEW_ROOM")]
    NewRoom,
    #[serde(rename = "INVALID")]
    Invalid
}

