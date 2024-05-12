// Import the tcp module a
mod resp_parser;
mod tcp;

#[tokio::main]
async fn main() {
    tcp::start_server().await;
}
