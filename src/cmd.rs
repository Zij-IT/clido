use super::db::{DatabaseFile, Priority, Status, ToDo};
use super::util::clido_dir;

use anyhow::{Context, Result};
use chrono::{Local, NaiveDate, TimeZone};
use clap::ArgMatches;
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

        let start = sub_args
            .value_of("start")
            .map_or_else(Local::now, |date| {
                let naive = NaiveDate::parse_from_str(date, "%d-%m-%Y").unwrap();
                Local
                    .from_local_datetime(&(naive.and_hms(0, 0, 0))).unwrap()
            });

        let prio = Priority::try_from(sub_args.value_of("priority")).ok();

        let due = sub_args.value_of("due_date").map(|date| {
            let naive = NaiveDate::parse_from_str(date, "%d-%m-%Y").unwrap();
            Local
                .from_local_datetime(&(naive.and_hms(0, 0, 0))).unwrap()
        });

        ToDo {
            desc,
            start,
            prio,
            due,
            status: Status::Pending,
            tags: vec![],
            recur: None,
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

pub fn list() -> Result<()> {
    let mut db = DatabaseFile::new(clido_dir()?);

    db.open()?.list();

    Ok(())
}
