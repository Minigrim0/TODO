use diesel::sqlite::SqliteConnection;
use diesel::prelude::*;
use std::{io, io::Write};
use colored::Colorize;
use directories::ProjectDirs;

use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

fn run_migrations(connection: &mut diesel::sqlite::SqliteConnection) {
    match connection.run_pending_migrations(MIGRATIONS) {
        Ok(_m) => (),
        Err(e) => panic!("Error running migrations: {:?}", e),
    }
}

pub fn ensure_db_path() {
    if let Some(proj_dirs) = ProjectDirs::from("xyz", "Minigrim0",  "TODO") {
        let database_path = proj_dirs.data_dir();

        if !database_path.exists() {
            std::fs::create_dir_all(database_path).expect("Error creating database directory");
        }
        run_migrations(&mut establish_connection());
    } else {
        panic!("Error getting project directories");
    }
}

pub fn establish_connection() -> SqliteConnection {
    if let Some(proj_dirs) = ProjectDirs::from("xyz", "Minigrim0",  "TODO") {
        let database_path = proj_dirs.data_dir();
        let database_url = format!("{}/todo.db", database_path.to_str().unwrap());

        SqliteConnection::establish(&database_url)
            .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
    } else {
        panic!("Error getting project directories");
    }
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
