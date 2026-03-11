use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum TypeReciveMessages {
    Identify {
        #[serde(rename = "type")]
        type_msg: String,
        username: String,
    },
    Status {
        #[serde(rename = "type")]
        type_msg: String,
        status: String,
    },
    Users {
        #[serde(rename = "type")]
        type_msg: String,
    },
    TextFrom {
        #[serde(rename = "type")]
        type_msg: String,
        username: String,
        text: String,
    },
    PublicText {
        #[serde(rename = "type")]
        type_msg: String,
        text: String,
    },
    RoomMesagge {
        #[serde(rename = "type")]
        type_msg: String,
        roomname: String,
    },
    Invitation {
        #[serde(rename = "type")]
        type_msg: String,
        roomname: String,
        usernames: String,
    },
    JoinRoom {
        #[serde(rename = "type")]
        type_msg: String,
        roomname: String,
    },
    RoomText {
        #[serde(rename = "type")]
        type_msg: String,
        roomname: String,
        text: String,
    },
    Disconect {
        #[serde(rename = "type")]
        type_msg: String,
    }
}

/// Esta función toma un String y verifica si corresponde a uno de los mensajes que recibe el servidor.
/// En caso positivo devuelve Ok con una instancia de la enumeración y en caso negativo devuelve un error nulo.
impl TypeReciveMessages {
    pub fn get_structured_message(msg: &str) -> Result<TypeReciveMessages, ()> {
        let Ok(message) = serde_json::from_str(&msg) else {
            return Err(());
        };
        Ok(message)
    }
}
