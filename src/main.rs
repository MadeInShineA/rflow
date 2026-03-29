use crate::task::{CommandTask, Task, TaskId};

mod task;
mod task_parser;

fn main() {
    let hello_world = CommandTask::new(String::from("hello-world"), String::from("ls"))
        .with_working_dir(String::from("./src"));

    let outputs = hello_world.execute();

    dbg!(outputs);
}
