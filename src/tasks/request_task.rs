use std::process::{Command, Output};

use serde::Deserialize;

use crate::tasks::{
    command_task::CommandTask,
    task::{Task, TaskId, TaskOutput},
};

#[derive(Deserialize)]
pub struct RequestTask {
    id: TaskId,
    dependencies: Vec<TaskId>,
    url: String,
}

impl Task for RequestTask {
    fn execute(&self) -> TaskOutput {
        if let Ok(response) = reqwest::blocking::get(&self.url) {
            return TaskOutput::Success(response.status().as_str().to_string());
        }

        TaskOutput::Failure("Request failed".to_string())
    }
}
