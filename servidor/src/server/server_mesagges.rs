use std::collections::HashMap;
use std::str;
use  crate::type_send_messages::{Operations, Result as Resultado};
use serde_json;
use crate::{type_send_messages::TypeSendMessages};
use crate::user::State;

pub(super) fn generate_not_identified_msg() -> Vec<u8> {
    let msg = TypeSendMessages::Response { operation: Operations::Invalid, result: Resultado::NotIdentified, extra: None };
    let mut msg_to_bytes = serde_json::to_vec(&msg).expect("No fue posible generar el mensaje de identificación");
    msg_to_bytes.push(b'\n');
    msg_to_bytes
}

pub(super) fn generate_not_valid_msg() -> Vec<u8> {
    let msg = TypeSendMessages::Response { operation: Operations::Invalid, result: Resultado::Invalid, extra: None };
    let mut msg_to_bytes = serde_json::to_vec(&msg).unwrap();
    msg_to_bytes.push(b'\n');
    msg_to_bytes
}

pub(super) fn generate_text_user_not_exist(username: &str) -> Vec<u8> {
    let message = TypeSendMessages::Response { operation: Operations::Text, result: Resultado::NoSuchUser, extra: Some(username.to_string()) };
    let mut msg = serde_json::to_vec(&message).unwrap();
    msg.push(b'\n');
    msg
}

pub(super) fn generate_succes_identify_response(name: &str) -> Vec<u8> {
    let msg = TypeSendMessages::Response { operation: Operations::Identify, result: Resultado::Success, extra: Some(name.to_string()) };
    // let msg = TypeSendMessages::Response { type_msg: String::from("RESPONSE"),
    //                                                          operation: String::from("IDENTIFY"),
    //                                                           result: String::from("SUCCESS"),
    //                                                            extra: String::from(name) };
    let mut msg_to_bytes = serde_json::to_vec(&msg).unwrap();
    msg_to_bytes.push(b'\n');
    msg_to_bytes
}

pub(super) fn generate_user_already_exists_response(name: &str) -> Vec<u8> {
    let msg = TypeSendMessages::Response { operation: Operations::Identify, result: Resultado::UserAlreadyExists, extra: Some(name.to_string()) };
    let mut msg_to_bytes = serde_json::to_vec(&msg).unwrap();
    msg_to_bytes.push(b'\n');
    msg_to_bytes
} 
pub(super) fn generate_disconected_msg(username: &str) -> Vec<u8> {
    let msg = TypeSendMessages::Disconnected { username: username.to_string() };
    let mut message = serde_json::to_vec(&msg).unwrap();
    message.push(b'\n');
    message
}

pub(super) fn generate_public_text_from(username: &str, text: &str) -> Vec<u8> {
    let msg = TypeSendMessages::PublicTextFrom { username: username.to_string(), text: text.to_string() };
    let mut msg_to_bytes = serde_json::to_vec(&msg).unwrap();
    msg_to_bytes.push(b'\n');
    msg_to_bytes
} 

pub(super) fn generate_new_status_msg(username: &str, status: State) -> Vec<u8> {
    let message = TypeSendMessages::NewStatus { username: username.to_string(), status };
    let mut msg = serde_json::to_vec(&message).expect("No se pudo generar el mensaje de nuevo status");
    msg.push(b'\n');
    msg
}

pub(super) fn generate_users_msg(map: HashMap<String, State>) -> Vec<u8> {
    let message = TypeSendMessages::UserList { users: map };
    let mut msg = serde_json::to_vec(&message).expect("Error al parsear el mapa");
    msg.push(b'\n');
    msg
} 

pub(super) fn generate_text_from_msg(username: &str, text: &str) -> Vec<u8> {
    let message = TypeSendMessages::TextFrom { username: username.to_string() , text: text.to_string() };
    let mut msg = serde_json::to_vec(&message).expect("Error al generar json para mensaje privado");
    msg.push(b'\n');
    msg
}

pub(super) fn generate_text_form_user_not_exist_response(username: &str) -> Vec<u8> {
    let message = TypeSendMessages::Response { operation: Operations::Text, result: Resultado::NoSuchUser, extra:  Some(username.to_string()) };
    // let message = TypeSendMessages::Response { type_msg: String::from("RESPONSE") ,
    //                                                              operation: String::from("TEXT"),
    //                                                              result: String::from("NO_SUCH_USER"),
    //                                                              extra: username };
    let mut msg = serde_json::to_vec(&message).expect("Error al generar mensaje de usuario no existe");
    msg.push(b'\n');
    msg
}

pub(super) fn generate_new_user_msg(username: &str) -> Vec<u8> {
    let message = TypeSendMessages::NewUser { username: username.to_string() };
    let mut msg = serde_json::to_vec(&message).expect("Error al generar mensaje de nuevo usuario");
    msg.push(b'\n');
    msg
}

pub(crate) fn generate_invitation_msg(username: &str, roomname: &str) -> Vec<u8> {
    let message = TypeSendMessages::Invitation { username: username.to_string(), roomname: roomname.to_string() };
    let mut msg = serde_json::to_vec(&message).unwrap();
    msg.push(b'\n');
    msg
}

pub(crate) fn generate_not_invitated_msg(roomname: &str) -> Vec<u8> {
    let message = TypeSendMessages::Response { operation: Operations::JoinRoom, result: Resultado::NotInvited, extra: Some(roomname.to_string()) };
    let mut msg = serde_json::to_vec(&message).unwrap();
    msg.push(b'\n');
    msg
}

