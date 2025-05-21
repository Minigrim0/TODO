use colored::Colorize;

use crate::cli::Cli;
use crate::tasks;

pub fn parse_args(args: Cli) {
    match args {
        Cli {
            command_list: true,
            show_overdue,
            ..
        } => {
            tasks::cmd_display_tasks(show_overdue);
        }
        Cli {
            new_task_name: Some(task_name),
            task_description,
            duedate,
            ..
        } => {
            tasks::cmd_add_task(task_name, task_description, duedate);
        }
        Cli {
            task_to_delete: Some(task_id),
            ..
        } => {
            if tasks::cmd_delete_task(task_id) {
                println!(
                    "{} {}",
                    "Successfully deleted task".green(),
                    task_id.to_string().green().bold()
                )
            } else {
                println!(
                    "{} {}",
                    "An error occured while deleting task".yellow(),
                    task_id.to_string().yellow().bold()
                )
            }
        }
        Cli {
            task_to_complete: Some(task_id),
            ..
        } => {
            if tasks::cmd_complete_task(task_id) {
                println!(
                    "{} {} {}",
                    "Successfully marked task".green().bold(),
                    task_id.to_string().green().bold(),
                    "as completed".green().bold()
                )
            }
            tasks::cmd_display_tasks(false);
        }
        _ => {
            // Print usage
            tasks::cmd_unknown();
        }
    }
}
