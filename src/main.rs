use clap::{Arg, Command};
use rusqlite::{Connection, Result};

fn main() -> Result<()> {
    let conn = Connection::open("tasks.db").unwrap();

    conn.execute(
        "CREATE TABLE IF NOT EXISTS task (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            done INTEGER NOT NULL DEFAULT 0
        )",
        [],
    )?;

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
            task_add(&task)
        }
        Some(("done", sub_m)) => {
            let task = sub_m.get_one::<String>("task").unwrap();
            task_done(&task)
        }
        Some(("remove", sub_m)) => {
            let task = sub_m.get_one::<String>("task").unwrap();
            task_remove(&task)
        }
        Some(("edit", sub_m)) => {
            let task = sub_m.get_one::<String>("task").unwrap();
            task_edit(&task)
        }
        Some(("list", _)) => task_list(),
        Some(("search", sub_m)) => {
            let task = sub_m.get_one::<String>("task").unwrap();
            task_search(&task)
        }
        Some(("clear", _)) => task_clear(),
        _ => Ok(()),
    }
}


fn task_add(task: &str) -> Result<()> {
    println!("Adding task: {}", task);
    Ok(())
}

fn task_done(task: &str) -> Result<()> {
    println!("Marking task as done: {}", task);
    Ok(())
}

fn task_remove(task: &str) -> Result<()> {
    println!("Removing task: {}", task);
    Ok(())
}

fn task_edit(task: &str) -> Result<()> {
    println!("Editing task: {}", task);
    Ok(())
}

fn task_list() -> Result<()> {
    println!("Listing all tasks");
    Ok(())
}

fn task_search(task: &str) -> Result<()> {
    println!("Searching for task: {}", task);
    Ok(())
}

fn task_clear() -> Result<()> {
    println!("Clearing all tasks");
    Ok(())
}