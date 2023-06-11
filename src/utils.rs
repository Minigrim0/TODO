use crate::cli::Cli;
use crate::models::NewTask;
use crate::database;

use colored::Colorize;
use diesel::sqlite::SqliteConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::io::Write;
use std::{env, io};





pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn validate_task_id(task_id: Option<i32>) -> i32 {
    let id = task_id.unwrap_or(-1);
    if id == -1 || id < 1 {
        std::process::exit(-1);
    }
    id
}


pub fn display_task(task: crate::models::Task) {
    let mut tasks = Vec::new();
    tasks.push(task);
    display_tasks(tasks, false);
}


pub fn display_tasks(tasks: Vec<crate::models::Task>, show_amount: bool) {
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


pub fn parse_args(args: Cli) {
    match args {
        Cli {command_list: true, show_overdue, ..} => {
            let tasks: Vec<crate::models::Task> = database::read_tasks(show_overdue);
            display_tasks(tasks, true);
        }
        Cli {command_add: true, task_name, task_description, .. } => {
            let mut name: String = String::new();
            match task_name {
                Some(t_name) => {
                    name = t_name;
                }
                None => {
                    // Ask for a name to the user
                    print!("{}", "Choose a name for the new task : ".blue());
                    io::stdout().flush().expect("An error occured");
                    loop {
                        let stdin: io::Stdin = io::stdin(); // We get `Stdin` here.
                        stdin.read_line(&mut name).expect("Invalid name");
                        if name == "" {
                            println!("{}", "Name cannot be empty !".red())
                        } else {
                            break;
                        }
                    }
                }
            }

            let mut description: String = String::new();
            match task_description {
                Some(_) => description = task_description.unwrap(),
                None => {
                    print!("{}", "No description specified, you can add one here : ".blue());
                    io::stdout().flush().expect("An error occured");
                    let stdin: io::Stdin = io::stdin(); // We get `Stdin` here.
                    stdin.read_line(&mut description).expect("Invalid description");
                },
            }

            name = name.replace("\n", "");
            description = description.replace("\n", "");

            let new_task = NewTask {
                title: &name,
                description: Some(&description),
                due_date: None
            };
            let task: crate::models::Task = database::add_task(new_task);
            display_task(task);
        }
        Cli {command_delete: true, task_id, ..} => {
            let id = validate_task_id(task_id);

            // TODO: Ask for a confirmation

            if database::delete_task(id) {
                println!("{}", "Removal successful !".green().bold())
            }
        }
        Cli {command_complete: true, task_id, ..} => {
            let id = validate_task_id(task_id);

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
        _ => {  // Print usage
            println!("{}", format!("Could not parse the command, type --help for help").red().bold());
        }
    }
}
