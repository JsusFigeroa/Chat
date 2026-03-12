use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::view::Status;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub(crate)  enum TypeReciveMesagges {
    #[serde(rename = "RESPONSE")]
    Response {
        operation: OperationType,
        result: String,
        extra: String,
    },
    #[serde(rename = "NEW_USER")]
    NewUser {
        username: String
    },
    #[serde(rename = "NEW_STATUS")]
    NewStatus {
        username: String,
        status: Status,
    },
    #[serde(rename = "USER_LIST")]
    UserList {
        users: HashMap<String, Status>
    },
    #[serde(rename = "TEXT_FROM")]
    TextFrom {
        username: String,
        text: String,
    },
    #[serde(rename = "PUBLIC_TEXT_FROM")]
    PublicTextFrom {
        username: String,
        text: String,
    },
    #[serde(rename = "DISCONNECTED")]
    Disconnected {
        username: String,
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub(crate) enum OperationType {
    #[serde(rename = "IDENTIFY")]
    Identify,
    #[serde(rename = "TEXT")]
    Text,
    #[serde(rename = "INVALID")]
    Invalid,
}

