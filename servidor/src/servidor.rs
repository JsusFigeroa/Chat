use std::collections::HashMap;
use std::net::{Ipv4Addr, TcpStream};
use std::net::{TcpListener, SocketAddrV4};
use crate::usuario::State;
use crate::usuario::Usuario;

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

    pub fn getConections(&self){
        let listener  = TcpListener::bind(SocketAddrV4::new(self.address, self.port)).unwrap_or_else(|_| panic!());
        let mut conections: Vec<TcpStream> = Vec::new();
        for stream in listener.incoming(){
            match stream {
                Ok(stream) => conections.push(stream),
                Err(_) => eprint!("No fue posible conectarse")
            }
        } 

    }
}