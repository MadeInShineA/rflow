use crate::tasks::task::Task;
use tokio::task::JoinSet;

pub async fn run_tasks(tasks: Vec<Task>) {
    let mut set = JoinSet::new();

    for task in tasks {
        set.spawn(async move { task.execute().await });
    }

    while let Some(result) = set.join_next().await {
        match result {
            Ok(output) => {
                println!("Got some output: {output:?}");
            }
            Err(e) => eprintln!("Task failed: {e}"),
        }
    }
}
