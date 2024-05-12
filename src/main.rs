// How to import functions from other files in Rust
// Answer: use the mod keyword to import the file and then call the function
// e.g. mod tcp; use tcp::{start_server};
mod tcp;
use tcp::start_server;

#[tokio::main]
async fn main() {
    start_server().await;
}
