use clap::Parser;
use colored::Colorize;

use todo::tasks;

#[derive(Parser, Default, Debug)]
#[clap(author="Minigrim0", version, about="A simple CLI task manager")]
pub struct Cli {
    #[arg(short = 'l', long = "list")]
    /// Lists tasks
    pub command_list: bool,

    #[arg(long="overdue")]
    /// Only display overdue tasks
    pub show_overdue: bool,

    #[arg(short = 'a', long = "add")]
    /// Create a new task with the given name
    pub new_task_name: Option<String>,

    #[arg(short = 'D', long = "description")]
    /// Defines the description when creating a task
    pub task_description: Option<String>,

    #[arg(short = 'e', long = "duedate")]
    /// Defines the due date when creating a task (format: dd-mm-YYYY)
    pub duedate: Option<String>,

    #[arg(short = 'd', long = "delete")]
    /// Deletes the task with the given id
    pub task_to_delete: Option<i32>,

    #[arg(short = 'c', long = "complete")]
    /// Marks the task with the given id as "completed"
    pub task_to_complete: Option<i32>,
}

pub fn parse_args(args: Cli) {
    match args {
        Cli {command_list: true, show_overdue, ..} => {
            tasks::cmd_display_tasks(show_overdue);
        }
        Cli {new_task_name: Some(task_name), task_description, duedate, .. } => {
            tasks::cmd_add_task(task_name, task_description, duedate);
        }
        Cli {task_to_delete: Some(task_id), ..} => {
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
        Cli {task_to_complete: Some(task_id), ..} => {
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
        _ => {  // Print usage
            tasks::cmd_unknown();
        }
    }
}

fn main() {
    todo::init();
    parse_args(Cli::parse());
}
