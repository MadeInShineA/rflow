use async_trait::async_trait;
use serde::Deserialize;

#[async_trait]
pub trait Task {
    async fn execute(&self) -> TaskOutput;
}

#[derive(Deserialize)]
pub struct TaskId(pub String);

#[derive(Debug, Deserialize)]
pub enum TaskOutput {
    Success(String),
    Failure(String),
}
