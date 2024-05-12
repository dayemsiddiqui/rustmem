// Import the tcp module a
use tcp::start_server;

#[tokio::main]
async fn main() {
    tcp::start_server().await;
}
