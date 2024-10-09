use serde::{Deserialize, Serialize};
use serde_json;
use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

#[derive(Serialize, Deserialize)]
struct Todo {
    id: usize,
    text: String,
    completed: bool,
}

fn main() -> io::Result<()> {
    let file_path = "todos.json";
    let mut todos = load_todos(file_path)?;

    let mut stdout = io::stdout().into_raw_mode()?;
    let stdin = io::stdin();

    loop {
        display_todos(&todos);
        println!("\nCommands: (a)dd, (x) toggle, (d)elete, (q)uit");

        for c in stdin.keys() {
            match c.unwrap() {
                Key::Char('a') => {
                    let new_todo = add_todo(todos.len())?;
                    todos.push(new_todo);
                    save_todos(file_path, &todos)?;
                    break;
                }
                Key::Char('x') => {
                    toggle_todo(&mut todos);
                    save_todos(file_path, &todos)?;
                    break;
                }
                Key::Char('d') => {
                    delete_todo(&mut todos);
                    save_todos(file_path, &todos)?;
                    break;
                }
                Key::Char('q') => return Ok(()),
                _ => {}
            }
        }
    }
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

fn display_todos(todos: &[Todo]) {
    println!("\x1B[2J\x1B[1;1H"); // Clear screen
    println!("Todo List:");
    for todo in todos.iter() {
        println!(
            "{}: [{}] {}",
            todo.id + 1,
            if todo.completed { "x" } else { " " },
            todo.text
        );
    }
}

fn add_todo(next_id: usize) -> io::Result<Todo> {
    println!("Enter new todo:");
    let mut text = String::new();
    io::stdin().read_line(&mut text)?;
    Ok(Todo {
        id: next_id,
        text: text.trim().to_string(),
        completed: false,
    })
}

fn toggle_todo(todos: &mut Vec<Todo>) {
    println!("Enter todo number to toggle:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    if let Ok(index) = input.trim().parse::<usize>() {
        if index > 0 && index <= todos.len() {
            todos[index - 1].completed = !todos[index - 1].completed;
        }
    }
}

fn delete_todo(todos: &mut Vec<Todo>) {
    println!("Enter todo number to delete:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    if let Ok(index) = input.trim().parse::<usize>() {
        if index > 0 && index <= todos.len() {
            todos.remove(index - 1);
            // Update IDs of remaining todos
            for (i, todo) in todos.iter_mut().enumerate() {
                todo.id = i;
            }
        }
    }
}
