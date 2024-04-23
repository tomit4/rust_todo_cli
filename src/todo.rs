use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize)]
pub struct Todo {
    pub title: String,
    pub completed: bool,
}

impl Todo {
    pub fn new(title: String) -> Self {
        Self {
            title,
            completed: false,
        }
    }

    pub fn complete(&mut self) {
        self.completed = true;
    }
}

pub fn read_todos_from_file() -> Vec<Todo> {
    match fs::read_to_string("todos.json") {
        Ok(content) => serde_json::from_str(&content).expect("Could not parse todos"),
        Err(_) => Vec::new(),
    }
}

pub fn write_todos_to_file(todos: &[Todo]) {
    let serialized = serde_json::to_string(&todos).expect("Could not serialize todos");
    fs::write("todos.json", serialized).expect("Could not write to file");
}
