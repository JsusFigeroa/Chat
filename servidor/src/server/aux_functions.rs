use crate::server::server_mesagges::{generate_not_identified_msg, generate_succes_identify_response, generate_not_valid_msg, generate_user_already_exists_response};
use crate::type_recive_messages::TypeReciveMessages;
use crate::type_send_messages::TypeSendMessages;
use std::sync::mpsc::{Receiver as Receptor, Sender as Senderr, self};
use crate::user::User;
use tokio::io::AsyncRead;
use::tokio::io::{BufReader, AsyncBufReadExt, AsyncWriteExt};
use super::*;

//Función que regresa un Result<>, Ok si se logro autenticar o Err en otro caso.
//Si es Ok devuelve el nombre de usuario para poder agregarlo a la lista de usuarios.
pub(super) async fn retry_identify<T: Unpin + AsyncRead>(reader: &mut FramedRead<T, LinesCodec>) -> Result<String, ()> {
    let Ok(msg) = reader.next().await.unwrap() else {
        return Err(())
    };
    let mensagge: TypeReciveMessages;
    match serde_json::from_str(&msg) {
        Ok(text) => {
            mensagge = text;
        }
        Err(_) => {
            return Err(());
        }
    }
    if let TypeReciveMessages::Identify { type_msg, username } = mensagge {

        if type_msg != "IDENTIFY" {
            return Err(());
        }
        else {
            return Ok(username)
        }
    }
    else {
        return Err(());
    }
}



