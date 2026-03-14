use crate::{server::server_mesagges::{generate_invitation_msg, generate_new_room_user_msg, generate_not_invitated_msg}, user::User};
use tokio::sync::mpsc::Sender;
use dashmap::DashMap;


pub struct Room {
    pub(crate) name: String,
    pub(crate) users: DashMap<String, User>,
    pub(crate) guests: DashMap<String, User>,
}

impl Room {
    pub(crate) async fn send_msg(&self, username: String, msg: Vec<u8>) {
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
}
