/* This module allows reading and writing Tasks to the database */

use diesel::SqliteConnection;
use diesel::prelude::*;

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
    let last_id: i32 = diesel::select(diesel::dsl::sql::<diesel::sql_types::Integer>("last_insert_rowid()"))
        .get_result(conn)
        .expect("Error getting last insert rowid");

    // Get inserted task
    tasks::table
        .find(last_id)
        .first(conn)
        .expect("Error getting inserted task")
}

pub fn delete_task(task_id: i32) -> bool {
    false
}
