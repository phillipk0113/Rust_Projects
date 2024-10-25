use std::collections::HashMap;
use std::io::{self, Write}; // For input/output handling
use std::fs;
use serde::{Deserialize, Serialize};

const FILE_PATH: &str = "tasks.json";

// Define the status of a task
#[derive(Debug, Serialize, Deserialize)]
enum Status {
    Pending,
    Completed,
}

// Define a task structure
#[derive(Debug, Serialize, Deserialize )]
struct Task {
    title: String,
    status: Status,
}

impl Task {
    // Create a new task with Pending status
    fn new(title: String) -> Self {
        Task {
            title,
            status: Status::Pending,
        }
    }
}

fn add_task(tasks: &mut HashMap<u32, Task>, next_id: &mut u32) {
    print!("Enter task title: ");
    io::stdout().flush().unwrap();
    let mut title = String::new();
    io::stdin().read_line(&mut title).expect("Failed to read input");
    let task = Task::new(title.trim().to_string());
    tasks.insert(*next_id, task);
    *next_id += 1;
    println!("Task added successfully!");

    save_tasks(tasks);
}

fn view_tasks(tasks: &HashMap<u32, Task>) {
    if tasks.is_empty() {
        println!("\nNo tasks found.");
        return;
    }
    println!("\nTask List:");
    for (id, task) in tasks {
        println!("ID: {}, Title: {}, Status: {:?}", id, task.title, task.status);
    }
}

fn mark_task_completed(tasks: &mut HashMap<u32, Task>) {
    print!("Enter task ID to mark as completed: ");
    io::stdout().flush().unwrap();
    let mut id = String::new();
    io::stdin().read_line(&mut id).expect("Failed to read input");
    let id:u32 = id.trim().parse().expect("Invalid task ID");
    if let Some(task) = tasks.get_mut(&id) {
        task.status = Status::Completed;
        println!("Task marked as completed!");
        save_tasks(tasks);
    } else {
        println!("Task with ID {} not found.", id);
    }
}

fn remove_task(tasks: &mut HashMap<u32, Task>) {
    print!("Enter task ID to remove: ");

    io::stdout().flush().unwrap();
    let mut id = String::new();

    io::stdin().read_line(&mut id).expect("Failed to read input");
    let id:u32 = id.trim().parse().expect("Invalid task ID");

    if tasks.remove(&id).is_some() {
        println!("Task {} removed.", id);
        save_tasks(tasks); // Save after removal
    } else {
        println!("Task ID not found.");
    }
}

fn load_tasks() -> HashMap<u32, Task> {
    let data = fs::read_to_string(FILE_PATH).unwrap_or_else(|_| String::from("{}"));
    let tasks: HashMap<u32, Task> = serde_json::from_str(&data).unwrap_or_else(|_| HashMap::new());
    return tasks;
}

fn save_tasks(tasks: &HashMap<u32, Task>) {
    let data = serde_json::to_string_pretty(tasks).expect("Failed to serialize tasks");
    fs::write(FILE_PATH, data).expect("Failed to write tasks to file");
}

fn main() {
    let mut tasks: HashMap<u32, Task> = load_tasks(); // Store tasks in a HashMap
    let mut next_id: u32 = tasks.len() as u32 + 1;

    loop {
        println!("\n---- To-Do List Manager ----");
        println!("1. Add Task");
        println!("2. View Tasks");
        println!("3. Mark Task as Completed");
        println!("4. Remove Task");
        println!("5. Exit");
        print!("Choose an option: ");
        io::stdout().flush().unwrap(); // Ensure prompt shows before input

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read input");

        match choice.trim() {
            "1" => add_task(&mut tasks, &mut next_id),
            "2" => view_tasks(&tasks),
            "3" => mark_task_completed(&mut tasks),
            "4" => remove_task(&mut tasks),
            "5" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid option, please try again."),
        }
    }
}
