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
mod commands;
mod db;
mod util;

use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(
    name = "clido",
    author = "Elijah Hartvigsen <elijah.reed@hartvigsen.xyz>",
    about = "A small todo-list application"
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    Add(commands::Add),
    Delete(commands::Delete),
    Mark(commands::Mark),
    List(commands::List),
}

fn main() -> Result<()> {
    match Cli::parse().command {
        Command::Add(add) => commands::add(&add),
        Command::Delete(del) => commands::del(&del),
        Command::Mark(mark) => commands::mark(&mark),
        Command::List(list) => commands::list(&list),
    }
}
