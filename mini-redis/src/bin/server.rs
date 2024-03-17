// server-side script
use tokio::net::{TcpListener, TcpStream};
use mini_redis::{Connection, Frame};

#[tokio::main]
async fn main() {
    // listen to TCP connections
    ini_listener().await;
}

async fn ini_listener() {
    // create TCP listener
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    // listen to connection
    loop {
        match listener.accept().await {
            Ok((_socket, addr)) => println!("new client: {:?}, information sent: {:?}", addr, _socket),
            Err(e) => println!("couldn't get client: {:?}", e),
        }
    }
}