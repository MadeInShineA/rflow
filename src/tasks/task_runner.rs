use tokio::task::JoinHandle;

use crate::tasks::task_config::TaskConfig;

pub fn run_tasks(task_configs: Vec<TaskConfig>) -> Vec<JoinHandle<()>> {
    let mut res = Vec::new();
    for task_config in task_configs {
        let task = task_config.as_task();

        let handle = tokio::spawn(async move {
            task.execute().await;
        });

        res.push(handle);
    }

    res
}
