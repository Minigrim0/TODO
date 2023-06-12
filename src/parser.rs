use crate::cli::Cli;
use crate::tasks;

pub fn parse_args(args: Cli) {
    match args {
        Cli {command_list: true, show_overdue, ..} => {
            tasks::cmd_display_tasks(show_overdue);
        }
        Cli {command_add: true, task_name, task_description, .. } => {
            tasks::cmd_add_task(task_name, task_description);
        }
        Cli {command_delete: true, task_id, ..} => {
            tasks::cmd_delete_task(task_id)
        }
        Cli {command_complete: true, task_id, ..} => {
            tasks::cmd_complete_task(task_id)
        }
        _ => {  // Print usage
            tasks::cmd_unknown();
        }
    }
}
