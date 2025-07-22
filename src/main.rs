use serde::{Deserialize, Serialize};
use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    id: usize,
    description: String,
    done: bool,
}

const FILE_PATH: &str = "tasks.json";
fn load_tasks() -> Vec<Task> {
    if Path::new(FILE_PATH).exists() {
        let data = fs::read_to_string(FILE_PATH).expect("Unable to read file");
        serde_json::from_str(&data).unwrap_or_else(|_| vec![])
    } else {
        vec![]
    }
}

fn save_tasks(tasks: &Vec<Task>) {
    let data = serde_json::to_string_pretty(tasks).expect("Serialization failed");
    fs::write(FILE_PATH, data).expect("Unable to write file");
}
fn main() {
    let mut tasks = load_tasks();

    println!("--- Todo App ---");
    println!("1. Add task");
    println!("2. List tasks");
    println!("3. Mark task as done");
    println!("4. Delete task");
    println!("5. Exit");

    print!("Choose an option: ");
    io::stdout().flush().unwrap();
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).unwrap();

    match choice.trim() {
        "1" => {
            print!("Enter task description: ");
            io::stdout().flush().unwrap();
            let mut desc = String::new();
            io::stdin().read_line(&mut desc).unwrap();

            let new_id = tasks.len() + 1;
            tasks.push(Task {
                id: new_id,
                description: desc.trim().to_string(),
                done: false,
            });
            save_tasks(&tasks);
            println!("Task added.");
        }
        "2" => {
            for task in &tasks {
                let status = if task.done { "[x]" } else { "[ ]" };
                println!("{} {} - {}", status, task.id, task.description);
            }
        }
        "3" => {
            print!("Enter task ID to mark done: ");
            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let id: usize = input.trim().parse().unwrap_or(0);

            if let Some(task) = tasks.iter_mut().find(|t| t.id == id) {
                task.done = true;
                save_tasks(&tasks);
                println!("Task marked as done.");
            } else {
                println!("Task not found.");
            }
        }
        "4" => {
            print!("Enter task ID to delete: ");
            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let id: usize = input.trim().parse().unwrap_or(0);

            tasks.retain(|t| t.id != id);
            save_tasks(&tasks);
            println!("Task deleted.");
        }
        "5" => {
            println!("Goodbye!");
        }
        _ => println!("Invalid option."),
    }
}