use std::io;
use rusqlite::{Result, Connection};


#[derive(Debug)]
pub enum ToDoOption {
    Add,
    Delete,
    Edit,
    Exit,
    }

impl std::str::FromStr for ToDoOption {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" => Ok(ToDoOption::Add),
            "2" => Ok(ToDoOption::Delete),
            "3" => Ok(ToDoOption::Edit),
            "4" => Ok(ToDoOption::Exit),
            _ => Err("Invalid option".to_string()),
        }
    }
}
// Add a new task
pub fn match_task(task:ToDoOption) -> (){
    match  task {
        ToDoOption::Add => {
            println!("Enter the task to add");
            let mut task = String::new();
            io::stdin().read_line(&mut task).expect("Failed to read line");
            create_connection(task.trim().to_string());
            println!("You entered:   {:?}", task.trim());
        }
        ToDoOption::Delete => {
            println!("Enter the task to delete");
            let mut task_delete = String::new();
            io::stdin().read_line(&mut task_delete).expect("Failed to read line");
            delete_task(task_delete.trim().to_string());
            println!("You entered:   {:?}", task_delete.trim());

        }
        ToDoOption::Edit => {
            println!("Enter the task to edit");
        }
        ToDoOption::Exit => {
            println!("Exiting");
        }
    }
}

fn create_connection(task: String) -> Result<Connection, rusqlite::Error> {
    let conn = Connection::open("todo.db")?;
    conn.execute("CREATE TABLE IF NOT EXISTS users (id INTEGER PRIMARY KEY, ITEMS TEXT NOT NULL)",(),)?;
    println!("Table created");
    conn.execute("INSERT INTO users (ITEMS) VALUES (?1)", [&task])?;
    println!("Task added");
    Ok(conn)
    }

fn delete_task(task:String) -> Result<(), rusqlite::Error>{
    let conn = Connection::open("todo.db")?;
    conn.execute("DELETE FROM users WHERE ITEMS = ?1", [task])?;
    println!("Task deleted");
    Ok(())
}