use super::{
    super::args::SHORT_DAYS, clido_dir, ArgMatches, Database, Local, NaiveDate, Priority, Result,
    Status, ToDo,
};
use chrono::{Datelike, NaiveDateTime, NaiveTime};
use std::convert::TryFrom;

pub fn add(sub_args: &ArgMatches<'_>) -> Result<()> {
    //Construct a To-Do based on the arguments passed
    let todo = {
        // Safety: Unwrap is safe because clap enforces that this
        // argument is present
        let desc = sub_args.value_of("todo").unwrap().to_string();

        let start: NaiveDateTime = sub_args
            .value_of("start")
            .map_or_else(|| Local::now().naive_local(), date_from_input);
        let due = sub_args.value_of("due_date").map(date_from_input);

        let prio = Priority::try_from(sub_args.value_of("priority")).ok();

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
    // Goal must match one of the &str above,
    // as the input is checked via Clap
    let goal = &desired_date.to_ascii_lowercase()[..3];
    debug_assert!(SHORT_DAYS.contains(&goal));

    let mut date = curr_date.date();

    while let Some(next_date) = date.succ_opt() {
        date = next_date;
        // Days are mapped such that Monday = 0, .. Sunday = 6
        if SHORT_DAYS[(date.weekday()) as usize] == goal {
            return Some(NaiveDateTime::new(date, NaiveTime::from_hms(0, 0, 0)));
        }
    }

    println!("Welcome to the end of the year 262143CE.\n This is about where CLIDO stops. Sorry.");
    None
}
