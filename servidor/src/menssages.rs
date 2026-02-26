use crate::user::{State, User};
use serde::{Deserialize, Serialize};
use serde_json::Map;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Menssages {
    Identify {
        #[serde(rename = "type")]
        type_msg: String,
        username: String,
    },
    Response {
        #[serde(rename = "type")]
        type_msg: String,
        operation: String,
        result: String,
        extra: String,
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
    NewStatus {
        #[serde(rename = "type")]
        type_msg: String,
        username: String,
        status: String,
    },
    Users {
        #[serde(rename = "type")]
        type_msg: String,
    },
    GiveUsers {
        #[serde(rename = "type")]
        type_msg: String,
        users: HashMap<String, State>,
    },
}
