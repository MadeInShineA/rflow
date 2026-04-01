use tasks::task_parser::parse_tasks;

mod tasks;

fn main() {
    let tasks = parse_tasks("tasks.toml");

    for task in tasks {
        let output = task.as_task().execute();
        dbg!(output);
    }
}
