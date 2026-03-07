use crate::type_recive_messages::TypeReciveMessages;
use crossbeam_channel::Sender;


pub struct Letter<T> {
    sender: String,
    msg: TypeReciveMessages,
    reply_to: Sender<T>
}