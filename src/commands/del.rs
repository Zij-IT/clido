use clap::Args;

use super::{Database, Result};

#[derive(Debug, Args)]
#[command(
    arg_required_else_help = true,
    alias = "del",
    about = "Deletes a task from the list"
)]
pub struct Delete {
    #[arg(
        value_name = "ID",
        required = true,
        help = "ID of the task to be deleted"
    )]
    todo_id: usize,
}

pub fn del(command: Delete) -> Result<()> {
    Database::from_clido_dir()?.delete(command.todo_id).save()
}
