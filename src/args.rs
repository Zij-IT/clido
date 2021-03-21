use chrono::NaiveDate;
use clap::arg_enum;

arg_enum! {
    #[allow(non_camel_case_types)]
    #[derive(PartialEq)]
    pub enum Priority {
        high,
        mid,
        low,
    }
}

#[allow(clippy::needless_pass_by_value)]
pub fn valid_priority(prio: String) -> Result<(), String> {
    if Priority::variants().contains(&&*prio) {
        Ok(())
    } else {
        Err(format!("Valid options are {:?}", Priority::variants()))
    }
}

#[allow(clippy::needless_pass_by_value)]
pub fn valid_date(date: String) -> Result<(), String> {
    if NaiveDate::parse_from_str(&date, "%d-%m-%Y").is_ok() {
        Ok(())
    } else {
        Err(String::from(
            "Valid options are formatted as such: dd-mm-yyyy",
        ))
    }
}

arg_enum! {
    #[allow(non_camel_case_types)]
    #[derive(PartialEq)]
    pub enum Recurrence {
        daily,
        weekly,
        monthly,
        yearly,
    }
}
