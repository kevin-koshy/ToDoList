use std::{io,thread};
use std::time::Duration;
use console::{Term, style};
use ToDoList::{match_task, ToDoOption};

fn main() {

    println!("Enter the appropriate option for ToDo list");
    println!("1. {}", style("Add a new task").green());
    println!("2. {}", style("Delete a task").red());
    println!("3. {}", style("Edit a task").yellow());
    println!("4. {}", style("Exit").blue());

    let mut input: String = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
     let as_enum: ToDoOption = input.trim().parse().unwrap();
    // println!("You entered:   {:?}", as_enum);


    match_task(as_enum);

    match input.as_str().trim() {
        "1" => {
            println!("Enter the tasks to add. 1");
        }
        "2" => {
            println!("Enter the task to delete");
        }
        "3" => {
            println!("Enter the task to edit");
        }
        "4" => {
            println!("Exiting..");
        }
        _ => {
            println!("Invalid option");
        }

    }
}