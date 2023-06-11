/* This module allows reading and writing to the database
 */

use diesel::SqliteConnection;
use diesel::prelude::*;
use todo::models::{Task, NewTask};
use todo::establish_connection;


pub fn read_tasks(overdue: bool) {
    use todo::schema::tasks::dsl::*;

    let connection: &mut SqliteConnection = &mut establish_connection();
    let results;

    if overdue {
        results = tasks
        .filter(status.eq(true))
        .limit(5)
        .select(Task::as_select())
        .load(connection)
        .expect("Error loading tasks");
    } else {
        results = tasks
        .limit(5)
        .select(Task::as_select())
        .load(connection)
        .expect("Error loading tasks");
    }

    println!("{} task(s)", results.len());
    for task in results {
        println!("{}", task.title);
        println!("-----------\n");
        println!("{}", task.description.unwrap());
    }
}


pub fn add_task(name: String) -> Task {
    use todo::schema::tasks::dsl::*;

    let conn: &mut SqliteConnection = &mut establish_connection();

    let new_task = vec![
        NewTask {
            title: &name,
            description: Some("Description"),
            due_date: None
        }
    ];

    diesel::insert_into(tasks)
        .values(&new_task)
        .returning(Task::as_returning())
        .get_result(conn)
        .expect("Error saving new post")
}
