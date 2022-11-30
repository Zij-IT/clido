use anyhow::Result;
use clap::Args;

use crate::{
    db::Database,
    todo::{Priority, Status, ToDoUpdate},
};

#[derive(Debug, Args)]
#[command(
    arg_required_else_help = true,
    about = "Modifies an item on your todo list",
    alias = "mod"
)]
pub struct Modify {
    #[arg(value_name = "ID", required = true)]
    todo_id: usize,

    #[arg(
        long = "desc",
        value_name = "DESC",
        help = "Sets the content of the chosen task"
    )]
    desc: Option<String>,

    #[arg(
        long = "status",
        value_name = "STATUS",
        help = "Sets the content of the chosen task"
    )]
    status: Option<Status>,

    #[arg(
        short = 'p',
        long = "prio",
        value_name = "PRIORITY",
        help = "Sets the priority of the task"
    )]
    priority: Option<Option<Priority>>,

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
    due_date: Option<Option<String>>,

    #[arg(
        short = 't',
        long = "tags",
        value_name = "TAGS",
        value_delimiter = ',',
        help = "Sets tags for the task which are later used to filter tasks"
    )]
    tags: Option<Vec<String>>,
}

pub fn modify(mods: &Modify) -> Result<()> {
    let mut update = ToDoUpdate::new();
    update.desc = mods.desc.clone();
    update.tags = mods.tags.clone();
    update.prio = mods.priority;
    update.status = mods.status;
    update.start = mods.start_date.as_deref().map(super::date_from_input);
    update.due = mods
        .due_date
        .as_ref()
        .map(|x| x.as_deref().map(super::date_from_input));

    Database::from_clido_dir()?
        .update(mods.todo_id, update)
        .save()
}
