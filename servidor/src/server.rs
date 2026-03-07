use crate::server::server_mesagges::{generate_not_identified_msg, generate_not_valid_msg, generate_succes_identify_response, generate_user_already_exists_response};
use crate::type_recive_messages::TypeReciveMessages;
use crate::user::User;
use crate::view;
use crate::letter::Letter;
use dashmap::DashMap;
use serde_json;
use std::sync::mpsc::{Receiver as Receptor, Sender as Senderr, self};
use std::net::{Ipv4Addr, SocketAddrV4};
use tokio::io::{AsyncBufReadExt, AsyncRead, AsyncWrite, AsyncWriteExt, BufReader, BufWriter};
use tokio::net::{TcpListener, TcpStream};
use std::sync::{Arc};
use crossbeam_channel::{unbounded, Sender, Receiver};
use tokio_util::codec::{FramedRead, LinesCodec};
use futures::stream::StreamExt;

pub mod server_mesagges;
pub mod aux_functions;

pub struct Server {
    users: Arc<DashMap<String, User>>,
    port: u16,
}

impl Server {
    pub fn new(port: u16) -> Server {
        let users = Arc::new(DashMap::new());
        Server {
            users,
            port,
        }
    }

    pub async fn run(){
        let port = view::get_port();
        let server = Server::new(port);
        let atm_server = Arc::new(server);
        //println!("Aceptando conexiones");
        Server::get_conections(atm_server, false).await;
    }  

    async fn run_local(){
        let server = Server::new(8080);
        let clone_server = Arc::new(server);
        //println!("Aceptando conexiones");
        Server::get_conections(clone_server, true).await;
    }

    async fn get_conections(server: Arc<Server>, local: bool) {
        let addr: SocketAddrV4;
        if local {
            addr = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), server.port);
        }
        else {
            addr = SocketAddrV4::new(Ipv4Addr::new(0, 0, 0, 0), server.port);
        }
        let listener = TcpListener::bind(addr).await.unwrap();
        let (tx, rx) = unbounded::<Letter<Vec<u8>>>();
        Server::build_msg_processors(rx);
        loop {
            let (socket, _) = listener.accept().await.unwrap();
            let server_for_client = Arc::clone(&server);
            let tx_for_client = tx.clone();
            tokio::spawn(async move { Server::process_conection(socket, server_for_client, tx_for_client).await });
        }
    } 

    async fn process_conection(socket: TcpStream, server: Arc<Server>, global_tx: Sender<Letter<Vec<u8>>>) {
        let (sok_reader, mut sok_writter) = socket.into_split();
        let (user_tx, user_rx) = mpsc::channel::<Vec<u8>>();
        let mut reader = FramedRead::new(sok_reader, LinesCodec::new_with_max_length(124));
        let Ok(msg) = reader.next().await.unwrap() else {
            return ;
        };
        let lectura = msg.trim();
        let mensagge: TypeReciveMessages;
            match serde_json::from_str(&lectura) {
                Ok(text) => {
                    mensagge = text;
                }
                Err(_) => {
                    let msg = generate_not_valid_msg().expect("Error al generar el mensaje de json inválido");
                    let Ok(_) = sok_writter.write(&msg).await else {
                        return ;
                    };
                    let Ok(_) = sok_writter.shutdown().await else {
                        return ;
                    };
                    return;
                }
            }
            if let TypeReciveMessages::Identify { type_msg, mut username } = mensagge {

                if type_msg != "IDENTIFY" {
                    let msg = generate_not_identified_msg().expect("Ocurrio un error al generar el mensaje");
                    let Ok(_) = sok_writter.write(&msg).await else {
                        return ;
                    };
                    let Ok(new_username) = aux_functions::retry_identify(&mut reader).await else {
                        let msg = server_mesagges::generate_not_valid_msg().unwrap();
                        let Ok(_) = sok_writter.write(&msg).await else {
                            return ;
                        };
                        return;
                    };
                    username = new_username;
                }

                let mut username_lowercase = username.to_lowercase();

                if server.users.contains_key(&username_lowercase) {
                    let msg = generate_user_already_exists_response(&username).expect("No fue posible generar el mensaje");
                    let Ok(_) = sok_writter.write(&msg).await else {
                        return ;
                    };
                    let Ok(new_username) = aux_functions::retry_identify(&mut reader).await else {
                        let msg = server_mesagges::generate_not_valid_msg().expect("Error el generar mensaje");
                        let Ok(_) = sok_writter.write(&msg).await else {
                            return ;
                        };
                        return;
                    };
                    let new_username_lower = new_username.to_lowercase();
                    if server.users.contains_key(&new_username_lower) {
                        let msg = server_mesagges::generate_not_valid_msg().expect("Error el generar mensaje");
                        let Ok(_) = sok_writter.write(&msg).await else {
                            return ;
                        };
                        return;
                    }
                    username = new_username;
                }
                username_lowercase = username.to_lowercase();
                let user_tx_clone = user_tx.clone();
                let username_clone = username.clone();
                tokio::spawn(async move {
                    Server::build_msg_client_processor(user_tx_clone, username_clone, reader, global_tx);
                });
                let msg = generate_succes_identify_response(&username).expect("No fue posible generar el mensaje");
                let Ok(_) = sok_writter.write(&msg).await else {
                    return ;
                };
                tokio::spawn(async move {
                    Server::build_msg_sender_to_client(user_rx, sok_writter);
                });
                println!("Nuevo usuario con nombre {} conectado", username);
                let new_user = User::new(username, user_tx);
                server.users.insert(username_lowercase, new_user);
            }
    }

    async fn build_msg_processors(rx: Receiver<Letter<Vec<u8>>>){
        let rx1 = rx.clone();
        let rx2 = rx.clone();
        let rx3 = rx.clone();
        tokio::spawn(async move {
            Server::process_letter(rx1);
        });
        tokio::spawn(async move {
            Server::process_letter(rx2);
        });
        tokio::spawn(async move {
            Server::process_letter(rx3);
        });
    }

    async fn process_letter(rx: Receiver<Letter<Vec<u8>>>) {
        unimplemented!();
    }

    async fn build_msg_sender_to_client<T: AsyncWrite + Unpin>(rx: Receptor<Vec<u8>>, mut writer: T) {
        for msg in rx {
            let Ok(_) = writer.write(&msg).await else {
                return ;
            };
        }
    }

    async fn build_msg_client_processor<T: AsyncRead + Unpin>(user_tx: Senderr<Vec<u8>>, 
                                                             username: String,
                                                             mut reader: FramedRead<T, LinesCodec>,
                                                            global_tx: Sender<Letter<Vec<u8>>>) {
        while let Some(result) = reader.next().await {
            match result {
                Ok(msg) => {
                    //Pasar el mensaje recibido "msg" a algun tipo de mensaje de entrada
                    let letter = aux_functions::generate_letter(username, user_tx, msg);
                    global_tx.send(letter);
                }
                Err(_) => {
                    //Tiene que mandar la señal para desconectar el cliente.
                    //Es decir cerrar su transmisor para que no se sigan
                    //enviando mensajes.
                    break;
                }
            }
        }

    }
    

}

