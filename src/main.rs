#![warn(
    clippy::perf,
    clippy::style,
    clippy::nursery,
    rust_2018_idioms,
    clippy::pedantic
)]

#[macro_use]
extern crate prettytable;

mod args;
mod cmd;
mod db;
mod util;

use cmd::commands;

use crate::args::{valid_date, valid_priority};
use anyhow::Result;
use clap::{clap_app, crate_authors, crate_description, crate_version};

fn main() -> Result<()> {
    let matches = clap_app!(clido =>
        (author: crate_authors!())
        (version: crate_version!())
        (about: crate_description!())
        (@setting UnifiedHelpMessage)
        (@setting SubcommandRequiredElseHelp)
        (@setting ColoredHelp)
        (@subcommand add =>
            (about: "Adds an item to your todo list")
            (@setting ColoredHelp)
            (@arg priority: -p --priority [PRIO] {valid_priority} "Sets the priority of the item")
            (@arg start: -s --start [START] {valid_date} "Sets the start date of the item")
            (@arg due_date: -d --due [DUE] {valid_date} "Sets the due date of the item")
            (@arg tags: -t --tags [TAGS] min_values(1) "Adds the following tags to the item")
            (@arg todo: <INPUT> "Item to be added.")
        )
        (@subcommand del =>
            (about: "Deletes an item from your todo-list")
            (@setting ColoredHelp)
            (@arg id: <ID> "ID of the item to be deleted")
        )
        (@subcommand mark =>
            (about: "Deletes an item from your todo-list")
            (@setting ColoredHelp)
            (@arg id: <ID> "ID of the item to be deleted")
        )
        (@subcommand list =>
            (about: "Lists all items on the todo-list")
            (@setting ColoredHelp)
            (@arg is_comp: -c --complete "Lists only complete items")
            (@arg is_pend: -p --pending "Lists only pending items")
            (@arg filter: -f --filter +takes_value min_values(1) "Filters list to only output todos that have the tag(s)")
        )
    )
    .get_matches();

    match matches.subcommand() {
        (commands::ADD, Some(matches)) => cmd::add(matches),
        (commands::DEL, Some(matches)) => cmd::delete(matches),
        (commands::MARK, Some(matches)) => cmd::mark(matches),
        (commands::LIST, Some(matches)) => cmd::list(matches),
        _ => Ok(()),
    }
}
