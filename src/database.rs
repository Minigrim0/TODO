/* This module allows reading and writing Tasks to the database */

use diesel::SqliteConnection;
use diesel::prelude::*;
use colored::Colorize;

use crate::models::{Task, NewTask};
use crate::utils::establish_connection;


pub fn read_tasks(overdue: bool) -> Vec<Task> {
    use crate::schema::tasks::dsl::*;

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

    results
}


pub fn add_task(task: NewTask) -> Task {
    use crate::schema::tasks;

    let conn: &mut SqliteConnection = &mut establish_connection();

    diesel::insert_into(tasks::table)
        .values(&task)
        .execute(conn)
        .expect("Error saving new task");

    // Get last_insert_row_id()
    let last_id: i32 = diesel::select(
        diesel::dsl::sql::<diesel::sql_types::Integer>(
            "last_insert_rowid()"
        )
    )
        .get_result(conn)
        .expect("Error getting last insert rowid");

    // Get inserted task
    tasks::table
        .find(last_id)
        .first(conn)
        .expect("Error getting inserted task")
}

pub fn delete_task(task_id: i32) -> bool {
    use crate::schema::tasks::dsl::{tasks, id};

    let conn: &mut SqliteConnection = &mut establish_connection();
    
    let num_deleted = diesel::delete(tasks.filter(id.eq(task_id)))
        .execute(conn)
        .expect(
            format!("Error while deleting the task, it the id {} correct ?", task_id)
                .red()
                .to_string()
                .as_str()
        );

    num_deleted == 1
}

pub fn complete_task(task_id: i32) -> bool {
    use crate::schema::tasks::dsl::{tasks, status};

    let conn: &mut SqliteConnection = &mut establish_connection();

    diesel::update(tasks.find(task_id))
        .set(status.eq(true))
        .execute(conn)
        .unwrap();

}