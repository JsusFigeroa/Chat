use serde::{Deserialize, Serialize};

use crate::user::State;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum TypeReciveMessages {
    #[serde(rename = "IDENTIFY")]
    Identify { username: String },
    #[serde(rename = "STATUS")]
    Status { status: State },
    #[serde(rename = "USERS")]
    Users,
    #[serde(rename = "TEXT")]
    Text { username: String, text: String },
    #[serde(rename = "PUBLIC_TEXT")]
    PublicText { text: String },
    #[serde(rename = "NEW_ROOM")]
    NewRoom { roomname: String },
    #[serde(rename = "INVITE")]
    Invite {
        roomname: String,
        usernames: Vec<String>,
    },
    #[serde(rename = "JOIN_ROOM")]
    JoinRoom { roomname: String },
    #[serde(rename = "ROOM_USERS")]
    RoomUsers { roomname: String },
    #[serde(rename = "ROOM_TEXT")]
    RoomText { roomname: String, text: String },
    #[serde(rename = "LEAVE_ROOM")]
    LeaveRoom { roomname: String },
    #[serde(rename = "DISCONNECT")]
    Disconect,
}
