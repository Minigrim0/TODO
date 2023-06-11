pub mod tasks;
pub mod help;

use colored::Colorize;

fn main() {
    // Read arguments
    let args: Vec<String> = std::env::args().collect();
    // Check length
    if args.len() < 2 {
        // Print usage
        help::show_help();
        return;
    }

    // Get second argument
    let arg: &String = &args[1];
    // Match argument
    match arg.as_ref() {
        "add" => {
            // Get third argument
            let s_task_id: &String = &args[2];
            let task_id: i32 = s_task_id.parse().unwrap();
            println!("{}", task_id);
            // Add task
            // add_task(task.to_string());
        }
        "remove" => {
            // Get third argument
            let task: &String = &args[2];
            println!("{}", task);
            // Remove task
            // remove_task(task.to_string());
        }
        "complete" => {
            // Get third argument
            let task: &String = &args[2];
            println!("{}", task);
            // Complete task
            // complete_task(task.to_string());
        }
        "view" => {  // List tasks
            tasks::read_tasks(false);
        }
        "overdue" => {  // List overdue tasks
            tasks::read_tasks(true);
        }
        _ => {  // Print usage
            println!("{}", format!("Unknown command '{}'", arg).red().bold());
            help::show_help();
        }
    }
}
