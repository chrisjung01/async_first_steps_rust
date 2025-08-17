#[tokio::main]
async fn main() {
    let result = make_some_async_work().await;
    println!("Result: {:?}", result);
}

async fn make_some_async_work() -> Option<String> {
    println!("start async work");
    // Simulate some asynchronous work
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    println!("async work done");
    Some("Hello from async work".to_string())
}
