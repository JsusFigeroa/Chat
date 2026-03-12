use crate::type_recive_messages::TypeReciveMessages;
use tokio::sync::mpsc::Sender;

#[derive(Clone, Debug)]
pub struct Letter<T> {
    pub usr_sender: String,
    pub msg: TypeReciveMessages,
    pub reply_to: Sender<T>
}

impl<T> Letter<T> {
    pub(crate) fn new(usr_sender: String, msg: TypeReciveMessages, reply_to: Sender<T>) -> Letter<T> {
        Letter { usr_sender, msg, reply_to}
    }
}