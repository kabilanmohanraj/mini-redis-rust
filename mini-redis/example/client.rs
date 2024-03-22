use bytes::Bytes;
// client-side script
use mini_redis::{client, Frame};
use std::time::Duration;
use tokio::{sync::{mpsc::{self, Receiver}, oneshot}, time::sleep};

mod common;
use common::types::Op::{self, Set, Get};

#[tokio::main]
async fn main() {

    let redis_addr: String = "localhost:6379".to_string();
    let (sender, receiver) = mpsc::channel(64);
    let sender_clone = sender.clone();

    let clerk = tokio::spawn(
        async move {
            start_clerk(redis_addr, receiver).await
        }
    );

    let client1 = tokio::spawn(
        async move {
            run_client(sender).await;
        }
    );

    let client2 = tokio::spawn(
        async move {
            run_client(sender_clone).await;
        }
    );

    // await on all processes (the tasks have already spawned, but we await their termination here)
    let (_clerk_ret, 
         _client1_ret, 
         _client2_ret) = tokio::join!(clerk, client1, client2);
    // or we can do this
    // clerk.await;
    // client1.await;
    // client2.await;
    
}

async fn run_client(request_sender: mpsc::Sender<Op>) {

    let (tx, mut rx) = oneshot::channel();
    let op = Set { 
        key: "Hello".to_string(), 
        value: "World!".into(),
        reply_channel: tx
    };
    request_sender.send(op).await.unwrap();
    match rx.await.unwrap() {
        Ok(()) => println!("[SUCCESS] SET"),
        Err(e) => println!("[FAIL] SET error: {:?}", e),
    }

    let (tx1, mut rx1) = oneshot::channel();
    let op = Get { 
        key: "Hello".to_string(),
        reply_channel: tx1
    };
    request_sender.send(op).await.unwrap();
    match rx1.await.unwrap() {
        Ok(Some(data)) => println!("[SUCCESS] GET output: {:?}", data),
        Ok(None) => println!("[SUCCESS] GET output empty"),
        Err(e) => println!("[FAIL] GET error: {:?}", e),
    }

}

async fn start_clerk(redis_addr: String, mut rx: Receiver<Op>) {
    match client::connect(redis_addr).await {
        Ok(mut clerk) => {
            println!("[SUCCESS] Clerk connected to server");
                // process client message
                // changed loop{ if let } to while let
                // reading from a closed channel returns `None`. when all sender objects go out of scope, the channel is closed.
                while let Some(op) = rx.recv().await {
                    match op {
                        Set{key, value, reply_channel} => {
                            // let resp = clerk.set(&key, value).await;
                            // println!("{:?}", resp.unwrap());

                            match clerk.set(&key, value).await {
                                Err(e) if e.to_string().contains("SUCCESS") => {
                                    match reply_channel.send(Ok(())) {
                                        Ok(_) => println!("[SUCCESS] clerk: SET response from server sent to client"),
                                        Err(e) => println!("[FAIL] clerk: SET failed to respond to client: {:?}", e),
                                    }
                                },
                                Err(e) => {
                                    match reply_channel.send(Err(e)) {
                                        Ok(_) => println!("[SUCCESS] clerk: SET response from server sent to client"),
                                        Err(e) => println!("[FAIL] clerk: SET failed to respond to client: {:?}", e),
                                    }
                                },
                                _ => {
                                    match reply_channel.send(Ok(())) {
                                        Ok(_) => println!("[SUCCESS] clerk: SET response from server sent to client"),
                                        Err(e) => println!("[FAIL] clerk: SET failed to respond to client: {:?}", e),
                                    }
                                }
                            }
                            
                        },
                        Get{key, reply_channel} => {
                            match reply_channel.send(clerk.get(&key).await) {
                                Ok(_) => println!("[SUCCESS] clerk: GET response from server sent to client"),
                                Err(e) => println!("[FAIL] clerk: GET failed to respond to client: {:?}", e),
                            }
                        },
                    }
                };
        }
        Err(_) => panic!("[FAIL] Clerk failed to establish connection"),
    };

    // sleep(Duration::from_secs(10)).await;
}