/* This module allows reading and writing to the database
 */

use diesel::SqliteConnection;
use diesel::prelude::*;
use todo::models::Task;
use todo::establish_connection;


pub fn read_tasks() {
    use todo::schema::tasks::dsl::*;

    let connection: &mut SqliteConnection = &mut establish_connection();
    let results = tasks
        .limit(5)
        .select(Task::as_select())
        .load(connection)
        .expect("Error loading tasks");

    println!("{} task(s)", results.len());
    for task in results {
        println!("{}", task.title);
        println!("-----------\n");
        println!("{}", task.description.unwrap());
    }
}
