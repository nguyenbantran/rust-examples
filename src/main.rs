use crate::async1::tasks::task_main;

mod async1;

#[tokio::main]
async fn main() {
    let _ = task_main().await;
}
