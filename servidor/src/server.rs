use crate::server::aux_functions::{generate_map_users, procces_letter_aux};
use crate::server::server_mesagges::{generate_disconected_msg, generate_new_status_msg, generate_new_user_msg, generate_not_identified_msg, generate_not_valid_msg, generate_public_text_from, generate_succes_identify_response, generate_text_from_msg, generate_user_already_exists_response, generate_user_not_exist_response, generate_users_msg};
use crate::type_recive_messages::TypeReciveMessages;
use crate::type_send_messages::TypeSendMessages;
use crate::user::{self, State, User};
use crate::view;
use crate::letter::Letter;
use dashmap::DashMap;
use serde_json::{self, ser};
use tokio::sync::mpsc::{Receiver, Sender, self};
use std::net::{Ipv4Addr, SocketAddrV4};
use tokio::io::{AsyncBufReadExt, AsyncRead, AsyncWrite, AsyncWriteExt, BufReader, BufWriter};
use tokio::net::{TcpListener, TcpStream};
use std::sync::{Arc};
use tokio_util::codec::{FramedRead, LinesCodec};
use futures::stream::StreamExt;
use async_channel::{self, Sender as Senderr, Receiver as Receiverr};

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
        println!("Aceptando conexiones");
        Server::get_conections(atm_server, false).await;
    }  

    async fn run_local(){
        let server = Server::new(8080);
        let clone_server = Arc::new(server);
        println!("Aceptando conexiones");
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
        println!("puerto: {}", listener.local_addr().unwrap());
        let (tx, rx) = async_channel::bounded::<Letter<Vec<u8>>>(124);
        Server::build_msg_processors(rx.clone(),  server.clone());
        loop {
            let (socket, _) = listener.accept().await.unwrap();
            println!("Conexión aceptada");
            let server_for_client = Arc::clone(&server);
            let tx_for_client = tx.clone();
            tokio::spawn(async move { Server::process_conection(socket, server_for_client, tx_for_client).await });
        }
    } 

    async fn process_conection(socket: TcpStream, server: Arc<Server>, global_tx: Senderr<Letter<Vec<u8>>>) {
        let (sok_reader, mut sok_writter) = socket.into_split();
        let (user_tx, user_rx) = mpsc::channel::<Vec<u8>>(124);
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
                    let Ok(_) = sok_writter.write_all(&msg).await else {
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
                    let Ok(_) = sok_writter.write_all(&msg).await else {
                        return ;
                    };
                    let Ok(new_username) = aux_functions::retry_identify(&mut reader).await else {
                        let msg = server_mesagges::generate_not_valid_msg().unwrap();
                        let Ok(_) = sok_writter.write_all(&msg).await else {
                            return ;
                        };
                        return;
                    };
                    username = new_username;
                }

                let mut username_lowercase = username.to_lowercase();

                if server.users.contains_key(&username_lowercase) {
                    let msg = generate_user_already_exists_response(&username).expect("No fue posible generar el mensaje");
                    let Ok(_) = sok_writter.write_all(&msg).await else {
                        return ;
                    };
                    let Ok(new_username) = aux_functions::retry_identify(&mut reader).await else {
                        let msg = server_mesagges::generate_not_valid_msg().expect("Error el generar mensaje");
                        let Ok(_) = sok_writter.write_all(&msg).await else {
                            return ;
                        };
                        return;
                    };
                    let new_username_lower = new_username.to_lowercase();
                    if server.users.contains_key(&new_username_lower) {
                        let msg = server_mesagges::generate_not_valid_msg().expect("Error el generar mensaje");
                        let Ok(_) = sok_writter.write_all(&msg).await else {
                            return ;
                        };
                        return;
                    }
                    username = new_username;
                }
                username_lowercase = username.to_lowercase();
                let user_tx_clone = user_tx.clone();
                let username_clone = username.clone();
                let global_tx_clone = global_tx.clone();
                tokio::spawn(async move {
                    Server::build_msg_client_processor(user_tx_clone, username_clone, reader, global_tx_clone).await;
                });
                let msg = generate_succes_identify_response(&username).expect("No fue posible generar el mensaje");
                let Ok(_) = sok_writter.write_all(&msg).await else {
                    return ;
                };
                tokio::spawn(async move {
                    Server::build_msg_sender_to_client(user_rx, sok_writter).await;
                });
                println!("Nuevo usuario con nombre {} conectado", username);
                let new_user = User::new(username, user_tx);
                let msg_new_user = TypeReciveMessages::Identify { type_msg: String::from("NEW_USER"), username: new_user.name.clone() };
                let letter = Letter::new(new_user.name.clone(), msg_new_user, new_user.tx.clone());
                server.users.insert(username_lowercase, new_user);
                let Ok(_) = global_tx.send(letter).await else {
                    return ;
                };    
            }
    }

    fn build_msg_processors(rx: Receiverr<Letter<Vec<u8>>>,server: Arc<Server>){
        let rx1 = rx.clone();
        let rx2 = rx.clone();
        let rx3 = rx.clone();
        drop(rx);
        let s1 = server.clone();
        let s3 = server.clone();
        let s2 = server.clone();
        tokio::spawn(async move {
            Server::process_letter(rx1, s1).await;
        });
        tokio::spawn(async move {
            Server::process_letter(rx2, s2).await;
        });
        tokio::spawn(async move {
            Server::process_letter(rx3, s3).await;
        });
    }

    /// Recibe una carta por la cola de mensajes y se encarga de procesar y llevar a cabo la acción correspondiente.
    /// Ejecuta las acciones correspondientes para cada mensaje, en particular para disconect solo elimina al usuario
    /// de la lista de usuarios (y grupos).
    async fn process_letter(rx: Receiverr<Letter<Vec<u8>>>, server: Arc<Server>) {
        while let Ok(msg) = rx.recv().await  {
            procces_letter_aux(msg, server.clone()).await;
        }
    }

    async fn build_msg_sender_to_client<T: AsyncWrite + Unpin>(mut rx: Receiver<Vec<u8>>, mut writer: T) {
        
        while let Some(msg) = rx.recv().await {

            let Ok(_) = writer.write_all(&msg).await else {
                return ;
            };
        }
    }


    ///Función que genera el receptor de mensajes del cliente, si hay algún mensaje se lee
    /// y se envia al procesador global de mensajes, si se sobrepasa el limite del búffer
    /// o hay algun error de lectura se cierra la conexión, en caso de mensaje inválido
    /// o de mensaje de desconexión termina el proceso y envia la carta correspondiente
    /// a el procesador de mensajes, como la identificación se produce previamente a
    /// el inicio de este proceso, si un cliente reenvia el mensaje de identificación
    /// se lo desconecta.
    async fn build_msg_client_processor<T: AsyncRead + Unpin>(user_tx: Sender<Vec<u8>>, 
                                                             username: String,
                                                             mut reader: FramedRead<T, LinesCodec>,
                                                            global_tx: Senderr<Letter<Vec<u8>>>) {
        while let Some(result) = reader.next().await {
            match result {
                Ok(msg) => {
                    let Ok(message): Result<TypeReciveMessages, serde_json::Error> = serde_json::from_str(&msg) else {
                        let message = generate_not_valid_msg().expect("NO fue posible generar el mensaje");
                        let diconect_msg = TypeReciveMessages::Disconect { type_msg: String::from("DISCONNECT") } ;
                        let letter = Letter::new(username, diconect_msg, user_tx.clone());
                        let Ok(_) = global_tx.send(letter).await else {
                            return ;
                        };
                        let Ok(_) = user_tx.send(message).await else {
                            return ;
                        };
                        break;
                    };
                    if let TypeReciveMessages::Disconect { type_msg: _ } = message  {
                        let message = TypeReciveMessages::Disconect { type_msg: String::from("DISCONECT") };
                        let letter = Letter::new(username, message, user_tx);
                        let Ok(_) = global_tx.send(letter).await else {
                            return ;
                        };
                        break;
                    };
                    if let TypeReciveMessages::Identify { type_msg: _, username } = message {
                        let message = generate_not_valid_msg().expect("NO fue posible generar el mensaje");
                        let diconect_msg = TypeReciveMessages::Disconect { type_msg: String::from("DISCONNECT") } ;
                        let letter = Letter::new(username, diconect_msg, user_tx.clone());
                        let Ok(_) = global_tx.send(letter).await else {
                            return ;
                        };
                        let Ok(_) = user_tx.send(message).await else {
                            return ;
                        };
                        break;
                    }
                    let letter = Letter::new(username.clone(), message, user_tx.clone());
                    let Ok(_) = global_tx.send(letter).await else {
                        return ;
                    };
                    
                }
                Err(_) => {
                    let diconect_msg = TypeReciveMessages::Disconect { type_msg: String::from("DISCONNECT") };
                    let letter = Letter::new(username, diconect_msg, user_tx);
                    let Ok(_) = global_tx.send(letter).await else {
                        return ;
                    };
                    break;
                }
            }
        }

    }

}

    

    
    
    


#[cfg(test)]
mod test {
    
}
