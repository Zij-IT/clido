use super::db::{Database, Priority, Status, ToDo};
use super::util::clido_dir;

use anyhow::{Context, Result};
use chrono::{Local, NaiveDate};
use clap::ArgMatches;
use prettytable::{format, Table};

use std::convert::TryFrom;

pub mod commands {
    pub const ADD: &str = "add";
    pub const DEL: &str = "del";
    pub const LIST: &str = "list";
    pub const MARK: &str = "mark";
}

pub fn add(sub_args: &ArgMatches<'_>) -> Result<()> {
    //Construct a To-Do based on the arguments passed
    let todo = {
        // Safety: Unwrap is safe because clap enforces that this
        // argument is present
        let desc = sub_args.value_of("todo").unwrap().to_string();

        // Doesn't allow for passing the time of an item yet
        // this is subject to change. Default value is current day
        let start = sub_args.value_of("start").map_or_else(
            || Local::now().naive_local(),
            |date| {
                // Safety: Unwrap is safe, because in order for a date to be considered
                // valid, it had to be verified by being parsed earlier
                let naive = NaiveDate::parse_from_str(date, "%d-%m-%Y").unwrap();
                naive.and_hms(0, 0, 0)
            },
        );

        let prio = Priority::try_from(sub_args.value_of("priority")).ok();

        let due = sub_args.value_of("due_date").map(|date| {
            // Safety: Unwrap is safe, because in order for a date to be considered
            // valid, it had to be verified by being parsed earlier
            let naive = NaiveDate::parse_from_str(date, "%d-%m-%Y").unwrap();
            naive.and_hms(0, 0, 0)
        });

        let tags = sub_args
            .values_of("tags")
            .map_or(Vec::new(), |tags| tags.map(str::to_string).collect());

        ToDo {
            desc,
            start,
            prio,
            due,
            tags,
            recur: None,
            status: Status::Pending,
        }
    };

    Database::from_path(clido_dir()?)?.add(todo).save();

    Ok(())
}

pub fn delete(sub_args: &ArgMatches<'_>) -> Result<()> {
    let id = sub_args
        .value_of("id")
        .with_context(|| "DEL_ID was not provided")?
        .parse::<usize>()
        .with_context(|| "Unable to parse DEL_ID")?;

    Database::from_path(clido_dir()?)?.delete(id).save();

    Ok(())
}

pub fn mark(sub_args: &ArgMatches<'_>) -> Result<()> {
    let id = sub_args
        .value_of("id")
        .with_context(|| "ID was not provided")?
        .parse::<usize>()
        .with_context(|| "Unable to parse ID")?;

    Database::from_path(clido_dir()?)?.mark_complete(id).save();

    Ok(())
}

pub fn list(sub_args: &ArgMatches<'_>) -> Result<()> {
    let db = Database::from_path(clido_dir()?)?;
    let todos = db.todos();

    if todos.len() == 0 {
        println!("\nThere were no To-Dos to print! Good job!\n");
        return Ok(());
    }

    let filters = sub_args
        .values_of("filter")
        .map(|args| args.into_iter().map(str::to_string).collect::<Vec<_>>());

    let req_comp = sub_args.is_present("is_comp");
    let req_pend = sub_args.is_present("is_pend");

    let mut table = get_list_table();
    let mut at_least_one = false;

    for (id, todo) in todos.iter().enumerate() {
        if let Some(filter_list) = filters.as_ref() {
            if !filter_list.iter().any(|filter| todo.tags.contains(filter)) {
                continue;
            }
        }

        if (req_comp && todo.status != Status::Complete)
            || (req_pend && todo.status != Status::Pending)
        {
            continue;
        }

        at_least_one = true;

        let id: String = id.to_string();
        let status = todo.status.as_symbol();
        let start = todo.start.date().to_string();
        let priority = todo
            .prio
            .as_ref()
            .map_or(String::from("None"), |p| p.to_string());
        let due_date = todo
            .due
            .map_or_else(|| String::from("None"), |d| d.date().to_string());

        table.add_row(row![c->id, c->status, l->todo.desc, c->start, c->priority, c->due_date,]);
    }

    if at_least_one {
        println!();
        table.printstd();
        println!();
    } else {
        println!("\nThere were no To-Dos to matching those filters!\n");
    }

    Ok(())
}

fn get_list_table() -> Table {
    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_NO_BORDER);

    table.set_titles(row![
        bc-> "ID",
        bc-> "Status",
        bl->"Description",
        bc-> "Start",
        bc-> "Priority",
        bc-> "Due Date"
    ]);

    table
}
