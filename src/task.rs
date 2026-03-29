use std::process::Command;

pub trait Task {
    fn execute(&self) -> TaskOutput;
}

pub struct TaskId(pub String);

pub enum TaskInput {
    StringInput(String),
}

#[derive(Debug)]
pub enum TaskOutput {
    StringOutput(String),
}

pub struct CommandTask {
    id: TaskId,
    dependencies: Vec<TaskId>,
    inputs: Vec<TaskInput>,
    command: String,
}

impl Task for CommandTask {
    fn execute(&self) -> TaskOutput {
        let command_output = Command::new("sh")
            .arg("-c")
            .arg(&self.command)
            .output()
            .expect("Failed to execute command");

        let stdout = String::from_utf8_lossy(&command_output.stdout).to_string();

        TaskOutput::StringOutput(stdout)
    }
}

impl CommandTask {
    pub fn new(
        id: TaskId,
        dependencies: Vec<TaskId>,
        inputs: Vec<TaskInput>,
        command: String,
    ) -> Self {
        Self {
            id,
            dependencies,
            inputs,
            command,
        }
    }
}
