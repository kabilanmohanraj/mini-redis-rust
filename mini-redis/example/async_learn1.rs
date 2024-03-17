use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let f1 = test_async_function();
    f1.await; 
    // let f2 = test_async_function();
    println!("Hello, world again!");

    // let mut handles = vec![];
    // handles.push(f1);
    // handles.push(f2);

    

}

async fn test_async_function() -> () {
    println!("async call function");
    sleep(Duration::from_millis(1000)).await;
}