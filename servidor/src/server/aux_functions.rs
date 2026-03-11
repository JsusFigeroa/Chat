use crate::server::server_mesagges::{generate_disconected, generate_not_identified_msg, generate_not_valid_msg, generate_succes_identify_response, generate_user_already_exists_response};
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
    if let TypeReciveMessages::Identify { username } = mensagge {
        return Ok(username)
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
    println!("llego al procesador");
    println!("Mensaje interpretado por Rust: {:#?}", letter.msg);
    match letter.msg {
    TypeReciveMessages::PublicText { text } => {
        let mut transmisors = Vec::new();
        for kv in server.users.iter() {
            if kv.name == letter.usr_sender.to_lowercase() {
                continue;
            }
            transmisors.push(kv.tx.clone());
        }
        let msg = generate_public_text_from(&letter.usr_sender, text);
        for tx in transmisors {
            let Ok(_) = tx.send(msg.clone()).await else {
                return ;
            };
        }
    }
    TypeReciveMessages::Status { status } => {
        if let Some(mut user) = server.users.get_mut(&letter.usr_sender.to_lowercase()) {
            user.state = status;
        }
        let mut transmisors = Vec::new();
        let msg = generate_new_status_msg(&letter.usr_sender, status);
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
    TypeReciveMessages::Users => {
        let map = generate_map_users(server.users.clone());
        let msg = generate_users_msg(map);
        letter.reply_to.send(msg).await.unwrap();
    }
    TypeReciveMessages::Text { username, text } => {
        if server.users.contains_key(&username.to_lowercase()) {
            let msg = generate_text_from_msg(letter.usr_sender, text);
            let opt_user = server.users.get_mut(&username.to_lowercase());
            let user = opt_user.unwrap(); 
            let usr_tx = user.tx.clone();
            usr_tx.send(msg).await.unwrap();                   
        }
        else {
            let msg = generate_user_not_exist_response(username);
            letter.reply_to.send(msg).await.unwrap();
        }
    }
    TypeReciveMessages::Identify { username } => {
        let msg = generate_new_user_msg(username.clone());
        let mut transmisors = Vec::new();
        for kv in server.users.iter() {
            if kv.key() == &username.to_lowercase() {
                continue;
            }
            else {
                transmisors.push(kv.tx.clone());
            }
        }
        for tx in transmisors {
            tx.send(msg.clone()).await.unwrap();
        }
    }
    TypeReciveMessages::Disconect => {
        let msg = generate_disconected(&letter.usr_sender).unwrap();
        let mut transmisors = Vec::new();
        for kv in server.users.iter() {
            if kv.key() == &letter.usr_sender.to_lowercase() {
                continue;
            }
            else {
                transmisors.push(kv.tx.clone());
            }
        }
        server.users.remove(&letter.usr_sender.to_lowercase());
        for tx in transmisors {
            tx.send(msg.clone()).await.unwrap();
        }
    }
}

}



