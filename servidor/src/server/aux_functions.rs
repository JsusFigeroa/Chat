use crate::server::server_mesagges::{generate_disconected_msg, no_such_room_join_room_msg};
use crate::type_recive_messages::TypeReciveMessages;
use std::collections::HashMap;
use crate::user::User;
use dashmap::Entry;
use tokio::io::AsyncRead;
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
    match letter.msg {
    TypeReciveMessages::PublicText { text } => {
        let mut transmisors = Vec::new();
        for kv in server.users.iter() {
            if kv.name.to_lowercase() == letter.usr_sender.to_lowercase() {
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
        let msg = generate_disconected_msg(&letter.usr_sender).unwrap();
        server.users.remove(&letter.usr_sender.to_lowercase());
        let mut transmisors = Vec::new();
        for kv in server.users.iter() {
            transmisors.push(kv.tx.clone());
        }
        
        for tx in transmisors {
            tx.send(msg.clone()).await.unwrap();
        }
    }
    
    TypeReciveMessages::Invite { roomname, usernames } => {
        if server.rooms.contains_key(&roomname.to_lowercase()) {
            let opt_user = usernames.iter().find(|&k| !server.users.contains_key(&k.to_lowercase()));
            match opt_user {
                Some(username) => {
                    let msg = server_mesagges::no_such_user_invite_msg(username);
                    let _ = letter.reply_to.send(msg).await;
                }
                None => {
                    let mut users = Vec::new();
                    for username in usernames {
                        let opt_user = server.users.get(&username)
                                                                .map(|ref_usr| ref_usr.value().clone());
                        if let Some(user) = opt_user {
                            users.push(user);
                        };
                    }
                    let opt_kv = server.rooms.get(&roomname.to_lowercase());
                    if let Some(kv) = opt_kv {
                        let room = kv.value();
                        let _ = room.process_invitation(users, &letter.usr_sender).await;
                    };
                }
            }
        }
        else {
            let msg = server_mesagges::no_such_room_invite_msg(&roomname);
            let _ = letter.reply_to.send(msg).await;
        }
    }
    TypeReciveMessages::JoinRoom { roomname } => {
        if server.rooms.contains_key(&roomname.to_lowercase()) {
            let opt_room = server.rooms.get(&roomname.to_lowercase());
            if let Some(room) = opt_room {
                let _ = room.accept_invitation(&letter.usr_sender, letter.reply_to).await;
            };
        }
        else {
            let msg = no_such_room_join_room_msg(&roomname);
            let _ = letter.reply_to.send(msg).await;
        }
    }
    TypeReciveMessages::NewRoom { roomname } => {
        let opt_user = server.users.get(&letter.usr_sender.to_lowercase())
                                                                .map(|k| k.value().clone());
        match opt_user {
            Some(usr) => {
                let roomname_lower = roomname.to_lowercase();
                let room = Room::new(roomname.clone(), usr);                
                server.rooms.insert(roomname_lower, room);
                let msg = server_mesagges::new_room_success(&roomname);
                let _ = letter.reply_to.send(msg).await;
            }
            None => {return ;}
        }
    }
    TypeReciveMessages::LeaveRoom { roomname } => {
        if server.rooms.contains_key(&roomname.to_lowercase()) {
            let entry = server.rooms.entry(roomname.to_lowercase());
            match entry {
                Entry::Occupied(mut locked_entry) => {
                    let room = locked_entry.get_mut();
                    let num_users = room.remove_user(letter.reply_to,
                                                             letter.usr_sender).await;
                    if num_users == 0 {
                        locked_entry.remove();
                    }
                }
                Entry::Vacant(_) => {}
            }
        }
        else {
            let msg = server_mesagges::leave_room_not_such_room(&roomname);
            let _ = letter.reply_to.send(msg).await;
        }
    }
    TypeReciveMessages::RoomText { roomname, text } => {
        if server.rooms.contains_key(&roomname.to_lowercase()) {
            let kv_opt = server.rooms.get(&roomname.to_lowercase());
            if let Some(kv) = kv_opt {
                let room = kv.value();
                let msg = text.into_bytes();
                let _ = room.send_msg(letter.usr_sender, msg, letter.reply_to).await;
            }
        }
        else {
            let msg = server_mesagges::room_text_no_such_room(&roomname);
            let _ = letter.reply_to.send(msg).await;
        }
    }
    TypeReciveMessages::RoomUsers { roomname } => {
        if server.rooms.contains_key(&roomname.to_lowercase()) {
            let opt_room = server.rooms.get(&roomname.to_lowercase());
            if let Some(kv) = opt_room {
                let room = kv.value();
                let _ = room.send_users(letter.reply_to, letter.usr_sender).await;
            }
        }
        else {
            let msg = server_mesagges::room_users_no_such_room(&roomname);
            let _ = letter.reply_to.send(msg).await;
        }
    }
}

}



