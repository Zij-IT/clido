use chrono::NaiveDateTime;
use clap::ValueEnum;
use serde::{Deserialize, Serialize};
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

#[derive(PartialEq, Eq, Serialize, Deserialize)]
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

#[derive(Default)]
pub struct ToDoUpdate {
    pub due: Option<Option<NaiveDateTime>>,
    pub prio: Option<Option<Priority>>,
    pub recur: Option<Option<Recurrence>>,

    pub desc: Option<String>,
    pub status: Option<Status>,
    pub start: Option<NaiveDateTime>,
    pub tags: Option<Vec<String>>,
}

impl ToDoUpdate {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_status(mut self, status: Status) -> Self {
        self.status = Some(status);
        self
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize, ValueEnum)]
pub enum Status {
    Complete,
    Pending,
}

impl Status {
    pub const fn as_symbol(&self) -> &str {
        match self {
            Self::Complete => "\u{2713}",
            Self::Pending => "x",
        }
    }
}

#[derive(Ord, PartialOrd, Debug, PartialEq, Eq, Serialize, Deserialize, Clone, Copy, ValueEnum)]
pub enum Priority {
    Low,
    #[value(alias("mid"))]
    #[value(alias("med"))]
    Medium,
    High,
}

impl ToString for Priority {
    fn to_string(&self) -> String {
        String::from(match self {
            Self::Low => "Low",
            Self::Medium => "Medium",
            Self::High => "High",
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Recurrence {
    Daily(NaiveDateTime),
    Weekly(NaiveDateTime),
    Monthly(NaiveDateTime),
    Yearly(NaiveDateTime),
    Quarterly(NaiveDateTime),
}
