use bytes::Bytes;
// client-side script
use mini_redis::client;
use clap::{App, Arg, SubCommand};
use tokio::{sync::{mpsc::{self, Receiver}, oneshot}, task::JoinHandle};

mod common;
use common::types::Op::{self, Set, Get};

#[tokio::main]
async fn main() {

    let redis_addr: String = "localhost:6379".to_string();
    let (sender, receiver) = mpsc::channel(64);
    // let sender_clone = sender.clone();

    let clerk = tokio::spawn(
        async move {
            start_clerk(redis_addr, receiver).await
        }
    );


    // // [TODO:] ensure the clerk is running
    // let clerk = match client::connect(redis_addr).await {
    //     Ok(_) => {
    //         println!("Clerk service is already running.");
    //     },
    //     Err(_) => {
    //         let clerk = tokio::spawn(
    //             async move {
    //                 start_clerk(redis_addr, receiver).await
    //             }
    //         );
    //         clerk
    //     }
    // }

    // instantiate `clap` cli parameters
    let cmd_matches = App::new("Mini Redis CLI")
        .version("0.1.0")
        .author("Your Name")
        .about("Interacts with a mini Redis server")
        .subcommand(
            SubCommand::with_name("set")
                .about("Sets a value for a given key")
                .arg(Arg::with_name("KEY").help("The key to set").required(true))
                .arg(Arg::with_name("VALUE").help("The value to set").required(true)),
        )
        .subcommand(
            SubCommand::with_name("get")
                .about("Gets the value of a given key")
                .arg(Arg::with_name("KEY").help("The key to get").required(true)),
        )
        .get_matches();

    // Initialize the JoinHandle as None
    let client_handle: Option<JoinHandle<()>>;

    if let Some(matches) = cmd_matches.subcommand_matches("set") {
        let key = matches.value_of("KEY").unwrap().to_string();
        let value = matches.value_of("VALUE").unwrap().to_string();
        client_handle = Some(tokio::spawn(async move {
            run_set_command(sender.clone(), key, value.into()).await;
        }));
    } else if let Some(matches) = cmd_matches.subcommand_matches("get") {
        let key = matches.value_of("KEY").unwrap().to_string();
        client_handle = Some(tokio::spawn(async move {
            run_get_command(sender.clone(), key).await;
        }));
    } else {
        client_handle = None; // unimplemented command
    }

    // Use the JoinHandle outside the if let block
    if let Some(handle) = client_handle {
        // if a task was spawned, wait for it to complete
        let _ = handle.await;
        let _ = clerk.await;
        println!("[SUCCESS] Client task completed");
    } else {
        //  no task was spawned (unimplemented command)
        println!("[FAIL] No task was spawned... Command unimplemented");
    }
    
}

async fn run_get_command(request_sender: mpsc::Sender<Op>, key: String) {

    let (tx1, rx1) = oneshot::channel();
    let op = Get { 
        key: key,
        reply_channel: tx1
    };
    request_sender.send(op).await.unwrap();
    match rx1.await.unwrap() {
        Ok(Some(data)) => println!("[SUCCESS] GET output: {:?}", data),
        Ok(None) => println!("[SUCCESS] GET output empty"),
        Err(e) => println!("[FAIL] GET error: {:?}", e),
    }

}

async fn run_set_command(request_sender: mpsc::Sender<Op>, key: String, value: Bytes) {

    let (tx, rx) = oneshot::channel();
    let op = Set { 
        key: key, 
        value: value,
        reply_channel: tx
    };
    request_sender.send(op).await.unwrap();
    match rx.await.unwrap() {
        Ok(()) => println!("[SUCCESS] SET"),
        Err(e) => println!("[FAIL] SET error: {:?}", e),
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