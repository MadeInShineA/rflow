use serde::Deserialize;

use crate::tasks::command_task::CommandTask;
use crate::tasks::request_task::RequestTask;
use crate::tasks::task::Task;

#[derive(Deserialize)]
#[serde(tag = "type")]
pub enum TaskConfig {
    #[serde(rename = "command")]
    Command(CommandTask),

    #[serde(rename = "request")]
    Request(RequestTask),
}

impl TaskConfig {
    pub fn as_task(&self) -> &dyn Task {
        match self {
            TaskConfig::Command(t) => t,
            TaskConfig::Request(t) => t,
        }
    }
}
