use chrono::{prelude::*, Duration};
use colored::Colorize;

pub fn display_tasks(tasks: Vec<crate::models::Task>, show_amount: bool) {
    if tasks.is_empty() {
        println!(
            "{} {} {}",
            "No task to display, try creating one with".blue(),
            "todo -a".green(),
            "!".blue()
        );
        return;
    }
    if show_amount {
        println!("Displaying {} tasks", tasks.len());
    }

    println!(
        "{:>5} | {:>10} | {:>30} | {:>9} | {:>8}",
        "ID", "Name", "Description", "Status", "Due Date"
    );

    let today: NaiveDate = Local::now().date_naive();

    for task in tasks {
        let task_overdue: bool = match task.due_date.clone() {
            Some(enddate) => {
                let task_enddate: NaiveDate =
                    NaiveDate::parse_from_str(enddate.as_str(), "%d-%m-%Y").unwrap();
                let diff = task_enddate - today;
                diff < Duration::zero()
            }
            None => false,
        };

        println!(
            "{:0>5} | {:>10} | {:>30} | {:>9} | {:>8}",
            task.id.to_string().magenta(),
            task.title,
            match task.description {
                Some(desc) => desc,
                None => "".to_string(),
            },
            if task.status {
                "Completed".green()
            } else if task_overdue {
                "overdue".red()
            } else {
                "Running".blue()
            },
            match task.due_date {
                Some(date) => date,
                None => "Unknown".yellow().to_string(),
            }
        );
    }
}

pub fn display_task(task: crate::models::Task) {
    display_tasks(vec![task], false);
}
