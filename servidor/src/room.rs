use crate::{
    server::server_mesagges::{
        self, generate_invitation_msg, generate_new_room_user_msg, generate_not_invitated_msg,
        generate_not_joined_response, generate_room_users_msg, generate_user_leaved_room,
        room_users_not_joined,
    },
    user::User,
};
use dashmap::DashMap;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::mpsc::Sender;
#[derive(Clone)]
pub struct Room {
    name: String,
    users: DashMap<String, Arc<User>>,
    guests: DashMap<String, Arc<User>>,
}

impl Room {
    pub(crate) fn new(name: String, first_usr: Arc<User>) -> Room {
        let users = DashMap::new();
        let guests = DashMap::new();
        users.insert(first_usr.name.to_lowercase(), first_usr);
        Room {
            name,
            users,
            guests,
        }
    }

    pub(crate) async fn send_msg(
        &self,
        username: String,
        text: String,
        tx_user_sender: Sender<Vec<u8>>,
    ) {
        let username_lowe = &username.to_lowercase();
        if !self.users.contains_key(username_lowe) {
            let msg = generate_not_joined_response(&self.name);
            let _ = tx_user_sender.send(msg).await;
            return;
        }
        let users = &self.users;
        let usr_sender = username_lowe.clone();
        let mut senders = Vec::new();
        for kv in users {
            if kv.key() == &usr_sender {
                continue;
            }
            senders.push(kv.tx.clone());
        }
        let message = server_mesagges::generate_room_text_from(&username, &self.name, &text);
        for user in senders {
            let _ = user.send(message.clone()).await;
        }
    }
    /// Invita a los usuarios de una lista a un cuarto, si los usuarios ya están invitados o
    /// en el cuarto no hace nada con ellos.
    /// Si el usuario que invita a el cuarto no está en el la lista de usuarios del cuarto,
    /// no hace nada.
    pub(crate) async fn process_invitation(&self, users: Vec<Arc<User>>, usr_sender: &str) {
        if !self.users.contains_key(&usr_sender.to_lowercase()) {
            return;
        }
        let mut invite_users = Vec::new();
        for user in users {
            let username_lower = user.name.to_lowercase();
            if (self.guests.contains_key(&username_lower))
                || (self.users.contains_key(&username_lower))
            {
                continue;
            }
            invite_users.push(user);
        }
        let mut senders = Vec::new();
        for user in &invite_users {
            senders.push(user.tx.clone());
        }
        for user in invite_users {
            let mut invitation_room_keys_guard = user.invitations_room_keys.lock().await;
            invitation_room_keys_guard.push(self.name.to_lowercase());
            drop(invitation_room_keys_guard);
            self.guests.insert(user.name.to_lowercase(), user);
        }
        let msg = generate_invitation_msg(usr_sender, &self.name);
        for tx in senders {
            let _ = tx.send(msg.clone()).await;
        }
    }

    /// Esta función toma un usuario que aceptó una invitación y lo agrega el cuarto en caso
    /// de estar invitado al mismo, también envía el mensaje de que un nuevo usuario se ha unido al grupo,
    /// en caso de no estar invitado envía el mensaje correspondiente según el protocolo.
    pub(crate) async fn accept_invitation(
        &self,
        usr_who_accepted: &str,
        tx_user_who_accepted: Sender<Vec<u8>>,
    ) {
        let username_to_lower = usr_who_accepted.to_lowercase();
        if let Some((name, user)) = self.guests.remove(&username_to_lower) {
            let msg = generate_new_room_user_msg(&self.name, usr_who_accepted);
            let mut senders = Vec::new();
            for kv in &self.users {
                senders.push(kv.tx.clone());
            }
            let mut invitations_guard = user.invitations_room_keys.lock().await;
            if let Some(index) = invitations_guard
                .iter()
                .rposition(|key| key == &self.name.to_lowercase())
            {
                invitations_guard.remove(index);
            }
            drop(invitations_guard);
            let mut room_keys_guard = user.rooms_keys.lock().await;
            (*room_keys_guard).push(self.name.to_lowercase());
            self.users.insert(name, user.clone());
            for sender in senders {
                let _ = sender.send(msg.clone()).await;
            }
            let user_msg = server_mesagges::success_join_room_response(&self.name);
            let _ = tx_user_who_accepted.send(user_msg).await;
        } else {
            let msg = generate_not_invitated_msg(&self.name);
            let _ = tx_user_who_accepted.send(msg).await;
        }
    }

    pub(crate) async fn send_users(&self, tx_user_to_send: Sender<Vec<u8>>, user_who_asks: String) {
        if !self.users.contains_key(&user_who_asks.to_lowercase()) {
            let msg = room_users_not_joined(&self.name);
            let _ = tx_user_to_send.send(msg).await;
        }
        let mut map = HashMap::new();
        let mut users = Vec::with_capacity(self.users.len());
        for user in &self.users {
            users.push(user.clone());
        }
        for user in users {
            let username = user.name.clone();
            let state_guard = user.state.lock().await;
            let state = *state_guard;
            drop(state_guard);
            map.insert(username, state);
        }
        let msg = generate_room_users_msg(&self.name, map);
        let _ = tx_user_to_send.send(msg).await;
    }
    /// Elimina a un usuario del cuarto si estaba en el cuarto, en otro caso envía el mensaje correspondiente
    /// según el protocolo, esta función devuelve el número de usuarios en el cuarto después de eliminar al usuario.
    pub(crate) async fn remove_user(
        &self,
        tx_user_to_remove: Sender<Vec<u8>>,
        user_to_remove: String,
    ) -> usize {
        let username_lower = user_to_remove.to_lowercase();
        if self.users.contains_key(&username_lower) {
            self.users.remove(&username_lower);
            let msg = generate_user_leaved_room(&user_to_remove, &self.name);
            let mut senders = Vec::new();
            for user in &self.users {
                let tx = user.tx.clone();
                senders.push(tx);
            }
            for tx in senders {
                let _ = tx.send(msg.clone()).await;
            }
            self.users.len()
        } else {
            let msg = server_mesagges::generate_not_joined_leave_room_response(&self.name);
            let _ = tx_user_to_remove.send(msg).await;
            self.users.len()
        }
    }

    pub(crate) fn remove_invitation(&self, username: &str) {
        self.guests.remove(&username.to_lowercase());
    }

    pub(crate) async fn remove_disconected_user(&self, user_to_remove: &str) -> usize {
        let username_lower = user_to_remove.to_lowercase();
        if self.users.contains_key(&username_lower) {
            self.users.remove(&username_lower);
            let msg = generate_user_leaved_room(user_to_remove, &self.name);
            let mut senders = Vec::new();
            for user in &self.users {
                let tx = user.tx.clone();
                senders.push(tx);
            }
            for tx in senders {
                let _ = tx.send(msg.clone()).await;
            }
            self.users.len()
        } else {
            self.users.len()
        }
    }
}
