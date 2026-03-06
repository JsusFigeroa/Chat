use std::{sync::Arc};
use servidor::server::Server;
use tokio::sync::Mutex;
use servidor::view;

#[tokio::main]
async fn main() {
    
    let (ip, port) = view::get_addr();
    let server = Server::new(ip, port);
    let server_clone = Arc::new(Mutex::new(server));
    Server::run(server_clone).await;
}


