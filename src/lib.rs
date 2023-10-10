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
            println!("Enter the task");
        }
        ToDoOption::Delete => {
            println!("Enter the task to delete");
        }
        ToDoOption::Edit => {
            println!("Enter the task to edit");
        }
        ToDoOption::Exit => {
            println!("Exiting");
        }
    }
}