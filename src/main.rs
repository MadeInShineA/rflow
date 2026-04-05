use tasks::task_parser::parse_tasks;
use tasks::task_runner::run_tasks;
use tokio::time::{Duration, sleep};

mod tasks;

#[tokio::main]
async fn main() {
    let task_configs = parse_tasks("tasks.toml");

    let outputs = run_tasks(task_configs).await;

    for output in outputs {
        dbg!(output);
    }
}
