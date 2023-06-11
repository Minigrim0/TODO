use colored::Colorize;
use clap::Parser;
use diesel::sqlite::SqliteConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::io::Write;
use std::{env, io};

use crate::database;
use crate::models::NewTask;


#[derive(Parser, Default, Debug)]
#[clap(author="Minigrim0", version, about="A simple CLI task manager")]
pub struct Cli {
    #[arg(short = 'l', long = "list")]
    /// Lists tasks
    command_list: bool,

    #[arg(short = 'a', long = "add")]
    /// Create a new task (with a dialog if no other argument is provided)
    command_add: bool,

    #[arg(short = 'n', long = "name")]
    /// Defines the task name to use in the action
    task_name: Option<String>,

    #[arg(short = 'd', long = "description")]
    /// Defines the description when adding a task
    task_description: Option<String>,

    #[arg(short = 'e', long = "duedate")]
    /// Defines the due date when adding a task
    duedate: Option<String>,

    #[arg(short = 't', long = "task", default_value_t = -1)]
    /// Defines the task ID for the viewing action
    task_id: i32,

    #[arg(long="overdue")]
    /// Defines the task ID for the viewing action
    show_overdue: bool,
}


pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
pub fn display_task(task: crate::models::Task) {
    let mut tasks = Vec::new();
    tasks.push(task);
    display_tasks(tasks);
}

pub fn display_tasks(tasks: Vec<crate::models::Task>) {
    print!("{:>5} | ", "ID");
    print!("{:>10} | ", "Name");
    println!("{:>30}", "Description");

    for task in tasks {
        print!("{:0>5} | ", task.id);
        print!("{:>10} | ", task.title);
        print!("{:>30}", task.description.unwrap());
        println!();
    }
}


pub fn parse_args(args: Cli) {
    match args {
        Cli {command_list: true, show_overdue, ..} => {
            let tasks: Vec<crate::models::Task> = database::read_tasks(show_overdue);
            println!("Displaying {} {}task(s)", tasks.len(), if show_overdue { "overdue " } else { "" });
            display_tasks(tasks);
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
        // "remove" => {
        //     // Get third argument
        //     let task: &String = &args[2];
        //     println!("{}", task);
        //     // Remove task
        //     // remove_task(task.to_string());
        // }
        // "complete" => {
        //     // Get third argument
        //     let task: &String = &args[2];
        //     println!("{}", task);
        //     // Complete task
        //     // complete_task(task.to_string());
        // }
        // "view" => {  // List tasks
        //     tasks::read_tasks(false);
        // }
        // "overdue" => {  // List overdue tasks
        //     tasks::read_tasks(true);
        // }
        _ => {  // Print usage
            println!("{}", format!("Could not parse the command, type --help for help").red().bold());
        }
    }
}
