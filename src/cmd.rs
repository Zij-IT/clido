use super::db::{DatabaseFile, Priority, Status, ToDo};
use super::util::clido_dir;

use anyhow::{Context, Result};
use chrono::Local;
use clap::ArgMatches;

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

        #[allow(clippy::match_same_arms)]
        let start = match sub_args.value_of("start_date") {
            Some(_date) => Local::now(), //todo: this should have a list of acceptable values
            None => Local::now(),
        };

        // Although there are only four (high, medium, low, NONE) possible values allowed for priority
        // there is no reason to panic should the value SOMEHOW be not among the
        // listed values. Just default to none, and inform the user that the priority
        // may not be correctly set
        // todo: If the value is Some(_) inform the user that the priority may not be correctly set.

        let prio = match sub_args.value_of("priority") {
            Some("high") => Some(Priority::High),
            Some("medium") => Some(Priority::Medium),
            Some("low") => Some(Priority::Low),
            Some(_) | None => None,
        };

        // todo: Define some values that can be used to assign a due date.
        // Examples of possible acceptable input
        //  - clido add -p high -d [tomorrow, week, two-weeks, month]
        //  - clido add -p medium -d 23.12.2020
        let due = None;

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
