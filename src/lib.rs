use std::io;
use rusqlite::{Result, Connection};

#[derive(Debug)]
struct Task {
    id: u8,
    task: String,
}
#[derive(Debug)]
pub enum ToDoOption {
    Add,
    Delete,
    Edit,
    Exit,
    Display,
    }

impl std::str::FromStr for ToDoOption {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" => Ok(ToDoOption::Add),
            "2" => Ok(ToDoOption::Delete),
            "3" => Ok(ToDoOption::Edit),
            "4" => Ok(ToDoOption::Exit),
            "5" => Ok(ToDoOption::Display),
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
            create_connection(task.trim().to_string()).ok();
            println!("You entered:   {:?}", task.trim());
        }
        ToDoOption::Delete => {
            println!("Enter the task to delete");
            let mut task_delete = String::new();
            io::stdin().read_line(&mut task_delete).expect("Failed to read line");
            delete_task(task_delete.trim().to_string()).ok();
            println!("You entered:   {:?}", task_delete.trim());

        }
        ToDoOption::Edit => {
            println!("Enter the task to edit");
            let mut task_edit = String::new();
            io::stdin().read_line(&mut task_edit).expect("Failed to read line");
            println!("Enter the new task");
            let mut new_task = String::new();
            io::stdin().read_line(&mut new_task).expect("Failed to read line");
            edit_task(task_edit.trim().to_string(), new_task.trim().to_string()).ok();
            println!("You entered:   {:?}", task_edit.trim());
        }
        ToDoOption::Exit => {
            println!("Exiting");
        }
        ToDoOption::Display => {
            println!("Displaying current tasks..");
            display_tasks();
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

fn edit_task(item_to_edit:String, new_task:String) -> Result<(), rusqlite::Error>{
    let conn = Connection::open("todo.db")?;
    conn.execute("UPDATE users SET ITEMS = ?1 WHERE ITEMS = ?2", [new_task, item_to_edit])?;
    println!("Task edited");
    Ok(())
}

fn display_tasks() {
    let conn = Connection::open("todo.db").unwrap();
    let mut stmt = conn.prepare("SELECT * FROM users").unwrap();
    // let rows = stmt.query_map([],|row| row.get::<usize,String>(1)).unwrap();
    // let rows = stmt.query_map([],|row| row.get::<usize,String>(1)).unwrap();
    // for row in rows {
    //     println!("{}", row.unwrap());
    // }

    let tasks = stmt.query_map([],|row|{
        Ok(Task{
            id: row.get(0)?,
            task: row.get(1)?,
        })
    }).unwrap();
    for task in tasks {
        println!("{:?}: {:?}", task.as_ref().unwrap().id, task.as_ref().unwrap().task);
    }
}