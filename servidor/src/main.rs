use servidor::server::{self, Server};

#[tokio::main]
async fn main() {
    Server::run().await;
}


