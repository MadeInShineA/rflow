use std::process::{Command, Output};

pub trait Task {
    fn execute(&self) -> TaskOutput;
}

pub struct TaskId(pub String);

pub enum TaskInput {
    StringInput(String),
}

#[derive(Debug)]
pub enum TaskOutput {
    Success(String),
    Failure(String),
}

pub struct CommandTask {
    id: TaskId,
    dependencies: Vec<TaskId>,
    command: String,
    arguments: Vec<String>,
    working_dir: Option<String>,
    env: Vec<(String, String)>,
}

impl Task for CommandTask {
    fn execute(&self) -> TaskOutput {
        let mut cmd = Command::new(&self.command);

        // Add arguments
        cmd.args(&self.arguments);

        // Set working directory
        if let Some(dir) = &self.working_dir {
            cmd.current_dir(dir);
        }

        // Set environment variables
        for (key, value) in &self.env {
            cmd.env(key, value);
        }

        // Execute command
        match cmd.output() {
            Ok(Output {
                stdout,
                stderr,
                status,
            }) => {
                if status.success() {
                    TaskOutput::Success(String::from_utf8_lossy(&stdout).to_string())
                } else {
                    TaskOutput::Failure(format!(
                        "Command failed with code {:?}: {}",
                        status.code(),
                        String::from_utf8_lossy(&stderr)
                    ))
                }
            }
            Err(e) => TaskOutput::Failure(format!("Failed to execute command: {}", e)),
        }
    }
}

impl CommandTask {
    pub fn new(id: String, command: String) -> Self {
        Self {
            id: TaskId(id),
            dependencies: Vec::new(),
            command: command,
            arguments: Vec::new(),
            working_dir: None,
            env: Vec::new(),
        }
    }

    pub fn with_dependencies(mut self, dependencies: Vec<TaskId>) -> Self {
        self.dependencies = dependencies;
        self
    }

    pub fn with_arguments(mut self, arguments: Vec<String>) -> Self {
        self.arguments = arguments;
        self
    }

    pub fn with_working_dir(mut self, working_dir: String) -> Self {
        self.working_dir = Some(working_dir);
        self
    }

    pub fn with_env(mut self, env: Vec<(String, String)>) -> Self {
        self.env = env;
        self
    }
}
