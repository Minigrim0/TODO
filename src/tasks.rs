use colored::Colorize;

use crate::database;
use crate::display::{display_task, display_tasks};
use crate::models::NewTask;
use crate::validation::validate_task_id;
use crate::utils;


pub fn cmd_display_tasks(overdue: bool) {
    let tasks: Vec<crate::models::Task> = database::read_tasks(overdue);
    display_tasks(tasks, true);
}


pub fn cmd_add_task(name: String, description: Option<String>) {
        let mut task_name: String = utils::verfify_or_ask_for_value(
            Some(name),
            "Task name".to_string(),
            false
        );

        let mut task_description: String = utils::verfify_or_ask_for_value(
            description,
            "Task description".to_string(),
            true
        );

        task_name = task_name.replace("\n", "");
        task_description = task_description.replace("\n", "");

        let new_task = NewTask {
            title: &task_name,
            description: Some(&task_description),
            due_date: None
        };

        let task: crate::models::Task = database::add_task(new_task);
        display_task(task);
}


pub fn cmd_delete_task(task_id: i32) {
    let id = validate_task_id(Some(task_id));

    // TODO: Ask for a confirmation

    if database::delete_task(id) {
        println!("{}", "Removal successful !".green().bold())
    }
}

pub fn cmd_complete_task(task_id: i32) {
    let id = validate_task_id(Some(task_id));

    // Complete task
    if database::complete_task(id) {
        println!(
            "{} {} {}",
            "Successfully marked task".green().bold(),
            id.to_string().green().bold(),
            "as completed".green().bold()
        )
    }
    let tasks: Vec<crate::models::Task> = database::read_tasks(false);
    display_tasks(tasks, false);
}

pub fn cmd_unknown() {
    println!("{}", format!("Could not parse the command, type --help for help").red().bold());
}
