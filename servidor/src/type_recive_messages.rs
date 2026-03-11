use serde::{Deserialize, Serialize};

use crate::user::State;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum TypeReciveMessages {
    #[serde(rename = "IDENTIFY")]
    Identify {
        username: String,
    },
    #[serde(rename = "STATUS")]
    Status {
        status: State
    },
    #[serde(rename = "USERS")]
    Users,
    #[serde(rename = "TEXT")]
    Text {
        username: String,
        text: String,
    },
    #[serde(rename = "PUBLIC_TEXT")]
    PublicText {
        text: String,
    },
    #[serde(rename = "DISCONNECT")]
    Disconect,
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
