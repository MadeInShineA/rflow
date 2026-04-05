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
            Ok(response) => {
                if response.status().is_success() {
                    match response.text() {
                        Ok(content) => TaskOutput::Success(content),
                        Err(e) => TaskOutput::Failure(format!("Failed to read response: {}", e)),
                    }
                } else {
                    TaskOutput::Failure(format!(
                        "Got an unexpected response status code: {}",
                        response.status()
                    ))
                }
            }
            Err(e) => TaskOutput::Failure(format!("Request failed: {}", e)),
        }
    }
}
