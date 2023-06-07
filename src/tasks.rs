/* This module allows reading and writing to the database
 */

use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

pub fn read_tasks() -> Vec<String> {
    if File::open("tasks.txt").is_err() {
        File::create("tasks.txt").expect("Could not create file");
    }
    let file = File::open("tasks.txt").expect("Could not open file");
    let reader = BufReader::new(file);
    let mut tasks = Vec::new();
    for line in reader.lines() {
        tasks.push(line.expect("Could not read line"));
    }
    tasks
}

pub fn write_tasks(tasks: Vec<String>) {
    let mut file = File::create("tasks.txt").expect("Could not create file");
    for task in tasks {
        writeln!(file, "{}", task).expect("Could not write line");
    }
}

pub fn add_task(task: String) {
    let mut tasks = read_tasks();
    tasks.push(task);
    write_tasks(tasks);
}

pub fn remove_task(task: String) {
    let mut tasks = read_tasks();
    let position = tasks
        .iter()
        .position(|t| t == &task)
        .expect("Task not found");
    tasks.remove(position);
    write_tasks(tasks);
}

pub fn complete_task(task: String) {
    let mut tasks = read_tasks();
    let position = tasks
        .iter()
        .position(|t| t == &task)
        .expect("Task not found");
    tasks[position].push_str(" [DONE]");
    write_tasks(tasks);
}
