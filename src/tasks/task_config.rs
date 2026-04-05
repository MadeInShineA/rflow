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
    pub fn as_task(self) -> Box<dyn Task + Send> {
        match self {
            TaskConfig::Command(t) => Box::new(t),
            TaskConfig::Request(t) => Box::new(t),
        }
    }
}
