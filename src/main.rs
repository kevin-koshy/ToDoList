use std::{io};
use console::{ style};
use ToDoList::{match_task, ToDoOption};

fn main() {

    println!("Enter the appropriate option for ToDo list");
    println!("1. {}", style("Add a new task").green());
    println!("2. {}", style("Delete a task").red());
    println!("3. {}", style("Edit a task").yellow());
    println!("4. {}", style("Exit").blue());
    println!("5. {}", style("Display existing tasks").magenta());

    let mut input: String = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
     let as_enum: ToDoOption = input.trim().parse().unwrap();

    match_task(as_enum);
    println!("Number of logical cores is {}", num_cpus::get());
}