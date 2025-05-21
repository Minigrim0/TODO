use colored::Colorize;

pub fn validate_task_id(task_id: Option<i32>) -> i32 {
    let id = task_id.unwrap_or(-1);
    if id == -1 || id < 1 {
        println!(
            "{}",
            format!(
                "Invalid ID '{}'",
                match task_id {
                    Some(id) => id.to_string(),
                    None => "None".to_string(),
                }
            )
            .red()
            .bold()
        );
        std::process::exit(-1);
    }
    id
}
