use colored::Colorize;

use crate::database;
use crate::display::{display_task, display_tasks};
use crate::models::{NewTask, Task};
use crate::validation::validate_task_id;
use crate::utils;


pub fn cmd_display_tasks(overdue: bool) {
    let tasks: Vec<crate::models::Task> = database::read_tasks(overdue);
    display_tasks(tasks, true);
}


pub fn cmd_add_task(name: String, description: Option<String>, enddate: Option<String>) -> Task {
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

        let mut deadline: String = utils::verfify_or_ask_for_value(
            enddate,
            "Task deadline".to_string(),
            true
        );

        task_name = task_name.replace("\n", "");
        task_description = task_description.replace("\n", "");
        deadline = deadline.replace("\n", "");

        let new_task = NewTask {
            title: &task_name,
            description: Some(&task_description),
            due_date: Some(&deadline)
        };

        let task: Task = database::add_task(new_task);
        display_task(task.clone());
        task
}


pub fn cmd_delete_task(task_id: i32) -> bool {
    let id = validate_task_id(Some(task_id));

    // TODO: Ask for a confirmation

    database::delete_task(id)
}

pub fn cmd_complete_task(task_id: i32) -> bool {
    let id = validate_task_id(Some(task_id));

    // Complete task
    database::complete_task(id)
}

pub fn cmd_update_task(task_id: i32, task_title: Option<String>, task_description: Option<String>, _task_date: Option<String>) -> bool {
    let id: i32 = validate_task_id(Some(task_id));

    let title_upd = match task_title {
        Some(title) => database::update_task_name(id, title),
        None => true
    };

    let desc_upd = match task_description {
        Some(description) => database::update_task_description(id, description),
        None => true
    };

    title_upd && desc_upd
}

pub fn cmd_unknown() {
    println!("{}", format!("Could not parse the command, type --help for help").red().bold());
}
