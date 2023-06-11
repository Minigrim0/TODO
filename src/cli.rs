use clap::Parser;


#[derive(Parser, Default, Debug)]
#[clap(author="Minigrim0", version, about="A simple CLI task manager")]
pub struct Cli {
    #[arg(short = 'l', long = "list")]
    /// Lists tasks
    pub command_list: bool,

    #[arg(short = 'a', long = "add")]
    /// Create a new task (with a dialog if no other argument is provided)
    pub command_add: bool,

    #[arg(short = 'd', long = "delete")]
    /// Deletes the task with the id given by the task argument
    pub command_delete: bool,

    #[arg(short = 'c', long = "complete")]
    /// Create a new task (with a dialog if no other argument is provided)
    pub command_complete: bool,

    #[arg(short = 'n', long = "name")]
    /// Defines the task name to use in the action
    pub task_name: Option<String>,

    #[arg(short = 'D', long = "description")]
    /// Defines the description when adding a task
    pub task_description: Option<String>,

    #[arg(short = 'e', long = "duedate")]
    /// Defines the due date when adding a task
    pub duedate: Option<String>,

    #[arg(short = 't', long = "task")]
    /// Defines the task ID for the viewing action
    pub task_id: Option<i32>,

    #[arg(long="overdue")]
    /// Defines the task ID for the viewing action
    pub show_overdue: bool,
}