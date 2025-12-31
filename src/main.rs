use crate::async1::feature::future_main;
use crate::async1::tasks::task_main;

mod async1;

#[tokio::main]
async fn main() {
    if false {
        // Example 01: execute async tasks
        task_main().await;
    }

    // Example 02: some basic future aspect
    future_main().await
}
