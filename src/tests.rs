#[cfg(test)]
mod tests {
    use crate::tasks;
    use crate::database;

    #[test]
    fn create_task() {
        let task_name: String = "Testing Task 1".to_string();
        let description: String = "Description of the test task".to_string();

        tasks::cmd_add_task(
            Some(task_name),
            Some(description)
        );

        assert_eq!(database::read_tasks(false).len(), 1);
    }
}
