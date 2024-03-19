// server-side script
use std::{time::Duration};
use std::collections::HashMap;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::sync::{Arc, RwLock};
use bytes::Bytes;
use tokio::{net::{TcpListener, TcpStream}, time::sleep};
use mini_redis::{Connection, Frame};

// type KVStore = Arc<RwLock<HashMap<String, Bytes>>>;
type ShardedKVStore = Arc<Vec<RwLock<HashMap<String, Bytes>>>>; // a vector of RwLock protected HashMaps

#[tokio::main]
async fn main() {
    // initialize the TcpListener object
    init_listener().await;
}

async fn init_listener() {

    // // shared state
    // the HashMap acts as our key-value store
    // Shard (using hash) the KVStore with Mutex/ RwLock wrapping
    let num_shards = 16;
    let mut db: ShardedKVStore = init_new_shard_db(num_shards).await;


    let mut retry_attempts = 5; // we do not assume that the network is stable

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
                        let db_clone = Arc::clone(&db);
                        tokio::spawn( async move {
                                process_query(tcp_stream, db_clone).await;
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

async fn init_new_shard_db(num_shards: usize) -> ShardedKVStore {
    let mut db_temp = Vec::with_capacity(num_shards);

    for _ in 0..num_shards {
        let hm: RwLock<HashMap<String, Bytes>> = RwLock::new(HashMap::new());
        db_temp.push(hm);
    }

    Arc::new(db_temp)
}

async fn process_query(tcp_stream: TcpStream, db: ShardedKVStore) -> () {
    use mini_redis::Command::{self, Set, Get};

    let peer_address = tcp_stream.peer_addr().unwrap();
    // TCP connections managed by the `Connection` object from mini-redis
    let mut connection = Connection::new(tcp_stream);
    
    // keep the current connection alive
    // TODO: Implement connection timeout here
    loop {
        // INFO: process the input from client (rewrite `match` to `if let`)
        // loop { match } -> while let || match (care only about subset of arms)  -> if let
        if let Ok(Some(frame)) = connection.read_frame().await {
            println!("[SUCCESS] Got new frame: {:?}", frame);

            // extract client command from the frame
            let command = Command::from_frame(frame);
            match command {
                Ok(cmd_temp) => {
                    println!(" {:?} ",cmd_temp);
                    let response_frame = match cmd_temp {
                        Set(cmd) => {
                            // compute partition ID
                            let hash_value = get_partition(cmd.key()).await;
                            let partition = (hash_value as usize)%db.len();
                            let mut write_guard = db[partition].write().unwrap();

                            write_guard.insert(cmd.key().to_string(), cmd.value().clone()); // store the value as a byte array

                            Frame::Simple("SUCCESS".to_string())
                        }
                        Get(cmd) => {

                            let hash_value = get_partition(cmd.key()).await;
                            let partition = (hash_value as usize)%db.len();
                            let read_guard = db[partition].read().unwrap();

                            if let Some(value) = read_guard.get(cmd.key()) {
                                Frame::Bulk(value.clone())
                            } else {
                                Frame::Null
                            }
                        }
                        _ => {
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
        } else {
            // Handle disconnection or error without panicking, by breaking out of the loop
            println!("[INFO] Client {:?} disconnected or error reading frame", peer_address);
            break;
        }
    }
}

async fn get_partition(str_to_hash: &str) -> u64 {
    let mut hash_fn = DefaultHasher::new();
    str_to_hash.hash(&mut hash_fn);
    hash_fn.finish().into()
}