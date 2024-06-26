// Import a module defined in lib.rs
use crate::resp_parser::process_request;
use std::net::SocketAddr;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

pub async fn handle_client(mut stream: TcpStream) {
    println!("Accepted connection from {}", stream.peer_addr().unwrap());
    let mut buffer = [0; 1024];

    loop {
        match stream.read(&mut buffer).await {
            Ok(n) if n == 0 => {
                println!("Connection closed");
                break;
            }
            Ok(n) => {
                let request = String::from_utf8_lossy(&buffer[..n]);
                println!("Received: {}", request);
                let request_string = request.into_owned();
                let response = process_request(&request_string).as_bytes();
                stream.write_all(&response).await.unwrap();
            }
            Err(e) => {
                eprintln!("Failed to read from socket; err = {:?}", e);
                break;
            }
        }
    }
}

pub async fn start_server() {
    let address = "127.0.0.1:6379"
        .parse::<SocketAddr>()
        .expect("Failed to parse address");
    let listener = TcpListener::bind(&address)
        .await
        .expect("Failed to bind to address");

    println!("Redis server listening on {}", address);

    loop {
        let (stream, _) = listener
            .accept()
            .await
            .expect("Failed to accept connection");
        tokio::spawn(async move {
            handle_client(stream).await;
        });
    }
}