use std::net::TcpStream;
pub struct Usuario {
    name: String,
    id: usize,
    socket: TcpStream,
}

impl Usuario {
    pub fn new(name: String, id: usize, socket: TcpStream) -> Usuario{
        Usuario {name, id, socket}
    }
}