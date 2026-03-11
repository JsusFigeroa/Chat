use crate::server::server_mesagges::{generate_not_identified_msg, generate_succes_identify_response, generate_not_valid_msg, generate_user_already_exists_response};
use crate::type_recive_messages::TypeReciveMessages;
use crate::type_send_messages::TypeSendMessages;
use std::collections::HashMap;
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

/// Esta función genera un mapa del cual cada llave corresponde a el nombre de usuario y
/// el valor es su estado actual.
pub(super) fn generate_map_users(users: Arc<DashMap<String, User>>) -> HashMap<String, State> {
    let mut map = HashMap::new();
    for kv in users.iter() {
        let state = kv.value().state;
        let username = kv.value().name.clone();
        map.insert(username, state);
    }
    map
}

pub(super) async fn procces_letter_aux(letter: Letter<Vec<u8>>, server: Arc<Server>) {
    match letter.msg {
    TypeReciveMessages::PublicText { type_msg, text } => {
        if type_msg == "PUBLIC_TEXT" {
            for kv in server.users.iter() {
                let user_tx = kv.value().tx.clone();
                let message = generate_public_text_from(&letter.usr_sender, text.clone());
                if String::from(kv.key()) == letter.usr_sender.to_lowercase() {
                    continue;
                }
                user_tx.send(message).await.unwrap();
            }
        }
        else {
            let message = generate_not_valid_msg().unwrap();
            server.users.remove(&letter.usr_sender.to_lowercase());
            letter.reply_to.send(message).await.unwrap();
        }
    }
    //Algo falla con este mensaje
    TypeReciveMessages::Status { type_msg, status } => {
        if type_msg == "STATUS" {
            let Ok(state) = State::get_from_str(&status) else {
                let message = generate_not_valid_msg().unwrap();
                server.users.remove(&letter.usr_sender.to_lowercase());
                letter.reply_to.send(message).await.unwrap();
                return;
            };
            if let Some(mut user) = server.users.get_mut(&letter.usr_sender.to_lowercase()) {
                user.state = state;
            }
            let mut transmisors = Vec::new();
            let msg = generate_new_status_msg(&letter.usr_sender, state);
            for kv in server.users.iter() {
                if String::from(kv.key()) == letter.usr_sender.to_lowercase() {
                    continue;
                }
                transmisors.push(kv.tx.clone());
            }
            for transmisor in transmisors {
                transmisor.send(msg.clone()).await.unwrap();
            }
        }   
    }
    TypeReciveMessages::Users { type_msg } => {
        if type_msg == "USERS" {
            let map = generate_map_users(server.users.clone());
            let msg = generate_users_msg(map);
            letter.reply_to.send(msg).await.unwrap();
        }
        else {
            let message = generate_not_valid_msg().unwrap();
            server.users.remove(&letter.usr_sender.to_lowercase());
            letter.reply_to.send(message).await.unwrap();
        }
    }
    TypeReciveMessages::TextFrom { type_msg, username, text } => {
            if type_msg == "TEXT_FROM" {
                if server.users.contains_key(&username.to_lowercase()) {
                    let msg = generate_text_from_msg(letter.usr_sender, text);
                    let opt_user = server.users.get_mut(&username);
                    let user = opt_user.unwrap(); 
                    user.tx.send(msg).await.unwrap();                   
                }
                else {
                    let msg = generate_user_not_exist_response(username);
                    letter.reply_to.send(msg).await.unwrap();
                }
        }
        else {
            let message = generate_not_valid_msg().unwrap();
            server.users.remove(&letter.usr_sender.to_lowercase());
            letter.reply_to.send(message).await.unwrap();
        }
    }
    TypeReciveMessages::Identify { type_msg, username } => {
        if type_msg == "NEW_USER" {
            let msg = generate_new_user_msg(username.clone());
            for kv in server.users.iter() {
                if kv.key() == &username.to_lowercase() {
                    continue;
                }
                else {
                    kv.value().tx.send(msg.clone()).await.unwrap();
                }
            }
        }
        else {
            let message = generate_not_valid_msg().unwrap();
            server.users.remove(&letter.usr_sender.to_lowercase());
            letter.reply_to.send(message).await.unwrap();
        }
    }
    _ => ()
}

}



