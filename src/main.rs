mod manager;
mod storage;
mod task;

use task::Task;

fn main() {
    let mut task = Task::new(1, String::from("Изучить Rust"));

    task.display();

    task.complete();

    task.display();
}
