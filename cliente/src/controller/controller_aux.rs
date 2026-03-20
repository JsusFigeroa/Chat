use crate::{type_send_message::TypeSendMessage};


pub(super) fn generate_identify(name: String) -> Vec<u8> {
    let identify = TypeSendMessage::Identify { 
        username: name 
    };
    let mut msg = serde_json::to_vec(&identify).expect("Error al serializar el mensaje");
    msg.push(b'\n');
    msg
}

