use std::{collections::HashMap, sync::Arc};

use crate::{room, server::server_mesagges::{generate_invitation_msg, generate_new_room_user_msg, generate_not_invitated_msg, generate_not_joined_response, generate_not_user_response, generate_room_users_msg, generate_user_leaved_room}, user::User};
use tokio::sync::mpsc::Sender;
use dashmap::{DashMap, Entry, OccupiedEntry};


pub struct Room {
    pub(crate) name: String,
    pub(crate) users: DashMap<String, User>,
    pub(crate) guests: DashMap<String, User>,
}

impl Room {
    pub(crate) async fn send_msg(&self, username: String, msg: Vec<u8>, tx_user_sender: Sender<Vec<u8>>) {
        if !self.users.contains_key(&username) {
            let msg = generate_not_joined_response(&username, &self.name);
            let _ = tx_user_sender.send(msg).await;
        }
      let users = &self.users;
      let usr_sender = String::from(username.to_lowercase());
      let mut senders = Vec::new();
      for kv in users {
        if kv.key() == &usr_sender {
            continue;
        }
        senders.push(kv.tx.clone());
      }
      for user in senders {
       let _ = user.send(msg.clone()).await;
      }  
    }
    /// Invita a los usuarios de una lista a un cuarto, si los usuarios ya están invitados o 
    /// en el cuarto no hace nada con ellos.
    pub(crate) async fn process_invitation(&self, users: Vec<User>, usr_sender: &str) {
        let mut invite_users = Vec::new();
        for user in users {
            let username_lower = user.name.to_lowercase();
            if (self.guests.contains_key(&username_lower)) || (self.users.contains_key(&username_lower)) {
                continue;
            }
            invite_users.push(user);
        }
        let mut senders = Vec::new();
        for user in invite_users.iter() {
            senders.push(user.tx.clone());
        }
        for user in invite_users {
            self.guests.insert(String::from(user.name.to_lowercase()), user);
        }
        let msg = generate_invitation_msg(usr_sender, &self.name);
        for tx in senders {
            let _ = tx.send(msg.clone()).await;
        }
    }
    /// Esta función toma un usuario que aceptó una invitación y lo agrega el cuarto en caso
    /// de estar invitado al mismo, en caso de no estar invitado envía el mensaje correspondiente
    /// según el protocolo.
    pub(crate) async fn accept_invitation(&self, usr_who_accepted: &str, tx_user_who_accepted: Sender<Vec<u8>>) {
        let username_to_lower = usr_who_accepted.to_lowercase();
        if self.guests.contains_key(&username_to_lower) {
            let (name, user) = self.guests.remove(&username_to_lower).unwrap();
            let msg = generate_new_room_user_msg(&self.name, usr_who_accepted);
            let mut senders = Vec::new();
            for kv in self.users.iter() {
                senders.push(kv.tx.clone());
            }
            for sender in senders {
                let _ = sender.send(msg.clone()).await;
            }
            self.users.insert(name, user);
            
        }
        else {
            let msg = generate_not_invitated_msg(&self.name);
            let _ = tx_user_who_accepted.send(msg).await;
        }

    }

    pub(crate) async fn send_users(&self, tx_user_to_send: Sender<Vec<u8>>) {
        let mut map = HashMap::new();
        for user in self.users.iter() {
            let username = user.name.clone();
            map.insert(username, user.state);
        }
        let msg = generate_room_users_msg(&self.name, map);
        let _ = tx_user_to_send.send(msg).await;
    }

    pub(crate) async fn remove_user(&self, tx_user_to_remove: Sender<Vec<u8>>, user_to_remove: String) -> usize {
        let username_lower = user_to_remove.to_lowercase(); 
        if self.users.contains_key(&username_lower) {
            self.users.remove(&username_lower);
            let msg = generate_user_leaved_room(&user_to_remove, &self.name);
            let _ = tx_user_to_remove.send(msg).await;
            self.users.len()
        }
        else {
            let msg = generate_not_joined_response(&user_to_remove, &self.name);
            let _ = tx_user_to_remove.send(msg).await;
            self.users.len()
        }
    }
}
