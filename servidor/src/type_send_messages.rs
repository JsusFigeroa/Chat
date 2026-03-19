use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::user::State;

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum TypeSendMessages {
    #[serde(rename = "RESPONSE")]
    Response {
        operation: Operations,
        result: Result,
        #[serde(skip_serializing_if = "Option::is_none")]
        extra: Option<String>,
    },
    #[serde(rename = "NEW_USER")]
    NewUser {
        username: String
    },
    #[serde(rename = "NEW_STATUS")]
    NewStatus {
        username: String,
        status: State
    },
    #[serde(rename = "USER_LIST")]
    UserList {
        users: HashMap<String, State>
    },
    #[serde(rename = "TEXT_FROM")]
    TextFrom {
        username: String,
        text: String,
    },
    #[serde(rename = "PUBLIC_TEXT_FROM")]
    PublicTextFrom {
        username: String,
        text: String
    },
    #[serde(rename = "INVITATION")]
    Invitation {
        username: String,
        roomname: String,
    },
    #[serde(rename = "JOINED_ROOM")]
    JoinedRoom {
        roomname: String,
        username: String
    },
    #[serde(rename = "ROOM_USER_LIST")]
    RoomUserList {
        roomname: String,
        users: HashMap<String, State>
    },
    #[serde(rename = "ROOM_TEXT_FROM")]
    RoomTextFrom {
        roomname: String,
        username: String,
        text: String
    },
    #[serde(rename = "LEFT_ROOM")]
    LeftRoom {
        roomname: String,
        username: String
    },
    #[serde(rename = "DISCONNECTED")]
    Disconnected {
        username: String
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub(crate) enum Operations {
    #[serde(rename = "INVITE")]
    Invite,
    #[serde(rename = "JOIN_ROOM")]
    JoinRoom,
    #[serde(rename = "ROOM_USERS")]
    RoomUsers,
    #[serde(rename = "ROOM_TEXT")]
    RoomText,
    #[serde(rename = "LEAVE_ROOM")]
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

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub(crate) enum Result {
    #[serde(rename = "SUCCESS")]
    Success,
    #[serde(rename = "USER_ALREADY_EXISTS")]
    UserAlreadyExists,
    #[serde(rename = "NO_SUCH_USER")]
    NoSuchUser,
    #[serde(rename = "ROOM_ALREADY_EXISTS")]
    RoomAlreadyExists,
    #[serde(rename = "NO_SUCH_ROOM")]
    NoSuchRoom,
    #[serde(rename = "NOT_INVITED")]
    NotInvited,
    #[serde(rename = "NOT_JOINED")]
    NotJoined,
    #[serde(rename = "NOT_IDENTIFIED")]
    NotIdentified,
    #[serde(rename = "INVALID")]
    Invalid
}

