use tokio::time::{Duration, sleep, Instant};
use std::future::Future;

async fn task_a() -> Result<u32, &'static str> {
    sleep(Duration::from_secs(1)).await;
    Ok(100)
}

async fn task_b() -> Result<u32, &'static str> {
    sleep(Duration::from_secs(2)).await;
    Err("Oops, Task B failed")
}

async fn task_c() -> Result<u32, &'static str> {
    sleep(Duration::from_secs(1)).await;
    Ok(200)
}

async fn task_d() -> Result<u32, &'static str> {
    sleep(Duration::from_secs(1)).await;
    Ok(300)
}

// Future pattern similar to Dart: function that returns a Future
fn create_future_task(value: u32) -> impl Future<Output = Result<u32, &'static str>> {
    async move {
        sleep(Duration::from_millis(500)).await;
        Ok(value * 2)
    }
}

#[tokio::main]
async fn main() {
    println!("=== Async/Await Basics in Rust ===\n");

    // 1. SEQUENTIAL EXECUTION
    println!("1. Sequential Execution:");
    let start = Instant::now();
    
    let result_a = task_a().await;
    let result_c = task_c().await;
    
    let sequential_time = start.elapsed();
    println!("   Task A: {:?}", result_a);
    println!("   Task C: {:?}", result_c);
    println!("   Total time (sequential): {:?}\n", sequential_time);

    // 2. PARALLEL EXECUTION
    println!("2. Parallel Execution:");
    let start = Instant::now();
    
    let (result_a_parallel, result_c_parallel) = tokio::join!(task_a(), task_c());
    
    let parallel_time = start.elapsed();
    println!("   Task A: {:?}", result_a_parallel);
    println!("   Task C: {:?}", result_c_parallel);
    println!("   Total time (parallel): {:?}\n", parallel_time);

    // 3. FUTURE PATTERNS (similar to Dart)
    println!("3. Future Patterns:");
    let start = Instant::now();
    
    let future1 = create_future_task(10);
    let future2 = create_future_task(20);
    
    // Futures are only executed when awaited
    let result1 = future1.await;
    let result2 = future2.await;
    
    let future_time = start.elapsed();
    println!("   Future 1 (10 * 2): {:?}", result1);
    println!("   Future 2 (20 * 2): {:?}", result2);
    println!("   Total time (futures): {:?}\n", future_time);

    // 4. ERROR HANDLING with try_join!
    println!("4. Error Handling - try_join! (fail-fast):");
    let start = Instant::now();
    
    match tokio::try_join!(task_a(), task_b()) {
        Ok((a, b)) => println!("   Success: {a}, {b}"),
        Err(e) => println!("   Error: {e}"),
    }
    
    let try_join_time = start.elapsed();
    println!("   Total time (try_join!): {:?}\n", try_join_time);

    // 5. ERROR HANDLING with join! (wait-for-all)
    println!("\n5. Error Handling - join! (wait for all):");
    let start = Instant::now();
    
    let (res_a, res_b) = tokio::join!(task_a(), task_b());
    
    let join_time = start.elapsed();
    println!("   Task A Result: {:?}", res_a);
    println!("   Task B Result: {:?}", res_b);
    println!("   Total time (join!): {:?}\n", join_time);

    // 6. PARALLEL EXECUTION with multiple tasks
    println!("\n6. Parallel Execution with Multiple Tasks:");
    let start = Instant::now();
    
    let (res_a, res_c, res_d) = tokio::join!(task_a(), task_c(), task_d());
    
    let multi_parallel_time = start.elapsed();
    println!("   Task A: {:?}", res_a);
    println!("   Task C: {:?}", res_c);
    println!("   Task D: {:?}", res_d);
    println!("   Total time (3 tasks parallel): {:?}", multi_parallel_time);
}
