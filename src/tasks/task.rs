use serde::Deserialize;

pub trait Task {
    fn execute(&self) -> TaskOutput;
}

#[derive(Deserialize)]
pub struct TaskId(pub String);

#[derive(Debug, Deserialize)]
pub enum TaskOutput {
    Success(String),
    Failure(String),
}
