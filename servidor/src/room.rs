use std::sync::Arc;
use crate::user::User;
use dashmap::DashMap;


pub struct Room {
    pub(crate) users: Arc<DashMap<String, User>>,
    pub(crate) guests: Arc<DashMap<String, User>>,
    pub(crate) usr_rejected: Arc<DashMap<String, User>>
}

impl Room {
    pub fn send_msg(username: String, msg: Vec<u8>) {
        
    }
}