#[cfg(test)]
mod test {
    use std::{net::SocketAddrV4, sync::{Arc}};
    use serde_json::{Value, json};
    use tokio::sync::Mutex;
    use super::*;
    use tokio::net::{TcpListener, TcpStream};
    use tokio::io::BufWriter;

    use crate::{server::Server, type_recive_messages::TypeReciveMessages};

    async fn setup_server(port: u16) -> (TcpStream, TcpStream, Arc<Mutex<Server>>){
        let server = Server::new_debug(127,0,0,1, port);
        let listener = TcpListener::bind(SocketAddrV4::new(server.adress, server.port)).
                                                                        await.unwrap();
        let server_task = tokio::spawn(async move {
            listener.accept().await.unwrap()
        });
        
        let client_stream = TcpStream::connect(SocketAddrV4::new(server.adress, server.port))
                                                                                                  .await.unwrap();
        let (stream, _addrs) = server_task.await.unwrap();                                                                               
        let server_mutx = Arc::new(Mutex::new(server));
        (stream, client_stream, server_mutx)
    }

    async fn get_client(port: u16) -> TcpStream{
        let ip = Ipv4Addr::new(127, 0, 0, 1);
        let client_stream = TcpStream::connect(SocketAddrV4::new(ip, port)).await.unwrap();
        client_stream
    }

    #[tokio::test]
    async fn test_process_conection(){
        let (server_socket, client_socket, server) = setup_server(8080).await;
        let msg = TypeReciveMessages::Identify { 
            type_msg: String::from("IDENTIFY"),
            username: String::from("Karla") 
        };
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

    #[tokio::test]
    async fn test_process_conection_usr_alr_exist(){
        let ip = Ipv4Addr::new(127, 0, 0, 1);
        let server = Arc::new(Mutex::new(Server::new(ip, 4444)));
        let clone_server = Arc::clone(&server);

        tokio::spawn(async move {
            Server::run(clone_server).await;
        });

        tokio::time::sleep(std::time::Duration::from_millis(50)).await;

        let client_1 = get_client(4444).await;
        let (_, sok_writer_1) = client_1.into_split();
        let mut buf_writer_1 = BufWriter::new(sok_writer_1);
        
        let msg = TypeReciveMessages::Identify { 
            type_msg: String::from("IDENTIFY"), 
            username: String::from("Karla") 
        };
        let mut json_msg = serde_json::to_vec(&msg).unwrap();
        json_msg.push(b'\n');
        
        buf_writer_1.write_all(&json_msg).await.unwrap();
        buf_writer_1.flush().await.unwrap();

        tokio::time::sleep(std::time::Duration::from_millis(50)).await;

        let cliente_2 = get_client(4444).await;
        let (sok_reader_2, sok_writter_2) = cliente_2.into_split();
        let mut buf_writer_2 = BufWriter::new(sok_writter_2);
        let mut buf_reader_2 = BufReader::new(sok_reader_2);
        
        buf_writer_2.write_all(&json_msg).await.unwrap();
        buf_writer_2.flush().await.unwrap();

        let mut line = String::new();
        let bytes = buf_reader_2.read_line(&mut line).await.unwrap();
        assert!(!(bytes == 0));
        
        let server_response: Value = serde_json::from_str(line.trim()).unwrap();
        
        let expected_response = json!({ 
            "type": "RESPONSE",
            "operation": "IDENTIFY",
            "result": "USER_ALREADY_EXISTS",
            "extra": "Karla" 
        });
        
        assert_eq!(server_response, expected_response);

    }
}
