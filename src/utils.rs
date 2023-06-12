use diesel::sqlite::SqliteConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::{env, io, io::Write};
use colored::Colorize;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn verfify_or_ask_for_value(current_value: Option<String>, value_name: String, optional: bool) -> String {
    let mut new_value: String = String::new();
    match current_value {
        Some(t_name) => t_name,
        None => {
            // Ask for a name to the user
            print!(
                "{}",
                format!(
                    "Choose a value for the field '{}' {} : ",
                    value_name,
                    if optional { "[optional]".green() } else { "[required]".red() }
                ).blue()
            );
            io::stdout().flush().expect("An error occured");
            loop {
                let stdin: io::Stdin = io::stdin(); // We get `Stdin` here.
                stdin.read_line(&mut new_value).expect("Invalid name");
                if !optional && new_value == "" {
                    println!("{}", "Name cannot be empty !".red())
                } else {
                    break;
                }
            }
            new_value
        }
    }
}
