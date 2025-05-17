// Include the todo module from todo.rs
mod todo;

use todo::Task;
use std::io::{self, Write};

fn main() {
    println!("📝 Simple To-Do List (Rust)");

    // Vector to store the list of tasks
    let mut tasks: Vec<Task> = Vec::new();

    loop {
        // Display the menu options
        println!("\nChoose an option:");
        println!("1. Add task");
        println!("2. View tasks");
        println!("3. Mark task as done");
        println!("4. Exit");

        // Read the user's choice
        let choice = read_input("Enter your choice: ");

        match choice.trim() {
            "1" => {
                // Adding a new task
                let title = read_input("Enter task title: ");
                let task = Task::new(title);
                tasks.push(task);
                println!("✅ Task added.");
            }
            "2" => {
                // Displaying all tasks
                println!("\n📋 To-Do List:");
                for (i, task) in tasks.iter().enumerate() {
                    let status = if task.completed { "[Done]" } else { "[Pending]" };
                    println!("{}. {} {}", i + 1, status, task.title);
                }
            }
            "3" => {
                // Marking a task as done
                let index_input = read_input("Enter task number to mark as done: ");
                if let Ok(index) = index_input.trim().parse::<usize>() {
                    if index >= 1 && index <= tasks.len() {
                        tasks[index - 1].mark_done();
                        println!("✅ Task marked as done.");
                    } else {
                        println!("⚠️ Invalid task number.");
                    }
                } else {
                    println!("⚠️ Please enter a valid number.");
                }
            }
            "4" => {
                // Exit the loop and quit the app
                println!("👋 Goodbye!");
                break;
            }
            _ => println!("⚠️ Invalid option. Try again."),
        }
    }
}

// Helper function to read input from the user
fn read_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap(); // Flush to ensure prompt is printed
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap(); // Read user input
    input.trim().to_string() // Remove whitespace and return
}
