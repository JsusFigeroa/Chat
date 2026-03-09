use serde_json;
use crate::{type_send_messages::TypeSendMessages};

pub(super) fn generate_not_identified_msg() -> Result<Vec<u8>, serde_json::Error> {
    let msg = TypeSendMessages::Invalid { type_msg: String::from("RESPONSE"),
                                                  operation: String::from("INVALID"),
                                                  result: String::from("NOT_IDENTIFIED") };
    let mut msg_to_bytes = serde_json::to_vec(&msg)?;
    msg_to_bytes.push(b'\n');
    Ok(msg_to_bytes)
}

pub(super) fn generate_not_valid_msg() -> Result<Vec<u8>, serde_json::Error> {
    let msg = TypeSendMessages::Invalid { type_msg: String::from("RESPONSE"),
                                                            operation: String::from("INVALID"), 
                                                            result: String::from("INVALID") };
    let mut msg_to_bytes = serde_json::to_vec(&msg)?;
    msg_to_bytes.push(b'\n');
    Ok(msg_to_bytes)
}

pub(super) fn generate_succes_identify_response(name: &str) -> Result<Vec<u8>, serde_json::Error> {
    let msg = TypeSendMessages::Response { type_msg: String::from("RESPONSE"),
                                                             operation: String::from("IDENTIFY"),
                                                              result: String::from("SUCCESS"),
                                                               extra: String::from(name) };
    let mut msg_to_bytes = serde_json::to_vec(&msg)?;
    msg_to_bytes.push(b'\n');
    Ok(msg_to_bytes)
}

pub(super) fn generate_user_already_exists_response(name: &str) -> Result<Vec<u8>, serde_json::Error> {
    let msg = TypeSendMessages::Response { type_msg: String::from("RESPONSE"),
                                                             operation: String::from("IDENTIFY"),
                                                              result: String::from("USER_ALREADY_EXISTS"),
                                                               extra: String::from(name) };
    let mut msg_to_bytes = serde_json::to_vec(&msg)?;
    msg_to_bytes.push(b'\n');
    Ok(msg_to_bytes)
} 
pub(super) fn generate_disconected_msg(username: &str) -> Result<String, serde_json::Error> {
    let msg = TypeSendMessages::IdentifyOrDisconect { type_msg: String::from("DISCONNECTED"), username: String::from(username) };
    let message = serde_json::to_string(&msg)?;
    Ok(message)
}