pub(crate) fn success_join_room_response(roomname: &str) -> Vec<u8> {
    let message = TypeSendMessages::Response { operation: Operations::JoinRoom, result: Resultado::Success , extra: Some(roomname.to_string()) };
    let mut msg = serde_json::to_vec(&message).unwrap();
    msg.push(b'\n');
    msg
}

pub(crate) fn generate_new_room_user_msg(roomname: &str, new_user: &str) -> Vec<u8> {
    let message = TypeSendMessages::JoinedRoom { roomname: roomname.to_string(), username: new_user.to_string() };
    let mut msg = serde_json::to_vec(&message).unwrap();
    msg.push(b'\n');
    msg
}

pub(crate) fn generate_room_users_msg(roomname: &str, users_map: HashMap<String, State>) -> Vec<u8> {
    let message = TypeSendMessages::RoomUserList { roomname: roomname.to_string(), users: users_map };
    let mut msg = serde_json::to_vec(&message).unwrap();
    msg.push(b'\n');
    msg
}

pub(crate) fn generate_not_joined_response(roomname: &str) -> Vec<u8>{
    let message = TypeSendMessages::Response { operation: Operations::RoomText, result: Resultado::NotJoined, extra: Some(roomname.to_string()) };
    let mut msg = serde_json::to_vec(&message).unwrap();
    msg.push(b'\n');
    msg
}

pub(crate) fn generate_user_leaved_room(username: &str, roomname: &str) -> Vec<u8> {
    let message = TypeSendMessages::LeftRoom { roomname: roomname.to_string(), username: username.to_string() };
    let mut msg = serde_json::to_vec(&message).unwrap();
    msg.push(b'\n');
    msg
}

pub(crate) fn no_such_room_invite_msg(roomname: &str ) -> Vec<u8> {
    let message = TypeSendMessages::Response { operation: Operations::JoinRoom, result: Resultado::NoSuchRoom, extra: Some(roomname.to_string()) };
    let mut msg = serde_json::to_vec(&message).unwrap();
    msg.push(b'\n');
    msg
}

pub(crate) fn no_such_user_invite_msg(username: &str) -> Vec<u8> {
    let message = TypeSendMessages::Response { operation: Operations::Invite, result: Resultado::NoSuchUser, extra: Some(username.to_string()) };
    let mut msg = serde_json::to_vec(&message).unwrap();
    msg.push(b'\n');
    msg
}

pub(crate) fn no_such_room_join_room_msg(roomname: &str) -> Vec<u8> {
    let message = TypeSendMessages::Response { operation: Operations::JoinRoom, result: Resultado::NoSuchRoom, extra: Some(roomname.to_string()) };
    let mut msg = serde_json::to_vec(&message).unwrap();
    msg.push(b'\n');
    msg
}

//Genera la respuesta para cuando un grupo se crea exitosamente
pub(crate) fn new_room_success(roomname: &str) -> Vec<u8> {
    let  message = TypeSendMessages::Response { operation: Operations::NewRoom, result: Resultado::Success, extra: Some(roomname.to_string()) };
    let mut msg = serde_json::to_vec(&message).unwrap();
    msg.push(b'\n');
    msg
}

pub(crate) fn leave_room_not_such_room(roomname: &str) -> Vec<u8> {
    let message = TypeSendMessages::Response { operation: Operations::LeaveRoom, result: Resultado::NoSuchRoom, extra: Some(roomname.to_string()) };
    let mut msg = serde_json::to_vec(&message).unwrap();
    msg.push(b'\n');
    msg
}

pub(crate) fn room_text_no_such_room(roomname: &str) -> Vec<u8> {
    let message = TypeSendMessages::Response { operation: Operations::RoomText, result: Resultado::NoSuchRoom, extra: Some(roomname.to_string()) };
    let mut msg = serde_json::to_vec(&message).unwrap();
    msg.push(b'\n');
    msg
}

pub(crate) fn room_users_not_joined(roomname: &str) -> Vec<u8> {
    let message = TypeSendMessages::Response { operation: Operations::RoomUsers, result: Resultado::NotJoined, extra: Some(roomname.to_string()) };
    let mut msg = serde_json::to_vec(&message).unwrap();
    msg.push(b'n');
    msg
}

pub(crate) fn room_users_no_such_room(roomname: &str) -> Vec<u8> {
    let message = TypeSendMessages::Response { operation: Operations::RoomUsers, result: Resultado::NoSuchRoom, extra: Some(roomname.to_string()) };
    let mut msg = serde_json::to_vec(&message).unwrap();
    msg.push(b'\n');
    msg
}

pub(crate) fn generate_not_joined_leave_room_response(roomname: &str) -> Vec<u8> {
    let message = TypeSendMessages::Response { operation: Operations::LeaveRoom, result: Resultado::NotJoined, extra: Some(roomname.to_string()) };
    let mut msg = serde_json::to_vec(&message).unwrap();
    msg.push(b'\n');
    msg
}

pub(crate) fn generate_room_text_from(username: &str, roomname: &str, text: &str) -> Vec<u8> {
    let message = TypeSendMessages::RoomTextFrom { roomname: roomname.to_string(), username: username.to_string(), text: text.to_string() };
    let mut msg = serde_json::to_vec(&message).unwrap();
    msg.push(b'\n');
    msg
}