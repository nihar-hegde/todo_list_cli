use std::{
    fs::{File, OpenOptions},
    io::{self, BufReader},
    os::unix::fs::OpenOptionsExt,
};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Todo {
    id: usize,
    title: String,
    completed: bool,
}

fn main() {
    println!("hellow")
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

fn save_todos(file_path: &str, todos: &[Todo]) -> io::Result<()> {
    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(file_path)?;

    serde_json::to_writer_pretty(file, todos)?;
    Ok(())
}
