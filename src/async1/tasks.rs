use std::thread;
use std::time::{Duration, Instant};
use tokio::time::sleep;

pub(crate) async fn task_main() {
    let start_time = Instant::now();
    let coffee_mug_step = prep_coffee_mug();
    let coffee_step = make_coffee();
    let toast_step = make_toast();

    tokio::join!(coffee_mug_step, coffee_step, toast_step);
    let elapsed_time = start_time.elapsed();
    println!("It took: {} seconds", elapsed_time.as_secs());
}

async fn prep_coffee_mug() {
    sleep(Duration::from_millis(100)).await;
    println!("[1]Pouring milk...");
    thread::sleep(Duration::from_secs(1));
    println!("[1]Milk poured.");
    println!("[1]Putting instant coffee...");
    thread::sleep(Duration::from_secs(1));
    println!("[1]Instant coffee put.");
}

async fn make_coffee() {
    println!("[2]boiling kettle...");
    sleep(Duration::from_secs(2)).await;
    println!("[2]kettle boiled.");
    println!("[2]pouring boiled water...");
    thread::sleep(Duration::from_secs(1));
    println!("[2]boiled water poured.");
}

async fn make_toast() {
    println!("[3]putting bread in toaster...");
    sleep(Duration::from_secs(2)).await;
    println!("[3]bread toasted.");
    println!("[3]buttering toasted bread...");
    thread::sleep(Duration::from_secs(1));
    println!("[3]toasted bread buttered.");
}