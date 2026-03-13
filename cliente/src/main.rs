use cliente::controller;
#[tokio::main]
async fn main() {
    controller::start().await;

    std::process::exit(0);
}
