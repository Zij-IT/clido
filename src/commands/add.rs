use super::{
    super::args::SHORT_DAYS, clido_dir, ArgMatches, Database, Local, NaiveDate, Priority, Result,
    Status, ToDo,
};
use chrono::{Datelike, NaiveDateTime};
use clap::Args;
use std::{convert::TryFrom, ffi::OsString};

#[derive(Debug, Args)]
#[command(arg_required_else_help = true)]
pub struct Add {
    #[arg(value_name = "TODO", required = true)]
    todo: String,

    #[arg(value_name = "PRIORITY")]
    priority: Option<String>,

    #[arg(value_name = "PRIORITY")]
    start_date: Option<String>,

    #[arg(value_name = "PRIORITY")]
    due_date: Option<String>,

    #[arg(value_name = "PRIORITY")]
    tags: Option<Vec<String>>,
}

pub fn add(command: &Add) -> Result<()> {
    //Construct a To-Do based on the arguments passed
    let todo = {
        // Safety: Unwrap is safe because clap enforces that this
        // argument is present
        let desc = command.todo.clone();

        let start: NaiveDateTime = command
            .start_date
            .as_deref()
            .map_or_else(|| Local::now().naive_local(), date_from_input);

        let due = command.due_date.as_deref().map(date_from_input);

        let prio = Priority::try_from(command.priority.as_deref()).ok();

        let tags = command
            .tags
            .as_ref()
            .map_or(Vec::new(), |tags| tags.clone());

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

fn date_from_input(date: &str) -> NaiveDateTime {
    if SHORT_DAYS.contains(&&date[..3]) {
        let now = Local::now().naive_local();
        day_to_date(&now, date).unwrap_or(now)
    } else {
        let naive = NaiveDate::parse_from_str(date, "%d-%m-%Y").unwrap();
        naive.and_hms(0, 0, 0)
    }
}

fn day_to_date(curr_date: &NaiveDateTime, desired_date: &str) -> Option<NaiveDateTime> {
    let goal = &desired_date.to_ascii_lowercase()[..3];
    curr_date
        .date()
        .iter_days()
        .find(|x| SHORT_DAYS[x.weekday() as usize] == goal)
        .map(|x| x.and_hms(0, 0, 0))
}
