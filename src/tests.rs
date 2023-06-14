#[cfg(test)]
mod tests {
    use crate::tasks;

    #[test]
    fn create_task() {
        let task_name: String = "Testing Task 1".to_string();
        let description: String = "This task is created".to_string();

        let inserted_task = tasks::cmd_add_task(
            task_name.clone(),
            Some(description)
        );

        assert_eq!(inserted_task.title, task_name);
    }

    #[test]
    fn close_task() {
        let task_name: String = "Testing task 2".to_string();
        let task_description: String = "This task should be completed".to_string();

        let inserted_task = tasks::cmd_add_task(
            task_name.clone(),
            Some(task_description).clone()
        );

        assert_eq!(tasks::cmd_complete_task(inserted_task.id), true);
    }

    #[test]
    fn delete_task() {
        let task_name: String = "Testing task 2".to_string();
        let task_description: String = "This task should be deleted".to_string();

        let inserted_task = tasks::cmd_add_task(
            task_name.clone(),
            Some(task_description).clone()
        );

        assert_eq!(tasks::cmd_delete_task(inserted_task.id.clone()), true);
    }

    #[test]
    fn update_task_name() {

    }

    #[test]
    fn update_task_description() {

    }
}
