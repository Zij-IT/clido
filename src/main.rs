mod cmd;
mod db;
mod util;

use anyhow::Result;
use clap::{crate_authors, crate_version, App, Arg, SubCommand};

#[macro_use]
extern crate prettytable;

pub mod sub_names {
    pub const ADD: &str = "add";
    pub const DEL: &str = "del";
    pub const LIST: &str = "list";
    pub const COMP: &str = "mark";
}

pub mod arg_names {
    pub const START: &str = "START";
    pub const TODO: &str = "To-Do";
    pub const PRIO: &str = "priority";
    pub const DUE: &str = "due_date";

    pub const HIGH_PRIO: &str = "high";
    pub const MID_PRIO: &str = "mid";
    pub const LOW_PRIO: &str = "low";

    pub const DEL_ID: &str = "ID";
}

fn main() -> Result<()> {
    let matches = App::new("clido")
        .author(crate_authors!())
        .version(crate_version!())
        .about("A todo list manager written for CLI users")
        .subcommand(
            SubCommand::with_name(sub_names::ADD)
                .about("Adds an item to your todo list")
                .version(crate_version!())
                .display_order(1)
                .arg(
                    Arg::with_name(arg_names::START)
                        .short("s")
                        .long("start")
                        .value_name("START")
                        .help("Sets the start date of the item"),
                )
                .arg(
                    Arg::with_name(arg_names::DUE)
                        .short("d")
                        .long("due")
                        .value_name("DUE")
                        .help("Sets the due date of the item"),
                )
                .arg(
                    Arg::with_name(arg_names::PRIO)
                        .short("p")
                        .long("priority")
                        .value_name("PRIORITY")
                        .possible_values(&[
                            arg_names::HIGH_PRIO,
                            arg_names::MID_PRIO,
                            arg_names::LOW_PRIO,
                        ])
                        .help("Sets the priority of the item"),
                )
                .arg(Arg::with_name(arg_names::TODO).help("The actual todo item being added")),
        )
        .subcommand(
            SubCommand::with_name(sub_names::LIST)
                .about("Lists all items on the todo-list")
                .version(crate_version!())
                .display_order(1),
        )
        .subcommand(
            SubCommand::with_name(sub_names::DEL)
                .about("Deletes an item from your todo-list")
                .version(crate_version!())
                .display_order(1)
                .arg(
                    Arg::with_name(arg_names::DEL_ID)
                        .value_name("ID")
                        .help("Numerical ID of the todo to be deleted from the list"),
                ),
        )
        .subcommand(
            SubCommand::with_name(sub_names::COMP)
                .about("Marks an item on the todo list as complete")
                .version(crate_version!())
                .display_order(1)
                .arg(
                    Arg::with_name(arg_names::DEL_ID)
                        .value_name("ID")
                        .help("Numerical ID of the todo to be marked on the list"),
                ),
        )
        .get_matches();

    match matches.subcommand_name() {
        Some(sub_names::ADD) => cmd::add(
            matches
                .subcommand_matches(sub_names::ADD)
                .expect("'add' was found... and then lost!"),
        ),
        Some(sub_names::DEL) => cmd::delete(
            matches
                .subcommand_matches(sub_names::DEL)
                .expect("'del' was found... and then lost!"),
        ),
        Some(sub_names::COMP) => cmd::mark(
            matches
                .subcommand_matches(sub_names::COMP)
                .expect("'del' was found... and then lost!"),
        ),
        Some(sub_names::LIST) => cmd::list(),
        _ => Ok(()),
    }
}
