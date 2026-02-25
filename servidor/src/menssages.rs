use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use serde_json::Map;
use crate::usuario::{Usuario, State};

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum menssages{

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
    }

}