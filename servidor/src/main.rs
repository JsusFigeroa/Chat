use servidor::server::{Server};

#[tokio::main]
async fn main() {
    Server::run().await;
}


