use crate::type_recive_messages::TypeReciveMessages;
use std::sync::mpsc::Sender;


pub struct Letter<T> {
    pub usr_sender: String,
    pub msg: TypeReciveMessages,
    pub reply_to: Sender<T>
}

impl Letter<T> {
    pub(crate) fn new<T>(usr_sender: String, msg: TypeReciveMessages, reply_to: Sender<T>) -> Letter<T> {
        Letter { usr_sender, msg, reply_to}
    }
}