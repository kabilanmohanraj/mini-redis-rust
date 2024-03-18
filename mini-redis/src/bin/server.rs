// server-side script
use std::time::Duration;
use tokio::{net::{TcpListener, TcpStream}, time::sleep};
use mini_redis::{Connection, Frame};

#[tokio::main]
async fn main() {
    // initialize the TcpListener object
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
                        println!("[SUCCESS] New client: {:?}, information received: {:?}", socket_address, tcp_stream);

                        // TODO: make this asynchronous, as this is a blocking operation now
                        process_query(tcp_stream).await;

                    },
                    Err(e) => {
                        // connection retry logic - to handle transient network errors, where the server has to accept new connections 
                        // even after temporary failures
                        
                        retry_attempts -= 1;
                        if retry_attempts < 0 {
                            println!("[FAIL] Exceeded maximum retry attempts... Exiting server...");
                            return;
                        }
                        println!("[FAIL] Couldn't get client: {:?}... Retrying...", e);
                        sleep(Duration::from_secs(2)).await;
                    },
                }

            },
            Err(e) => {
                retry_attempts -= 1;
                if retry_attempts < 0 {
                    println!("[FAIL] Exceeded maximum retry attempts... Exiting server...");
                    return;
                }
                println!("[FAIL] Failed to bind: {:?}... Retrying...", e);
                sleep(Duration::from_secs(2)).await;
            }
        }
    }
}

async fn process_query(tcp_stream: TcpStream) {
    let mut connection = Connection::new(tcp_stream);
    
    // process the input from client (rewrite `match` to `if let`)
    if let Some(frame) = connection.read_frame().await.unwrap() {
        println!("[SUCCESS] Got new frame: {:?}", frame);

        // // send response to the client
        // construct response frame
        let mut response_frame = Frame::Error("Method Unimplemented".to_string());

        // let temp_response_frame = Frame::Integer(5);
        // write frame to connection buffer
        match connection.write_frame(&response_frame).await {
            Ok(()) => println!("[SUCCESS] Buffer write"),
            Err(e) => println!("[FAIL] Buffer Write e: {:?}", e),
        }

    } else {
        println!("[WARN] Empty frame received...");
    }
}

