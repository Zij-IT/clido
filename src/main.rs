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

use anyhow::Result;
use clap::{crate_authors, crate_description, crate_version, clap_app};

pub mod sub_names {
    pub const ADD: &str = "add";
    pub const DEL: &str = "del";
    pub const LIST: &str = "list";
    pub const MARK: &str = "mark";
}

pub mod arg_names {
    pub const START: &str = "start";
    pub const TODO: &str = "todo";
    pub const PRIO: &str = "priority";
    pub const DUE: &str = "due_date";

    pub const HIGH_PRIO: &str = "high";
    pub const MID_PRIO: &str = "mid";
    pub const LOW_PRIO: &str = "low";

    pub const DEL_ID: &str = "id";
}

fn main() -> Result<()> {
    let valid_prio = |prio: String| {
        if prio == arg_names::HIGH_PRIO ||
            prio == arg_names::MID_PRIO ||
            prio == arg_names::LOW_PRIO {
            Ok(())
        } else {
            Err(format!("Valid options are: [{}, {}, {}]", arg_names::HIGH_PRIO, arg_names::MID_PRIO, arg_names::LOW_PRIO))
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
            (@arg due: -d --due [DUE] "Sets the due date of the item")
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
        (sub_names::ADD, Some(matches)) => cmd::add(matches),
        (sub_names::DEL, Some(matches)) => cmd::delete(matches),
        (sub_names::MARK, Some(matches)) => cmd::mark(matches),
        (sub_names::LIST, _) => cmd::list(),
        _ => Ok(()),
    }
}