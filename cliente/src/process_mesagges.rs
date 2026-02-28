use std::io::Write;
use std::string;
use std::net::TcpStream;
use std::io::BufWriter;
use crate::type_recive_mesagges::TypeReciveMesagge;
use crate::type_send_mesagges::TypeSendMesagges;
use serde::Deserialize;
use serde_json::{Deserializer};
use serde_json;

pub fn send_identifier(username: String, socket: &TcpStream) {
    let type_msg = String::from("IDENTIFY");
    let identifier = TypeSendMesagges::Identify { typ_msg: type_msg, username: username };
    let mut json = serde_json::to_string(&identifier).expect("No fue posible serializar Mesasagge::Identify");
    let mut buf_socket = BufWriter::new(socket);
    json.push_str("\n");
    let mut json_to_bytes = json.as_bytes();
    buf_socket.write_all(json_to_bytes).expect("No fue posible escribir el mensaje en el búfer");
    buf_socket.flush().expect("No fue posible enviar el mensaje");

}