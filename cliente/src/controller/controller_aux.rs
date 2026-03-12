use crate::{type_receive_message::{OperationType, TypeReciveMesagges}, type_send_message::TypeSendMessage, view};


pub(super) fn generate_identify(name: String) -> Vec<u8> {
    let identify = TypeSendMessage::Identify { 
        username: name 
    };
    let mut msg = serde_json::to_vec(&identify).expect("Error al serializar el mensaje");
    msg.push(b'\n');
    msg
}

pub(super) fn procces_server_msg_aux(message: TypeReciveMesagges) {
    match message {
        TypeReciveMesagges::Disconnected { username } => {
            view::disconnected_by_server();
        }
        TypeReciveMesagges::NewStatus { username, status } => {
            view::print_new_status(username, status);
        }
        TypeReciveMesagges::PublicTextFrom { username, text } => {
            view::print_public_text(username, text);
        }
        TypeReciveMesagges::Response { operation: OperationType::Identify, result, extra } => {

        }
        TypeReciveMesagges::Response { operation: OperationType::Invalid, result, extra } => {
            view::print_invalid_response();
        }
        TypeReciveMesagges::Response { operation: OperationType::Text, result, extra } => {
            view::print_text_response_no_such_usr(extra);
        }
        TypeReciveMesagges::UserList { users } => {
            view::print_users(users);
        }
        TypeReciveMesagges::TextFrom { username, text } => {
            view::print_private_text(username, text);
        }
        TypeReciveMesagges::NewUser { username } => {
            view::print_new_user_connected(username);
        }
    }
}