use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::ops::{Deref, DerefMut};

#[derive(Serialize, Deserialize)]
pub struct ToDoList(Vec<ToDo>);

impl Deref for ToDoList {
    type Target = Vec<ToDo>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ToDoList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<Vec<ToDo>> for ToDoList {
    fn from(todos: Vec<ToDo>) -> Self {
        Self(todos)
    }
}

// What makes a to-do:
// Req:
// - Description    -> String
// - Start Date     -> DateTime<Local>
// - Status         -> StatusEnum
// Optional:        -> Option<X>
// - Tags               -> Vec<String>
// - Due Date           -> DateTime<Local>
// - Priority           -> PriorityEnum
// - Recur              -> String
//   - Requires "until" -> DateTime<Local>

#[derive(PartialEq, Serialize, Deserialize)]
pub struct ToDo {
    // Optional
    pub recur: Option<Recurrence>,
    pub due: Option<NaiveDateTime>,
    pub prio: Option<Priority>,

    // Required
    pub start: NaiveDateTime,
    pub desc: String,
    pub status: Status,
    pub tags: Vec<String>, // Allowed to be empty
}

#[derive(PartialEq, Serialize, Deserialize)]
pub enum Status {
    Complete,
    Pending,
}

#[derive(PartialEq, Serialize, Deserialize)]
pub enum Priority {
    Low,
    Medium,
    High,
}

impl TryFrom<Option<&str>> for Priority {
    type Error = ();

    fn try_from(string: Option<&str>) -> Result<Self, Self::Error> {
        match string {
            Some("high") => Ok(Self::High),
            Some("medium") => Ok(Self::Medium),
            Some("low") => Ok(Self::Low),
            _ => Err(()),
        }
    }
}

#[derive(PartialEq, Serialize, Deserialize)]
pub enum Recurrence {
    Daily(NaiveDateTime),
    Weekly(NaiveDateTime),
    Monthly(NaiveDateTime),
    Yearly(NaiveDateTime),
    Quarterly(NaiveDateTime),
}
