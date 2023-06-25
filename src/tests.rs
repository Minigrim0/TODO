#[cfg(test)]
mod tests {
    use crate::tasks;

    #[test]
    fn create_task() {
        let task_name: String = "Testing Task 1".to_string();
        let description: String = "This task is created".to_string();
        let date: String = "01-01-1970".to_string();

        let inserted_task = tasks::cmd_add_task(
            task_name.clone(),
            Some(description),
            Some(date)
        );

        assert_eq!(inserted_task.title, task_name);
    }

    #[test]
    fn close_task() {
        let task_name: String = "Testing task 2".to_string();
        let task_description: String = "This task should be completed".to_string();
        let date: String = "01-01-1970".to_string();

        let inserted_task = tasks::cmd_add_task(
            task_name.clone(),
            Some(task_description).clone(),
            Some(date)
        );

        assert_eq!(tasks::cmd_complete_task(inserted_task.id), true);
    }

    #[test]
    fn delete_task() {
        let task_name: String = "Testing task 2".to_string();
        let task_description: String = "This task should be deleted".to_string();
        let date: String = "01-01-1970".to_string();

        let inserted_task = tasks::cmd_add_task(
            task_name.clone(),
            Some(task_description).clone(),
            Some(date)
        );

        assert_eq!(tasks::cmd_delete_task(inserted_task.id.clone()), true);
    }

    #[test]
    fn update_task_name() {
        let task_title: String = "Testing task 2.5".to_string();
        let task_description: String = "This is the testing task 3!".to_string();
        let date: String = "01-01-1970".to_string();

        let inserted_task = tasks::cmd_add_task(
            task_title.clone(),
            Some(task_description.clone()),
            Some(date)
        );

        let new_title: String = "Testing task 3".to_string();
        assert!(
            tasks::cmd_update_task(
                inserted_task.id,
                Some(new_title),
                None,
                None
            )
        )
    }

    #[test]
    fn update_task_description() {
        let task_title: String = "Testing task 3".to_string();
        let task_description: String = "This is the testing task 2!".to_string();
        let date: String = "01-01-1970".to_string();

        let inserted_task = tasks::cmd_add_task(
            task_title.clone(),
            Some(task_description),
            Some(date)
        );

        let new_description: String = "This is the testing task 3!".to_string();
        assert!(
            tasks::cmd_update_task(
                inserted_task.id,
                None,
                Some(new_description),
                None
            )
        )
    }
}
