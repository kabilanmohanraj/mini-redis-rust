// server-side script
use tokio::net::{tcp, TcpListener, TcpStream};
use mini_redis::{Connection, Frame};

#[tokio::main]
async fn main() {
    // listen to TCP connections
    init_listener().await;
}

async fn init_listener() {
    let mut retry_attempts = 5; // do not assume that the network is stable

    // listen to connection and retry
    loop {
        match TcpListener::bind("127.0.0.1:6379").await {
            Ok(listener) => {
                // binding done, listen to requests
                match listener.accept().await {
                    Ok((tcp_stream, socket_address)) => {
                        println!("new client: {:?}, information received: {:?}", socket_address, tcp_stream);
                    },
                    Err(e) => {
                        retry_attempts -= 1;
                        if retry_attempts < 0 {
                            println!("Exceeded maximum retry attempts... Exiting server...");
                            return;
                        }
                        println!("couldn't get client: {:?}... Retrying...", e);
                    },
                }

            },
            Err(e) => println!("couldn't get client: {:?}", e),
        }
    }
}