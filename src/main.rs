use tasks::task_parser::parse_tasks;
use tasks::task_runner::run_tasks;
use tokio::time::{Duration, sleep};

mod tasks;

#[tokio::main]
async fn main() {
    let tasks = parse_tasks("tasks.toml");

    let _ = run_tasks(tasks).await;
}
