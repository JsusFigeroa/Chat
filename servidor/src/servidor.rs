use std::collections::HashMap;
use std::net::{Ipv4Addr, SocketAddrV4};
use crate::usuario::Usuario;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncBufReadExt, BufReader};
use crate::menssages::Menssages;
use serde_json;
use serde::{Serialize, Deserialize};

pub struct Servidor{
    users: HashMap<String, Usuario>,
    numberUsers: usize,
    address: Ipv4Addr,
    port: u16,
}

impl Servidor{
    pub fn new() -> Servidor{
        let users: HashMap<String, Usuario> = HashMap::new();
        let numberUsers = 0;
        let address = Ipv4Addr::new(127, 0, 0, 1);
        let port = 4444;

        Servidor {users, numberUsers, address, port}
    }

    pub async fn getConections(&self){
        let listener  = TcpListener::bind(SocketAddrV4::new(self.address, self.port)).await.unwrap();
        loop {
            let (socket, _) = listener.accept().await.unwrap();
            tokio::spawn(async move {
                processConection(socket).await
            });
        }
        

    }

    
}

pub async  fn processConection(socket: TcpStream){
    let mut lector = BufReader::new(socket);
    let mut lectura = String::new();
    
    loop {
        let bytes_leidos = lector.read_line(&mut lectura).await.unwrap_or_else(|_|);

        if bytes_leidos == 0 {
            break;
        }
    
        match serde_json::from_str::<Menssages>(&lectura) {
            Ok(texto) => {
                
            }
            Err(_) => break
        }

    }
}
pub async fn recibeMensaje(socket: TcpStream) -> Menssages{
    
}