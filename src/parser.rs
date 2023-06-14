use crate::cli::Cli;
use crate::tasks;

pub fn parse_args(args: Cli) {
    match args {
        Cli {command_list: true, show_overdue, ..} => {
            tasks::cmd_display_tasks(show_overdue);
        }
        Cli {new_task_name: Some(task_name), task_description, .. } => {
            tasks::cmd_add_task(task_name, task_description);
        }
        Cli {task_to_delete: Some(task_id), ..} => {
            tasks::cmd_delete_task(task_id)
        }
        Cli {task_to_complete: Some(task_id), ..} => {
            tasks::cmd_complete_task(task_id)
        }
        _ => {  // Print usage
            tasks::cmd_unknown();
        }
    }
}
