use super::{date_from_input, Database, Local, Priority, Result, Status, ToDo};
use clap::Args;

#[derive(Debug, Args)]
#[command(
    arg_required_else_help = true,
    about = "Adds an item to your todo list"
)]
pub struct Add {
    #[arg(value_name = "TODO", required = true)]
    todo: String,

    #[arg(
        short = 'p',
        long = "priority",
        value_name = "PRIORITY",
        help = "Sets the priority of the task"
    )]
    priority: Option<Priority>,

    #[arg(
        short = 's',
        long = "start",
        value_name = "START_DATE",
        help = "Sets the start date for the task. Format: dd-mm-yyyy"
    )]
    start_date: Option<String>,

    #[arg(
        short = 'd',
        long = "due",
        value_name = "DUE_DATE",
        help = "Sets the due date for the task. Format: dd-mm-yyyy"
    )]
    due_date: Option<String>,

    #[arg(
        short = 't',
        long = "tags",
        value_name = "TAGS",
        value_delimiter = ',',
        help = "Sets tags for the task which are later used to filter tasks"
    )]
    tags: Option<Vec<String>>,
}

pub fn add(command: Add) -> Result<()> {
    let desc = command.todo.clone();

    let start = command
        .start_date
        .as_deref()
        .map_or_else(|| Local::now().naive_local(), date_from_input);

    let due = command.due_date.as_deref().map(date_from_input);

    let prio = command.priority;

    let tags = command.tags.as_ref().map_or(Vec::new(), Clone::clone);

    let todo = ToDo {
        desc,
        start,
        prio,
        due,
        tags,
        recur: None,
        status: Status::Pending,
    };

    Database::from_clido_dir()?.add(todo).save()
}
