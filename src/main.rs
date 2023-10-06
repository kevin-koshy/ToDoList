use std::{io,thread};
use std::time::Duration;
use console::{Term, style};

fn main() {

    println!("Enter the appropriate option for ToDo list");
    println!("1. {}", style("Add a new task").green());
    println!("2. {}", style("Delete a task").red());
    println!("3. {}", style("Edit a task").yellow());
    println!("4. {}", style("Exit").blue());

    let mut input: String = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    println!("You entered:   {}", input);

    match input.as_str().trim() {
        "1" => {
            println!("Enter the task");
            
        }
        _ => {
            println!("Invalid option");
        }
    }
}