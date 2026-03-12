use serde::{Deserialize, Serialize};

use crate::view::Status;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub(crate)  enum TypeSendMessage {
    #[serde(rename = "IDENTIFY")]
    Identify {
        username: String,
    },
    #[serde(rename = "STATUS")]
    Status {
        status: Status,
    },
    #[serde(rename = "USERS")]
    Users,
    #[serde(rename = "TEXT")]
    Text {
        username: String,
        text: String
    },
    #[serde(rename = "PUBLIC_TEXT")]
    PublicText {
        text: String
    },
    #[serde(rename = "DISCONNECT")]
    DISCONNECT
}

