use std::fs;

use serde::Deserialize;

use crate::tasks::task::Task;

#[derive(Deserialize)]
struct TaskFile {
    tasks: Vec<Task>,
}

pub fn parse_tasks(task_file_path: &str) -> Vec<Task> {
    let file_content = fs::read_to_string(task_file_path).unwrap();

    let file: TaskFile = toml::from_str(&file_content).unwrap();

    file.tasks
}
