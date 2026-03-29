use crate::task::{CommandTask, Task, TaskId};

mod task;

fn main() {
    let hello_world = CommandTask::new(
        TaskId(String::from("hello-world")),
        vec![],
        vec![],
        String::from("ls"),
    );

    let outputs = hello_world.execute();

    dbg!(outputs);
}
