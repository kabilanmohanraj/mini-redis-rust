// server-side script
use std::{time::Duration};
use tokio::{net::{TcpListener, TcpStream}, time::sleep};
use mini_redis::{Connection, Frame};

#[tokio::main]
async fn main() {
    // initialize the TcpListener object
    init_listener().await;
}

async fn init_listener() {
    let mut retry_attempts = 5; // do not assume that the network is stable

    match TcpListener::bind("127.0.0.1:6379").await {
        Ok(listener) => {
            println!("Listening...");
            loop {
                // binding done, listen to requests
                match listener.accept().await {
                    Ok((tcp_stream, socket_address)) => {
                        println!("[SUCCESS] New client: {:?}, information received: {:?}", socket_address, tcp_stream);

                        // like go routines - spawn asynchronous tasks for each new connection
                        // and continue execution of the program
                        tokio::spawn( async move {
                                process_query(tcp_stream).await;
                            }
                        );

                    },
                    Err(e) => {
                        // connection retry logic - to handle transient network errors,
                        // where the server has to accept new connections even after temporary failures
                        
                        retry_attempts -= 1;
                        if retry_attempts < 0 {
                            println!("[FAIL] Exceeded maximum retry attempts... Exiting server...");
                            return;
                        }
                        println!("[FAIL] Couldn't get client: {:?}... Retrying...", e);
                        sleep(Duration::from_secs(2)).await;
                    },
                }
            }
        },
        Err(e) => {
            println!("[FAIL] Failed to bind: {:?}", e);
        }
    }
}

async fn process_query(tcp_stream: TcpStream) -> () {
    use std::collections::HashMap;
    use mini_redis::Command::{self, Set, Get};

    // the HashMap acts as our key-value store
    let mut db: HashMap<String, Vec<u8>> = HashMap::new();

    // TCP connections managed by the `Connection` object from mini-redis
    let mut connection = Connection::new(tcp_stream);
    
    // keep the current connection alive
    // TODO: Implement connection timeout here
    loop {
        // INFO: process the input from client (rewrite `match` to `if let`)
        // loop { match } -> while let || match (care only about subset of arms)  -> if let
        if let Some(frame) = connection.read_frame().await.unwrap() {
            println!("[SUCCESS] Got new frame: {:?}", frame);

            // extract client command from the frame
            let command = Command::from_frame(frame);
            match command {
                Ok(cmd_temp) => {
                    println!(" {:?} ",cmd_temp);
                    let response_frame = match cmd_temp {
                        Set(cmd) => {
                            db.insert(cmd.key().to_string(), cmd.value().to_vec()); // store the value as a byte array
                            Frame::Simple("SUCCESS".to_string())
                        }
                        Get(cmd) => {
                            if let Some(value) = db.get(cmd.key()) {
                                Frame::Bulk(value.clone().into())
                            } else {
                                Frame::Null
                            }
                        }
                        cmd => {
                            Frame::Error("[FAIL] Method Unimplemented".to_string())
                        }
                    };

                    // // send response to the client
                    // construct response frame
                    // let mut response_frame = Frame::Error("Method Unimplemented".to_string());

                    // let temp_response_frame = Frame::Integer(5);
                    // write frame to connection buffer
                    match connection.write_frame(&response_frame).await {
                        Ok(()) => println!("[SUCCESS] Buffer write"),
                        Err(e) => println!("[FAIL] Buffer Write e: {:?}", e),
                    }
                },
                Err(_) => {
                    Frame::Error("[FAIL] Error parsing frame".to_string());
                },
            };
        } 
    }
}

