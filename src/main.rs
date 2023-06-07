pub mod tasks;

use tasks::{add_task, remove_task, complete_task};

fn main() {
    // Read arguments
    let args: Vec<String> = std::env::args().collect();
    // Check length
    if args.len() < 2 {
        // Print usage
        println!("Usage:");
        println!("    add <task>: Adds a task");
        println!("    remove <task>: Removes a task");
        println!("    complete <task>: Completes a task");
        return;
    }

    // Get second argument
    let arg: &String = &args[1];
    // Match argument
    match arg.as_ref() {
        "add" => {
            // Get third argument
            let task: &String = &args[2];
            // Add task
            add_task(task.to_string());
        }
        "remove" => {
            // Get third argument
            let task: &String = &args[2];
            // Remove task
            remove_task(task.to_string());
        }
        "complete" => {
            // Get third argument
            let task: &String = &args[2];
            // Complete task
            complete_task(task.to_string());
        }
        "list" => {
            // List tasks
            let tasks = tasks::read_tasks();
            for task in tasks {
                println!("{}", task);
            }
        }
        _ => {
            // Print usage
            println!("Usage:");
            println!("    add <task>: Adds a task");
            println!("    remove <task>: Removes a task");
            println!("    complete <task>: Completes a task");
        }
    }
}
