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

//todo: Entering the following two commands breaks the serialization / deserialization
// add -d 02-10-2021 -s 01-10-2023 "hello there"
// add -p high "hello there" -d 23-01-1232 -s 14-02-2321

fn main() -> Result<()> {
    let matches = clap_app!(clido =>
        (author: crate_authors!())
        (version: crate_version!())
        (about: crate_description!())
        (@subcommand add =>
            (about: "Adds an item to your todo list")
            (@arg todo: <INPUT> "Item to be added.")
            (@arg priority: -p --priority [PRIO] {valid_priority} "Sets the priority of the item")
            (@arg start: -s --start [START] {valid_date} "Sets the start date of the item")
            (@arg due_date: -d --due [DUE] {valid_date} "Sets the due date of the item")
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
    )
    .get_matches();

    match matches.subcommand() {
        (commands::ADD, Some(matches)) => cmd::add(matches),
        (commands::DEL, Some(matches)) => cmd::delete(matches),
        (commands::MARK, Some(matches)) => cmd::mark(matches),
        (commands::LIST, _) => cmd::list(),
        _ => Ok(()),
    }
}
