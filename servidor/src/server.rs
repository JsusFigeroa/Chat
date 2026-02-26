use crate::menssages::{self, Menssages};
use crate::user::User;
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use std::net::{Ipv4Addr, SocketAddrV4};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::{TcpListener, TcpStream};
use tokio::runtime::Runtime;
use std::sync::{Arc};
use tokio::sync::Mutex;

pub struct Server {
    users: HashMap<String, User>,
    numberUsers: usize,
    address: Ipv4Addr,
    port: u16,
}

impl Server {
    pub fn new() -> Server {
        let users: HashMap<String, User> = HashMap::new();
        let numberUsers = 0;
        let address = Ipv4Addr::new(127, 0, 0, 1);
        let port = 4444;

        Server {
            users,
            numberUsers,
            address,
            port,
        }
    }    

}

pub async  fn runServer(){
    let server = Arc::new(Mutex::new(Server::new()));
    let clone_server = Arc::clone(&server);
    tokio::spawn(async move {
        getConections(clone_server).await;
    });
}

pub async fn getConections(clone_sever: Arc<Mutex<Server>>) {
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
            tokio::spawn(async move { processConection(socket, server_for_client).await });
        }
}

pub async fn processConection(socket: TcpStream, server: Arc<Mutex<Server>>) {
        let mut lector = BufReader::new(socket);
        let mut lectura = String::new();

            let bytes_leidos = lector.read_line(&mut lectura).await.unwrap();

            if bytes_leidos == 0 {
                lector.shutdown();
                return;
            }
            let mensagge: Menssages;
            match serde_json::from_str::<Menssages>(&lectura) {
                Ok(text) => {
                    mensagge = text;
                }
                Err(_) => {
                    lector.shutdown();
                    return;
                }
            }
            if let Menssages::Identify { type_msg, username } = mensagge {
                let new_user = User::new(username, lector);
                let name = new_user.name.clone();
                let mut locked_server = server.lock().await;
                locked_server.users.insert(name, new_user);
            }
    }

pub async fn recibeMensaje(socket: TcpStream) -> Menssages {}
