use clap::Args;

use crate::todo::{Status, ToDoUpdate};

use super::{Database, Result};

#[derive(Debug, Args)]
#[command(arg_required_else_help = true, about = "Marks a task as complete")]
pub struct Mark {
    #[arg(
        value_name = "ID",
        required = true,
        help = "ID of the task to be marked completed"
    )]
    todo_id: usize,
}

pub fn mark(command: Mark) -> Result<()> {
    Database::from_clido_dir()?
        .update(
            command.todo_id,
            ToDoUpdate::new().with_status(Status::Complete),
        )
        .save()
}
