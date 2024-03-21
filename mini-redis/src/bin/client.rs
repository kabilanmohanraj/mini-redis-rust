use std::time::Duration;

// client-side script
use mini_redis::client;
use tokio::{sync::mpsc::{self, Receiver}, time::sleep};

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
            start_client(sender).await;
        }
    );

    let client2 = tokio::spawn(
        async move {
            start_client(sender_clone).await;
        }
    );

    // await on all processes (the tasks have already spawned, but we await their termination here)
    let (clerk_ret, client1_ret, client2_ret) = tokio::join!(clerk, client1, client2);
    // or we can do this
    // clerk.await;
    // client1.await;
    // client2.await;
    
}

async fn start_client(request_sender: mpsc::Sender<Op>) {

    // TODO: match statement here to support other operations as well

    let op = Set { key: "Hello".to_string(), value: "World!".into() };
    request_sender.send(op).await.unwrap();

    let op = Get { key: "Hello".to_string() };
    request_sender.send(op).await.unwrap();

    
    // // TODO: retry logic for failed requests
    // match client::connect(redis_addr).await {
    //     Ok(mut client) => {
    //         // client
    //         println!("[SUCCESS] Connected to server");
            
    //         let query1 = tokio::spawn(async {
    //             // match client.set("Hello", "World!".into()).await {
    //             //     Ok(()) => println!("[SUCCESS] SET"),
    //             //     Err(e) => println!("[FAIL] SET error: {:?}", e),
    //             // }
    //         });

    //         match client.get("Hello").await {
    //             Ok(Some(data)) => println!("[SUCCESS] GET output: {:?}", data),
    //             Ok(None) => println!("[SUCCESS] GET output empty"),
    //             Err(e) => println!("[FAIL] GET error: {:?}", e),
    //         };

    //         // match client.set("Hello", "brother!".into()).await {
    //         //     Ok(()) => println!("[SUCCESS] SET"),
    //         //     Err(e) => println!("[FAIL] SET error: {:?}", e),
    //         // };

    //         // match client.get("Hello").await {
    //         //     Ok(Some(data)) => println!("[SUCCESS] GET output: {:?}", data),
    //         //     Ok(None) => println!("[SUCCESS] GET output empty"),
    //         //     Err(e) => println!("[FAIL] GET error: {:?}", e),
    //         // };
            
    //         // let mut client2 = client::connect("localhost:6379").await.unwrap();
    //         // match client2.set("Hello", "people".into()).await {
    //         //     Ok(()) => println!("[SUCCESS] SET"),
    //         //     Err(e) => println!("[FAIL] SET error: {:?}", e),
    //         // };

    //         // let mut client3 = client::connect("localhost:6379").await.unwrap();
    //         // match client3.get("Hello").await {
    //         //     Ok(Some(data)) => println!("[SUCCESS] GET output: {:?}", data),
    //         //     Ok(None) => println!("[SUCCESS] GET output empty"),
    //         //     Err(e) => println!("[FAIL] GET error: {:?}", e),
    //         // };
    //     }
    //     Err(_) => panic!("[FAIL] Failed to establish connection"),
    // };

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
                        Set{key, value} => {
                            match clerk.set(&key, value).await {
                                Ok(()) => println!("[SUCCESS] SET"),
                                Err(e) => println!("[FAIL] SET error: {:?}", e),
                            }
                        },
                        Get{key} => {
                            match clerk.get(&key).await {
                                Ok(Some(data)) => println!("[SUCCESS] GET output: {:?}", data),
                                Ok(None) => println!("[SUCCESS] GET output empty"),
                                Err(e) => println!("[FAIL] GET error: {:?}", e),
                            }
                        },
                    }
                };
        }
        Err(_) => panic!("[FAIL] Clerk failed to establish connection"),
    };

    // sleep(Duration::from_secs(10)).await;
}