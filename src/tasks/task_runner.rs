use crate::tasks::{task::TaskOutput, task_config::TaskConfig};
use tokio::task::JoinSet;

pub async fn run_tasks(task_configs: Vec<TaskConfig>) -> Vec<TaskOutput> {
    let mut set = JoinSet::new();

    for task_config in task_configs {
        let task = task_config.as_task();
        set.spawn(async move { task.execute().await });
    }

    let mut outputs = Vec::with_capacity(set.len());

    while let Some(result) = set.join_next().await {
        match result {
            Ok(output) => outputs.push(output),
            Err(e) => eprintln!("Task failed: {e}"),
        }
    }

    outputs
}
