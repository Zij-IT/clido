use super::db::{DatabaseFile, Priority, Status, ToDo};
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
    let todo = {
        let desc = sub_args
            .value_of("todo")
            .expect("What. This one is required, so something broke.")
            .to_string();

        let start = sub_args.value_of("start").map_or_else(
            || Local::now().naive_local(),
            |date| {
                let naive = NaiveDate::parse_from_str(date, "%d-%m-%Y").unwrap();
                naive.and_hms(0, 0, 0)
            },
        );

        let prio = Priority::try_from(sub_args.value_of("priority")).ok();

        let due = sub_args.value_of("due_date").map(|date| {
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

    // TodoListFile still needs to be in scope for the database to function,
    // so I am shadowing the variable to make sure that it is impossible to
    // drop the database file first.
    let mut db = DatabaseFile::new(clido_dir()?);
    let mut db = db.open()?;
    db.add(todo);

    println!("Item successfully added to the list!");
    Ok(())
}

pub fn delete(sub_args: &ArgMatches<'_>) -> Result<()> {
    let mut db = DatabaseFile::new(clido_dir()?);
    let mut db = db.open()?;

    let id = sub_args
        .value_of("id")
        .with_context(|| "DEL_ID was not provided")?
        .parse::<usize>()
        .with_context(|| "Unable to parse DEL_ID")?;

    if db.delete(id) {
        println!("Item successfully removed from the list!");
    } else {
        println!("The ID {} was not an ID for a ToDo", id);
    }

    Ok(())
}

pub fn mark(sub_args: &ArgMatches<'_>) -> Result<()> {
    let mut db = DatabaseFile::new(clido_dir()?);
    let mut db = db.open()?;

    let id = sub_args
        .value_of("id")
        .with_context(|| "DEL_ID was not provided")?
        .parse::<usize>()
        .with_context(|| "Unable to parse DEL_ID")?;

    if db.mark_complete(id) {
        println!("Item successfully marked!");
    } else {
        println!("The ID {} was not an ID for a ToDo", id);
    }

    Ok(())
}

pub fn list(sub_args: &ArgMatches<'_>) -> Result<()> {
    let mut db = DatabaseFile::new(clido_dir()?);
    let db = db.open()?;
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
    let mut hit = false;

    for (id, todo) in todos.iter().enumerate() {
        if let Some(filter_list) = filters.as_ref() {
            if !filter_list.iter().any(|filter| todo.tags.contains(filter)) {
                continue;
            }
        }

        if (req_comp && todo.status != Status::Complete) || (req_pend && todo.status != Status::Pending) {
            continue;
        }

        hit = true;

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

    if hit {
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
