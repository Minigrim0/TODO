use clap::Parser;


#[derive(Parser, Default, Debug)]
#[clap(author="Minigrim0", version, about="A simple CLI task manager")]
pub struct Cli {
    #[arg(short = 'l', long = "list")]
    /// Lists tasks
    pub command_list: bool,
    
    #[arg(long="overdue")]
    /// Only display overdue tasks
    pub show_overdue: bool,

    #[arg(short = 'a', long = "add")]
    /// Create a new task with the given name
    pub new_task_name: Option<String>,

    #[arg(short = 'D', long = "description")]
    /// Defines the description when creating a task
    pub task_description: Option<String>,

    #[arg(short = 'e', long = "duedate")]
    /// Defines the due date when creating a task
    pub duedate: Option<String>,

    #[arg(short = 'd', long = "delete")]
    /// Deletes the task with the given id
    pub task_to_delete: Option<i32>,

    #[arg(short = 'c', long = "complete")]
    /// Marks the task with the given id as "completed"
    pub task_to_complete: Option<i32>,
}
