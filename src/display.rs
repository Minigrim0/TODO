use colored::Colorize;

pub fn display_tasks(tasks: Vec<crate::models::Task>, show_amount: bool) {
    if tasks.len() == 0 {
        println!(
            "{} {} {}",
            "No task to display, try creating one with".blue(),
            "todo -a".green(),
            "!".blue()
        );
        return;
    }
    if show_amount { println!("Displaying {} tasks", tasks.len()); }

    println!(
        "{:>5} | {:>10} | {:>30} | {:>9} | {:>8}",
        "ID",
        "Name",
        "Description",
        "Status",
        "Due Date"
    );

    for task in tasks {
        println!(
            "{}",
            format!("{:0>5} | {:>10} | {:>30} | {:>9} | {:>8}",
                task.id.to_string().magenta(),
                task.title,
                match task.description {
                    Some(desc) => desc,
                    None => "".to_string()
                },
                if task.status { "Completed".green() } else { "Running".blue() },
                match task.due_date {
                    Some(date) => date,
                    None => "Unknown".yellow().to_string()
                }
            )
        );
    }
}


pub fn display_task(task: crate::models::Task) {
    let mut tasks = Vec::new();
    tasks.push(task);
    display_tasks(tasks, false);
}
