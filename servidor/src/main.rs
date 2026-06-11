use servidor::{server::Server, view};

#[tokio::main]
async fn main() {
    let port = view::get_port();
    let server = Server::new(port);
    server.run().await;
}
