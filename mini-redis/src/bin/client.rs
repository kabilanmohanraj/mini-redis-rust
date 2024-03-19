use std::string;

// client-side script
use mini_redis::client;

#[tokio::main]
async fn main() {

    let redis_addr: String = "localhost:6379".to_string();

    // create_client(redis_addr);
    // TODO: retry logic for failed requests
    match client::connect(redis_addr).await {
        Ok(mut client) => {
            // client
            println!("[SUCCESS] Connected to server");
            
            match client.set("Hello", "World!".into()).await {
                Ok(()) => println!("[SUCCESS] SET"),
                Err(e) => println!("[FAIL] SET error: {:?}", e),
            };

            match client.get("Hello").await {
                Ok(Some(data)) => println!("[SUCCESS] GET output: {:?}", data),
                Ok(None) => println!("[SUCCESS] GET output empty"),
                Err(e) => println!("[FAIL] GET error: {:?}", e),
            };

            match client.set("Hello", "brother!".into()).await {
                Ok(()) => println!("[SUCCESS] SET"),
                Err(e) => println!("[FAIL] SET error: {:?}", e),
            };

            match client.get("Hello").await {
                Ok(Some(data)) => println!("[SUCCESS] GET output: {:?}", data),
                Ok(None) => println!("[SUCCESS] GET output empty"),
                Err(e) => println!("[FAIL] GET error: {:?}", e),
            };
            
            let mut client2 = client::connect("localhost:6379").await.unwrap();
            match client2.set("Hello", "people".into()).await {
                Ok(()) => println!("[SUCCESS] SET"),
                Err(e) => println!("[FAIL] SET error: {:?}", e),
            };

            let mut client3 = client::connect("localhost:6379").await.unwrap();
            match client3.get("Hello").await {
                Ok(Some(data)) => println!("[SUCCESS] GET output: {:?}", data),
                Ok(None) => println!("[SUCCESS] GET output empty"),
                Err(e) => println!("[FAIL] GET error: {:?}", e),
            };
        }
        Err(_) => panic!("[FAIL] Failed to establish connection"),
    };
}

async fn create_client(redis_addr: String) {

}

async fn handle_command() {

}