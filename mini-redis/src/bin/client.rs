use mini_redis::{client::{self, connect}, Result};

#[tokio::main]
async fn main() {
    let client = match client::connect("localhost:6379").await {
        Ok(client) => println!("Connected to server"),
        Err(_) => panic!("failed to establish connection"),
    };
}