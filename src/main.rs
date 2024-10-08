use std::{
    fs::File,
    io::{self, BufReader},
};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Todo {
    id: usize,
    title: String,
    completed: bool,
}

fn main() -> Result<()> {
    let file_path = "todos.json";

    Ok(())
}

fn load_todos(file_path: &str) -> io::Result<Vec<Todo>> {
    let file = File::open(file_path).or_else(|_| File::create(file_path))?;
    let reader = BufReader::new(file);

    let todos: Vec<Todo> = match serde_json::from_reader(reader) {
        Ok(todos) => todos,
        Err(_) => Vec::new(),
    };

    Ok(todos)
}
