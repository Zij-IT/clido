use super::{clido_dir, Database, Local, NaiveDate, Priority, Result, Status, ToDo};
use chrono::{Datelike, NaiveDateTime};
use clap::Args;
use std::convert::TryFrom;

const SHORT_DAYS: [&str; 7] = ["mon", "tue", "wed", "thu", "fri", "sat", "sun"];

#[derive(Debug, Args)]
#[command(arg_required_else_help = true)]
pub struct Add {
    #[arg(value_name = "TODO", required = true)]
    todo: String,

    #[arg(value_name = "PRIORITY")]
    priority: Option<String>,

    #[arg(value_name = "START_DATE")]
    start_date: Option<String>,

    #[arg(value_name = "DUE_DATE")]
    due_date: Option<String>,

    #[arg(value_name = "TAGS")]
    tags: Option<Vec<String>>,
}

pub fn add(command: &Add) -> Result<()> {
    let todo = {
        let desc = command.todo.clone();

        let start: NaiveDateTime = command
            .start_date
            .as_deref()
            .map_or_else(|| Local::now().naive_local(), date_from_input);

        let due = command.due_date.as_deref().map(date_from_input);

        let prio = Priority::try_from(command.priority.as_deref()).ok();

        let tags = command.tags.as_ref().map_or(Vec::new(), Clone::clone);

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
