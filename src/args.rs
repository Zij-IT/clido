use chrono::NaiveDate;

// I would love the pass the values by reference,
// however doing that prevents it from compiling, as clap
// expects a function fn(String)->Result<(), String>

#[allow(clippy::needless_pass_by_value)]
pub fn valid_priority(prio: String) -> Result<(), String> {
    static PRIORITIES: [&str; 3] = ["high", "mid", "low"];
    if PRIORITIES.contains(&&*prio) {
        Ok(())
    } else {
        Err(format!("Valid options are {:?}", PRIORITIES))
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
