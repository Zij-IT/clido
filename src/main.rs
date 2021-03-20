#![warn(
    clippy::perf,
    clippy::style,
    clippy::nursery,
    rust_2018_idioms,
    clippy::pedantic
)]

#[macro_use]
extern crate prettytable;

mod cmd;
mod db;
mod util;
mod args;

use cmd::commands;

use anyhow::Result;
use clap::{crate_authors, crate_description, crate_version, clap_app};

fn main() -> Result<()> {
    let valid_prio = |prio: String| {
        if args::Priority::variants().contains(&&*prio) {
            Ok(())
        } else {
            Err(format!("Valid options are {:?}", args::Priority::variants()))
        }
    };

    let matches = clap_app!(clido =>
        (author: crate_authors!())
        (version: crate_version!())
        (about: crate_description!())
        (@subcommand add =>
            (about: "Adds an item to your todo list")
            (@arg todo: <INPUT> "Item to be added.")
            (@arg priority: -p --priority [PRIO] {valid_prio} "Sets the priority of the item")
            (@arg start: -s --start [START] "Sets the start date of the item")
            (@arg due_date: -d --due [DUE] "Sets the due date of the item")
        )
        (@subcommand del =>
            (about: "Deletes an item from your todo-list")
            (@arg id: <ID> "ID of the item to be deleted")
        )
        (@subcommand mark =>
            (about: "Deletes an item from your todo-list")
            (@arg id: <ID> "ID of the item to be deleted")
        )
        (@subcommand list => (about: "Lists all items on the todo-list"))
    ).get_matches();

    match matches.subcommand() {
        (commands::ADD, Some(matches)) => cmd::add(matches),
        (commands::DEL, Some(matches)) => cmd::delete(matches),
        (commands::MARK, Some(matches)) => cmd::mark(matches),
        (commands::LIST, _) => cmd::list(),
        _ => Ok(()),
    }
}