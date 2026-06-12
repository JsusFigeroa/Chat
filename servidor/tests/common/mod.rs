use serde_json::json;
use servidor::server::Server;
use servidor::type_recive_messages::TypeReciveMessages;
use tokio::io::AsyncWriteExt;
use tokio::{
    io::BufReader,
    net::{
        TcpStream,
        tcp::{OwnedReadHalf, OwnedWriteHalf},
    },
};

pub fn clean_readed_line(line: &str) -> &str {
    let trim_line = line.trim();
    trim_line.trim_matches(|c| c == '\0')
}

pub async fn identify_client(addr: &str, name: &str) -> (BufReader<OwnedReadHalf>, OwnedWriteHalf) {
    let connection = TcpStream::connect(addr).await;
    let socket = connection.expect("Error al conectarse al servidor");
    let (sok_reader, mut sok_writer) = socket.into_split();
    let msg = json!({"type":"IDENTIFY", "username":name});
    let mut message = serde_json::to_vec(&msg).unwrap();
    message.push(b'\n');
    let Ok(()) = sok_writer.write_all(&message).await else {
        panic!()
    };
    let reader = BufReader::new(sok_reader);
    (reader, sok_writer)
}

pub async fn start_server(port: u16) {
    let server = Server::new(port);
    tokio::spawn(async move {
        server.run().await;
    });
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;
}

pub fn generate_vec_msg(message: &TypeReciveMessages) -> Vec<u8> {
    let mut msg = serde_json::to_vec(message).unwrap();
    msg.push(b'\n');
    msg
}
