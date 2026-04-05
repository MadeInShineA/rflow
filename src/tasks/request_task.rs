use std::process::{Command, Output};

use reqwest::StatusCode;
use serde::Deserialize;

use crate::tasks::task::{Task, TaskId, TaskOutput};

#[derive(Deserialize)]
pub enum RequestMethod {
    GET,
    POST,
}

#[derive(Deserialize)]
pub struct RequestTask {
    id: TaskId,
    dependencies: Vec<TaskId>,
    method: RequestMethod,
    url: String,
}

impl Task for RequestTask {
    fn execute(&self) -> TaskOutput {
        let client = reqwest::blocking::Client::new();
        let response_result = match self.method {
            RequestMethod::GET => client.get(&self.url).send(),
            RequestMethod::POST => client.post(&self.url).send(),
        };

        match response_result {
            Ok(response) => match response.error_for_status() {
                Ok(response) => match response.text() {
                    Ok(body) => TaskOutput::Success(body),
                    Err(e) => TaskOutput::Failure(format!("Failed to read response: {}", e)),
                },
                Err(e) => TaskOutput::Failure(format!("HTTP error: {}", e)),
            },
            Err(e) => TaskOutput::Failure(format!("Request failed: {}", e)),
        }
    }
}
