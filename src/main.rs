use tokio::time::{Duration, sleep};

async fn task_a() -> Result<u32, &'static str> {
    sleep(Duration::from_secs(1)).await;
    Ok(100)
}

async fn task_b() -> Result<u32, &'static str> {
    sleep(Duration::from_secs(2)).await;
    Err("Oops, Task B fehlgeschlagen")
}

#[tokio::main]
async fn main() {
    // if one task fails, the whole operation fails
    // try_join! returns the first error encountered
    // if all tasks succeed, it returns a tuple of all results
    match tokio::try_join!(task_a(), task_b()) {
        Ok((a, b)) => println!("Ergebnisse: {a}, {b}"),
        Err(e) => println!("Fehler: {e}"),
    }

    // join! waits for all tasks to complete, regardless of success or failure
    // it returns a tuple of all results, including errors
    let (res_a, res_b) = tokio::join!(task_a(), task_b());

    println!("Task A Ergebnis: {:?}", res_a);
    println!("Task B Ergebnis: {:?}", res_b);
}
