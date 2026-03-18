use std::collections::HashMap;
use std::str;

use serde_json;
use crate::{type_send_messages::TypeSendMessages};
use crate::user::State;

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
pub(super) fn generate_disconected_msg(username: &str) -> Result<Vec<u8>, serde_json::Error> {
    let msg = TypeSendMessages::IdentifyOrDisconect { type_msg: String::from("DISCONNECTED"), username: String::from(username) };
    let mut message = serde_json::to_vec(&msg)?;
    message.push(b'\n');
    Ok(message)
}

pub(super) fn generate_public_text_from(username: &str, text: String) -> Vec<u8>{
    let message = TypeSendMessages::TextFrom { type_msg: String::from("PUBLIC_TEXT_FROM"),
                                                                                        username: String::from(username),
                                                                                        text };
    let mut msg = serde_json::to_vec(&message).expect("No fue posible generar el mensaje");
    msg.push(b'\n');
    msg
} 

pub(super) fn generate_new_status_msg(username: &str, status: State) -> Vec<u8>{
    let message = TypeSendMessages::NewStatus { type_msg: String::from("NEW_STATUS"), username: String::from(username), status };
    let mut msg = serde_json::to_vec(&message).expect("No se pudo generar el mensaje de nuevo status");
    msg.push(b'\n');
    msg
}

pub(super) fn generate_users_msg(map: HashMap<String, State>) -> Vec<u8>{
    let message = TypeSendMessages::GiveUsers { type_msg: String::from("USER_LIST"), users: map };
    let mut msg = serde_json::to_vec(&message).expect("Error al parsear el mapa");
    msg.push(b'\n');
    msg
} 

pub(super) fn generate_text_from_msg(username: String, text: String) -> Vec<u8>{
    let message = TypeSendMessages::TextFrom { type_msg: String::from("TEXT_FROM"), username, text };
    let mut msg = serde_json::to_vec(&message).expect("Error al generar json para mensaje privado");
    msg.push(b'\n');
    msg
}

pub(super) fn generate_user_not_exist_response(username: String) -> Vec<u8>{
    let message = TypeSendMessages::Response { type_msg: String::from("RESPONSE") ,
                                                                 operation: String::from("TEXT"),
                                                                 result: String::from("NO_SUCH_USER"),
                                                                 extra: username };
    let mut msg = serde_json::to_vec(&message).expect("Error al generar mensaje de usuario no existe");
    msg.push(b'\n');
    msg
}

pub(super) fn generate_new_user_msg(username: String) -> Vec<u8> {
    let message = TypeSendMessages::IdentifyOrDisconect { type_msg: String::from("NEW_USER"), username };
    let mut msg = serde_json::to_vec(&message).expect("Error al generar mensaje de nuevo usuario");
    msg.push(b'\n');
    msg
}

pub(crate) fn generate_invitation_msg(username: &str, roomname: &str) -> Vec<u8>{
    unimplemented!();
}

pub(crate) fn generate_not_invitated_msg(roomname: &str) -> Vec<u8> {
    unimplemented!();
}

pub(crate) fn generate_new_room_user_msg(roomname: &str, new_user: &str) -> Vec<u8> {
    unimplemented!();
}

pub(crate) fn generate_room_users_msg(roomname: &str, users_map: HashMap<String, State>) -> Vec<u8> {
    unimplemented!();
}

pub(crate) fn generate_not_joined_response(username: &str, roomname: &str) -> Vec<u8>{
    unimplemented!();
}

pub(crate) fn generate_user_leaved_room(username: &str, roomname: &str) -> Vec<u8> {
    unimplemented!();
}

pub(crate) fn no_such_room_invite_msg(roomname: &str ) -> Vec<u8> {
    unimplemented!();
}

pub(crate) fn no_such_user_invite_msg(username: &str) -> Vec<u8> {
    unimplemented!();
}

pub(crate) fn no_such_room_join_room_msg(roomname: &str) -> Vec<u8> {
    unimplemented!();
}

//Genera la respuesta para cuando un grupo se crea exitosamente
pub(crate) fn new_room_success(roomname: &str) -> Vec<u8> {
    unimplemented!();
}

pub(crate) fn leave_room_not_such_room(roomname: &str) -> Vec<u8> {
    unimplemented!();
}

pub(crate) fn room_text_no_such_room(roomname: &str) -> Vec<u8> {
    unimplemented!();
}

pub(crate) fn room_users_not_joined(roomname: &str) -> Vec<u8> {
    unimplemented!();
}

pub(crate) fn room_users_no_such_room(roomname: &str) -> Vec<u8> {
    unimplemented!();
}