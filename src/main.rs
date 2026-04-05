use tasks::task_parser::parse_tasks;
use tasks::task_runner::run_tasks;
use tokio::time::{Duration, sleep};

mod tasks;

#[tokio::main]
async fn main() {
    let task_configs = parse_tasks("tasks.toml");

    let handles = run_tasks(task_configs);
    for handle in handles {
        match handle.await {
            Ok(output) => {
                dbg!(output);
            }
            Err(e) => eprintln!("Task failed or was cancelled: {:?}", e),
        }
    }
}
