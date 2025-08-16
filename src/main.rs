use std::time::Duration;
use tokio;

#[tokio::main]
async fn main() {
    println!("=== Example 1: Sequential Execution ===");
    println!("Hello world");
    say_hello_async().await; // Wait here until the function completes
    println!("Hello world 2"); // Only printed after say_hello_async() finishes

    println!("\n=== Example 2: Parallel Execution ===");
    let task1 = say_hello_async();
    let task2 = say_hello_async();

    // Both tasks start in parallel, but we wait for both to complete
    let (result1, result2) = tokio::join!(task1, task2);
    println!("Both tasks are finished!");

    println!("\n=== Example 3: Future Pattern (like in Dart) ===");
    let future = say_hello_async();
    println!("Future created, but not executed yet");
    future.await; // Here the future is executed and we wait for completion
    println!("Future is completed!");
}

async fn say_hello_async() {
    println!("  [say_hello_async] Starting...");
    tokio::time::sleep(Duration::from_secs(2)).await;
    println!("  [say_hello_async] Finished after 2 seconds!");
}
