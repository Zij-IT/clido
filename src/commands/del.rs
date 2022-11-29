use clap::Args;

use super::{clido_dir, Database, Result};

#[derive(Debug, Args)]
#[command(arg_required_else_help = true, alias = "del")]
pub struct Delete {
    #[arg(value_name = "ID", required = true)]
    todo_id: usize,
}

pub fn del(command: &Delete) -> Result<()> {
    Database::from_path(clido_dir()?)?
        .delete(command.todo_id)
        .save();

    Ok(())
}
