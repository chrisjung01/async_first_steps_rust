#[tokio::main]
async fn main() {
    let handler = say_hello_async();

    handler.await;
}

async fn say_hello_async() {
    println!("Hello async");
}
