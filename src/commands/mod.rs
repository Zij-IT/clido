mod add;
mod del;
mod list;
mod mark;
mod modify;

use super::db::{Database, Priority, Status, ToDo};

use anyhow::Result;
use chrono::{Datelike, Local, NaiveDate, NaiveDateTime};
use prettytable::{format, Table};

pub use add::{add, Add};
pub use del::{del, Delete};
pub use list::{list, List};
pub use mark::{mark, Mark};
pub use modify::{modify, Modify};

const SHORT_DAYS: [&str; 7] = ["mon", "tue", "wed", "thu", "fri", "sat", "sun"];

fn date_from_input(date: &str) -> NaiveDateTime {
    if SHORT_DAYS.contains(&&date[..3]) {
        let now = Local::now().naive_local();
        day_to_date(&now, date).unwrap_or(now)
    } else {
        let naive = NaiveDate::parse_from_str(date, "%d-%m-%Y").unwrap();
        naive.and_time(Local::now().naive_local().time())
    }
}

fn day_to_date(curr_date: &NaiveDateTime, desired_date: &str) -> Option<NaiveDateTime> {
    let goal = &desired_date.to_ascii_lowercase()[..3];
    curr_date
        .date()
        .iter_days()
        .find(|x| SHORT_DAYS[x.weekday() as usize] == goal)
        .map(|x| x.and_time(Local::now().naive_local().time()))
}
