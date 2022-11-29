use clap::Args;

use super::{clido_dir, Database, Result};

#[derive(Debug, Args)]
#[command(arg_required_else_help = true)]
pub struct Mark {
    #[arg(value_name = "ID", required = true)]
    todo_id: usize,
}

pub fn mark(command: &Mark) -> Result<()> {
    Database::from_path(clido_dir()?)?
        .mark_complete(command.todo_id)
        .save();

    Ok(())
}
