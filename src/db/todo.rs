use chrono::{DateTime, Local};
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

#[derive(PartialEq, Serialize, Deserialize)]
pub struct ToDo {
    // Required
    pub desc: String,
    pub start: DateTime<Local>,
    pub status: Status,

    // Optional
    pub prio: Option<Priority>,
    pub due: Option<DateTime<Local>>,
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
