# Rust CLI Todo App

A simple command-line todo list manager built in Rust.  
Supports adding, listing, marking as done, and deleting tasks — all stored in a local JSON file.

## Features

- Add new tasks  
- List all tasks  
- Mark tasks as done  
- Delete tasks  
- Persistent storage using `tasks.json`

---

## Installation

1. Clone the repository:

   ```bash
   git clone https://github.com/yourusername/todoapp.git
   cd todoapp
2.	Add dependencies in Cargo.toml:
   [dependencies]
    serde = { version = "1.0", features = ["derive"] }
    serde_json = "1.0"

3.Build the project:
  cargo build

4.Run the app:
  cargo run
