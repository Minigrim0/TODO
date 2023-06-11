use colored::Colorize;

/**
 * Prints help text
 */
pub fn show_help() {
    println!("{}", "Usage:".green());
    println!("  {:15}: Adds a task", "todo add <task_id>".blue());
    println!("  {:15}: Completes a task", "todo complete <task_id>".blue());
    println!("  {:15}: Removes a task", "todo overdue <task_id>".blue());
    println!("  {:15}: Removes a task", "todo remove <task_id>".blue());
    println!("  {:15}: Removes a task", "todo view <task_id>".blue());
}
