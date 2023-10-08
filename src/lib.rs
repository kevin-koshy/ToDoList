use rusqlite::{Result, Connection};
pub enum ToDoOption {
    Add,
    Delete,
    Edit,
    Exit,
}

// Initialize Database
