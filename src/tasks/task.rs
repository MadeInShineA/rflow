use serde::Deserialize;
use std::process::Output;
use tokio::process::Command;

#[derive(Deserialize)]
pub struct TaskId(pub String);

#[derive(Deserialize)]
pub struct Task {
    id: TaskId,
    dependencies: Vec<TaskId>,
    command: String,
    arguments: Vec<String>,
    working_dir: Option<String>,
    env: Vec<(String, String)>,
}

#[derive(Debug, Deserialize)]
pub enum TaskOutput {
    Success(String),
    Failure(String),
}

impl Task {
    pub async fn execute(&self) -> TaskOutput {
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
        match cmd.output().await {
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
