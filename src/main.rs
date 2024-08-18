use clap::{Arg, Command};
use rusqlite::{params, Connection, Result};
use std::io::{self, Write};

fn main() -> Result<()> {
    let conn = Connection::open("tasks.db").unwrap();

    conn.execute(
        "CREATE TABLE IF NOT EXISTS task (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
            updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
            name TEXT NOT NULL,
            description TEXT,
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
                    Arg::new("name")
                        .index(1)
                        .required(true)
                    )
                .arg(
                    Arg::new("description")
                        .index(2)
                        .required(false)
                        .default_value("default description")
                    )
                )
        .subcommand(
            Command::new("done")
                .about("Mark a task as done")
                .arg(
                    Arg::new("id")
                        .index(1)
                        .required(true),
                )
        )
        .subcommand(
            Command::new("remove")
                .about("Remove a task from the list")
                .arg(
                    Arg::new("id")
                        .index(1)
                        .required(true),
                )
        )
        .subcommand(
            Command::new("edit")
                .about("Edit a task")
                .arg(
                    Arg::new("id")
                        .index(1)
                        .required(true),
                )
        )
        .subcommand(
            Command::new("search")
                .about("Search for a task")
                .arg(
                    Arg::new("name")
                        .index(1)
                        .required(true),
                )
        )
        .subcommand(
            Command::new("list")
                .about("List all tasks")
        )
        .subcommand(
            Command::new("clear")
                .about("Clear all tasks")
        )

        .get_matches();

    match matches.subcommand() {
        Some(("add", sub_m)) => {
            let name = sub_m.get_one::<String>("name").unwrap();
            let description = sub_m.get_one::<String>("description").unwrap();
            task_add(&conn, &name, &description)
        }
        Some(("done", sub_m)) => {
            let task = sub_m.get_one::<String>("id").unwrap();
            task_done(&conn, &task)
        }
        Some(("remove", sub_m)) => {
            let task = sub_m.get_one::<String>("id").unwrap();
            task_remove(&conn, &task)
        }
        Some(("edit", sub_m)) => {
            let task = sub_m.get_one::<String>("id").unwrap();
            task_edit(&conn, &task)
        }
        Some(("search", sub_m)) => {
            let task = sub_m.get_one::<String>("name").unwrap();
            task_search(&conn, &task)
        }
        Some(("list", _)) => task_list(&conn),
        Some(("clear", _)) => task_clear(&conn),
        _ => Ok(()),
    }
}


fn task_add(conn: &Connection, name: &str, description: &str) -> Result<()> {
    conn.execute("INSERT INTO task (name, description) VALUES (?1, ?2)", params![name, description])?;
    let last_id: i64 = conn.last_insert_rowid();
    println!("ID: {} \nTask added: {} \nDescription: {}", last_id, name, description);
    Ok(())
}

fn task_done(conn: &Connection, id: &str) -> Result<()> {
    let rows_affected = conn.execute("UPDATE task SET done = 1 WHERE id = ?1", params![id])?;
    if rows_affected == 0 {
        println!("No task found with id: {}", id);
    } else {
        println!("Marking task as done: {}", id);
    }
    Ok(())
}

fn task_remove(conn: &Connection, id: &str) -> Result<()> {
    let rows = conn.query_row("SELECT * FROM task WHERE id = ?1", params![id], |row| {
        let id: i64 = row.get("id")?;
        let name: String = row.get("name")?;
        let description: String = row.get("description")?;
        let done: i64 = row.get("done")?;
        Ok((id, name, description, done))
    })?;

    println!("ID: {} Task: {} \nDescription: {} \nDone: {}", rows.0, rows.1, rows.2, rows.3);
    println!("Are you sure you want to delete this task? (y/n)");

    loop {
        let mut input = String::new();
        io::stdout().flush().map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
        io::stdin().read_line(&mut input).map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
        let input = input.trim();

        if input.eq_ignore_ascii_case("y") {
            let rows_affected = conn.execute("DELETE FROM task WHERE id = ?1", params![id])?;
            if rows_affected == 0 {
                println!("Failed to delete task: {}", id);
                break
            } else {
                println!("Task deleted: {}", id);
                break
            }
        } else if input.eq_ignore_ascii_case("n") {
            println!("Task deletion canceled.");
            break
        } else {
            println!("Invalid input. Please enter 'y' or 'n'.");
        }
    }

    Ok(())
}

fn task_edit(conn: &Connection, id: &str) -> Result<()> {
    let rows = conn.query_row("SELECT * FROM task WHERE id = ?1", params![id], |row| {
        let id: i64 = row.get("id")?;
        let name: String = row.get("name")?;
        let description: String = row.get("description")?;
        let done: i64 = row.get("done")?;
        Ok((id, name, description, done))
    })?;

    println!("ID: {} Task: {} \nDescription: {} \nDone: {}", rows.0, rows.1, rows.2, rows.3);

    println!("Enter new task name:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
    let new_name = input.trim();

    println!("Enter new task description:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
    let new_description = input.trim();

    conn.execute("UPDATE task SET name = ?1 WHERE id = ?2", params![new_name, id])?;
    conn.execute("UPDATE task SET description = ?1 WHERE id = ?2", params![new_description, id])?;
    println!("Task name updated: {} to {}", rows.1, new_name);
    println!("Task description updated: {}\nto\n{}", rows.2, new_description);
    Ok(())
}

fn task_search(conn: &Connection, name: &str) -> Result<()> {
    let pattern = format!("%{}%", name);
    let mut stmt = conn.prepare("SELECT * FROM task WHERE name LIKE ?1")?;
    let mut rows = stmt.query(params![pattern])?;

    println!("Searching for tasks matching: {}", name);
    while let Some(row) = rows.next()? {
        let id: i32 = row.get(0)?;
        let name: String = row.get("name")?;
        let description: String = row.get("description")?;
        println!("Found task - ID: {}\tname: {}\tDescription: {}", id, name, description);
    }

    Ok(())
}

fn task_list(conn: &Connection) -> Result<()> {
    let mut stmt = conn.prepare("SELECT id, created_at, updated_at, name, description, done FROM task")?;
    let mut rows = stmt.query([])?;

    println!("Listing all tasks:");
    while let Some(row) = rows.next()? {
        let id: i32 = row.get("id")?;
        let created_at: String = row.get("created_at")?;
        let updated_at: String = row.get("updated_at")?;
        let name: String = row.get("name")?;
        let description: String = row.get("description")?;
        let done: i32 = row.get("done")?;
        println!("ID: {}\tCreated At: {}\tUpdated At: {}\tName: {}\tDescription: {}\tDone: {}", id, created_at, updated_at, name, description, done);
    }

    Ok(())
}

fn task_clear(conn: &Connection) -> Result<()> {
    println!("Are you sure you want to clear all tasks? This action cannot be undone. (y/n)");

    loop {
        let mut input = String::new();
        io::stdout().flush().map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
        io::stdin().read_line(&mut input).map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
        let input = input.trim();

        if input.eq_ignore_ascii_case("y") {
            conn.execute("DELETE FROM task", [])?;
            println!("All tasks cleared");
            break
        } else if input.eq_ignore_ascii_case("n") {
            println!("Task deletion canceled.");
            break
        } else {
            println!("Invalid input. Please enter 'y' or 'n'.");
        }
    }
    Ok(())
}