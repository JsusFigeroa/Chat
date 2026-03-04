use crate::server::server_mesagges::{generate_not_identified_msg, generate_not_valid_msg, generate_succes_identify_response, generate_user_already_exists_response};
use crate::type_recive_mesagges::TypeReciveMesagges;
use crate::type_send_mesagges::TypeSendMesagges;
use crate::user::User;
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use std::net::{Ipv4Addr, SocketAddrV4};
use std::time::Duration;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader, BufWriter, AsyncReadExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::runtime::Runtime;
use std::sync::{Arc};
use tokio::sync::Mutex;

pub struct Server {
    users: HashMap<String, User>,
    address: Ipv4Addr,
    port: u16,
}

impl Server {
    pub fn new() -> Server {
        let users: HashMap<String, User> = HashMap::new();
        let address = Ipv4Addr::new(127, 0, 0, 1);
        let port = 4444;

        Server {
            users,
            address,
            port,
        }
    }

    pub async fn run(server: Server){
        let server = Arc::new(Mutex::new(server));
        let clone_server = Arc::clone(&server);
        tokio::spawn(async move {
            Server::get_conections(clone_server).await;
        });
    }  

    async fn get_conections(clone_sever: Arc<Mutex<Server>>) {
        let (adress, port) = {
            let locked_server = clone_sever.lock().await;
            (locked_server.address, locked_server.port)
        };
        let listener = TcpListener::bind(SocketAddrV4::new(adress, port))
            .await
            .unwrap();
        loop {
            let (socket, _) = listener.accept().await.unwrap();
            let server_for_client = Arc::clone(&clone_sever);
            tokio::spawn(async move { Server::process_conection(socket, server_for_client).await });
        }
    } 

    async fn process_conection(socket: TcpStream, server: Arc<Mutex<Server>>) {
        let (sok_reader, sok_writter) = socket.into_split();
        let mut buf_reader = BufReader::new(sok_reader);
        let mut buf_writer = BufWriter::new(sok_writter);
        let mut lectura = String::new();

            let bytes_leidos = buf_reader.read_line(&mut lectura).await.unwrap();
            let lectura = lectura.trim();

            if bytes_leidos == 0 {
                buf_writer.shutdown().await.expect("Error al cerrar conexión");
            }
            let mensagge: TypeReciveMesagges;
            match serde_json::from_str(&lectura) {
                Ok(text) => {
                    mensagge = text;
                }
                Err(_) => {
                    let msg = generate_not_valid_msg().expect("Error al generar el mensaje de json inválido");
                    buf_writer.write_all(&msg).await.expect("No fue posible escribir en el socket");
                    buf_writer.flush().await.unwrap_or_default();
                    buf_writer.shutdown().await.expect("Error al cerrar la conexión");
                    return;
                }
            }
            if let TypeReciveMesagges::Identify { type_msg, username } = mensagge {
                if type_msg != "IDENTIFY" {
                    let msg = generate_not_identified_msg().expect("Ocurrio un error al generar el mensaje");
                    buf_writer.flush().await.unwrap_or_default();
                    buf_writer.write_all(&msg).await.expect("Error al escribir en socket");
                    buf_writer.shutdown().await.expect("Error al desconectar usuario");
                    return ;
                }
                let username_lowercase = username.to_lowercase();
                let mut locked_server = server.lock().await;
                if locked_server.users.contains_key(&username_lowercase) {
                    let msg = generate_user_already_exists_response(&username).expect("No fue posible generar el mensaje");
                    buf_writer.write_all(&msg).await.expect("No fue posible escribir en el stream");
                    buf_writer.flush().await.unwrap_or_default();
                    buf_writer.shutdown().await.expect("Error al cerrar el stream");
                    return ;
                }
                let msg = generate_succes_identify_response(&username).expect("No fue posible generar el mensaje");
                buf_writer.write_all(&msg).await.expect("No fue posible escribir");
                buf_writer.flush().await.unwrap_or_default();
                let new_user = User::new(username, buf_reader, buf_writer);
                locked_server.users.insert(username_lowercase, new_user);
            }
    }

}

pub mod server_mesagges;

#[cfg(test)]
mod test {
    use std::{net::SocketAddrV4, os::unix::process, sync::{Arc}};
    use serde_json::{Value, json};
    use tokio::sync::Mutex;
    use super::*;
    use tokio::net::{TcpListener, TcpStream};
    use tokio::io::BufWriter;

    use crate::{server::Server, type_recive_mesagges::TypeReciveMesagges};

    async fn setup_server() -> (TcpStream, TcpStream, Arc<Mutex<Server>>){
        let server = Server::new();
        let listener = TcpListener::bind(SocketAddrV4::new(server.address, server.port)).
                                                                        await.unwrap();
        let server_task = tokio::spawn(async move {
            listener.accept().await.unwrap()
        });
        
        let client_stream = TcpStream::connect(SocketAddrV4::new(server.address, server.port))
                                                                                                  .await.unwrap();
        let (stream, _addrs) = server_task.await.unwrap();                                                                               
        let server_mutx = Arc::new(Mutex::new(server));
        (stream, client_stream, server_mutx)
    }
    #[tokio::test]
    async fn test_process_conection(){
        let (server_socket, client_socket, server) = setup_server().await;
        let clone_server = Arc::clone(&server);
        let msg = TypeReciveMesagges::Identify { type_msg: String::from("IDENTIFY"),
                                                                     username: String::from("Karla") };
        tokio::spawn(async move {
            Server::process_conection(server_socket, server).await;
        });
        let (sok_reader, sok_writter) = client_socket.into_split();
        let mut buf_writer = BufWriter::new(sok_writter);
        let mut json_msg = serde_json::to_vec(&msg).unwrap();
        json_msg.push(b'\n');
        buf_writer.write_all(&json_msg).await.unwrap_or_default();
        buf_writer.flush().await.unwrap_or_default();
        let mut buf_reader = BufReader::new(sok_reader);
        let mut line = String::new();
        let readed_bytes = buf_reader.read_line(&mut line).await.unwrap();
        assert!(!(readed_bytes == 0));
        let line = line.trim();
        let server_response: Value = serde_json::from_str(line).unwrap();
        let json_expected = json!(
            {   
                "type": "RESPONSE",
                "operation": "IDENTIFY",
                "result": "SUCCESS",
                "extra": "Karla" 
            }
        );
        assert_eq!(json_expected, server_response);

    }
}
