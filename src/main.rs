use clap::{Arg, Command};

fn main() {
    let matches = Command::new("Rust CLI ToDo App")
        .version("0.1.0")
        .author("DragoSuzuki58 <drsz@dorasuzublog.com>")
        .about("This is a my first simple rust app to manage todo tasks.")

        .subcommand(
            Command::new("add")
                .about("Add a new task to the list")
                .arg(
                    Arg::new("task")
                        .index(1)
                        .required(true),
                )
        )
        .subcommand(
            Command::new("done")
                .about("Mark a task as done")
                .arg(
                    Arg::new("task")
                        .index(1)
                        .required(true),
                )
        )
        .subcommand(
            Command::new("remove")
                .about("Remove a task from the list")
                .arg(
                    Arg::new("task")
                        .index(1)
                        .required(true),
                )
        )
        .subcommand(
            Command::new("edit")
                .about("Edit a task")
                .arg(
                    Arg::new("task")
                        .index(1)
                        .required(true),
                )
        )
        .subcommand(
            Command::new("list")
                .about("List all tasks")
        )
        .subcommand(
            Command::new("search")
                .about("Search for a task")
                .arg(
                    Arg::new("task")
                        .index(1)
                        .required(true),
                )
        )
        .subcommand(
            Command::new("clear")
                .about("Clear all tasks")
        )

        .get_matches();

        match matches.subcommand() {
            Some(("add", sub_m)) => {
                let task = sub_m.get_one::<String>("task").unwrap();
                task_add(&task);
            }
            Some(("done", sub_m)) => {
                let task = sub_m.get_one::<String>("task").unwrap();
                task_done(&task);
            }
            Some(("remove", sub_m)) => {
                let task = sub_m.get_one::<String>("task").unwrap();
                task_remove(&task);
            }
            Some(("edit", sub_m)) => {
                let task = sub_m.get_one::<String>("task").unwrap();
                task_edit(&task);
            }
            Some(("list", _)) => {
                task_list();
            }
            Some(("search", sub_m)) => {
                let task = sub_m.get_one::<String>("task").unwrap();
                task_search(&task);
            }
            Some(("clear", _)) => {
                task_clear();
            }
            _ => {}
        }
}


fn task_add(task: &str) {
    println!("Adding task: {}", task);
}

fn task_done(task: &str) {
    println!("Marking task as done: {}", task);
}

fn task_remove(task: &str) {
    println!("Removing task: {}", task);
}

fn task_edit(task: &str) {
    println!("Editing task: {}", task);
}

fn task_list() {
    println!("Listing all tasks");
}

fn task_search(task: &str) {
    println!("Searching for task: {}", task);
}

fn task_clear() {
    println!("Clearing all tasks");
}