use std::time::Duration;
use tokio;

#[tokio::main]
async fn main() {
    println!("=== Beispiel 1: Sequentielle Ausführung ===");
    println!("Hello world");
    say_hello_async().await; // Wartet hier bis die Funktion fertig ist
    println!("Hello world 2"); // Wird erst nach say_hello_async() ausgegeben

    println!("\n=== Beispiel 2: Parallele Ausführung ===");
    let task1 = say_hello_async();
    let task2 = say_hello_async();

    // Beide Tasks starten parallel, aber wir warten auf beide
    let (result1, result2) = tokio::join!(task1, task2);
    println!("Beide Tasks sind fertig!");

    println!("\n=== Beispiel 3: Mit Future (wie in Dart) ===");
    let future = say_hello_async();
    println!("Future erstellt, aber noch nicht ausgeführt");
    future.await; // Hier wird das Future ausgeführt und gewartet
    println!("Future ist abgeschlossen!");
}

async fn say_hello_async() {
    println!("  [say_hello_async] Starte...");
    tokio::time::sleep(Duration::from_secs(2)).await;
    println!("  [say_hello_async] Fertig nach 2 Sekunden!");
}
