#![allow(clippy::use_self)]

use crate::todo::ToDoUpdate;

pub use super::todo::{Priority, Status, ToDo, ToDoList};
use anyhow::{bail, Context, Result};
use tempfile::{NamedTempFile, PersistError};

use std::io::Write;
use std::marker::PhantomData;
use std::path::{Path, PathBuf};

// Typestate :) Thanks u/ponkyol
pub trait State {}
pub struct Clean;
pub struct Dirty;
impl State for Clean {}
impl State for Dirty {}

pub struct Database<S: State> {
    todos: ToDoList,
    data_dir: PathBuf,
    status: PhantomData<S>,
}

impl<S: State> Database<S> {
    pub fn add(mut self, todo: ToDo) -> Database<Dirty> {
        self.todos.push(todo);
        println!("Successfully added the item.");

        Database::<Dirty> {
            todos: self.todos,
            data_dir: self.data_dir,
            status: PhantomData::default(),
        }
    }

    pub fn delete(mut self, id: usize) -> Database<Dirty> {
        if let Some(idx) = self.todos.iter().enumerate().position(|(i, _todo)| i == id) {
            self.todos.remove(idx);
            println!("Successfully removed the item");
        }

        Database::<Dirty> {
            todos: self.todos,
            data_dir: self.data_dir,
            status: PhantomData::default(),
        }
    }

    pub fn update(mut self, id: usize, update: ToDoUpdate) -> Database<Dirty> {
        if let Some(todo) = self.todos.get_mut(id) {
            if let Some(desc) = update.desc {
                todo.desc = desc;
            }

            if let Some(due) = update.due {
                todo.due = due;
            }

            if let Some(start) = update.start {
                todo.start = start;
            }

            if let Some(prio) = update.prio {
                todo.prio = prio;
            }

            if let Some(tags) = update.tags {
                todo.tags = tags;
            }

            if let Some(recur) = update.recur {
                todo.recur = recur;
            }

            if let Some(status) = update.status {
                todo.status = status;
            }
        }

        Database::<Dirty> {
            todos: self.todos,
            data_dir: self.data_dir,
            status: PhantomData::default(),
        }
    }

    pub fn todos(&self) -> &ToDoList {
        &self.todos
    }
}

impl Database<Clean> {
    pub fn from_clido_dir() -> Result<Self> {
        Self::from_path(clido_dir()?)
    }

    pub fn from_path<P: Into<PathBuf>>(data_dir: P) -> Result<Self> {
        let data_dir = data_dir.into();
        let path = list_path(&data_dir);
        match std::fs::read(&path) {
            Ok(buffer) => {
                let todos = bincode::deserialize(&buffer).with_context(|| {
                    format!("Could not deserialize todo-list: {}", path.display())
                })?;

                Ok(Self {
                    todos,
                    data_dir,
                    status: PhantomData::default(),
                })
            }
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                std::fs::create_dir_all(&data_dir).expect("Unable to create dir");

                Ok(Self {
                    todos: Vec::new().into(),
                    data_dir,
                    status: PhantomData::default(),
                })
            }
            Err(e) => {
                Err(e).with_context(|| format!("could not read from database: {}", path.display()))
            }
        }
    }
}

impl Database<Dirty> {
    pub fn save(&self) -> Result<()> {
        let mut file = NamedTempFile::new_in(&self.data_dir).with_context(|| {
            format!(
                "Could not create temp. database in: {}",
                self.data_dir.display()
            )
        })?;

        let bytes = bincode::serialize(&self.todos)?;

        //Attempt to preallocate enough space!
        let _ = file.as_file().set_len(bytes.len() as u64);
        file.write_all(&bytes).with_context(|| {
            format!("Couldn't write to temp database: {}", file.path().display(),)
        })?;

        let path = list_path(&self.data_dir);

        // Replace old with new
        persist(file, &path)
            .with_context(|| format!("Couldn't replace temp database: {}", path.display(),))?;

        Ok(())
    }
}

#[cfg(windows)]
fn persist<P: AsRef<Path>>(mut file: NamedTempFile, path: P) -> Result<(), PersistError> {
    use rand::distributions::{Distribution, Uniform};
    use std::thread;
    use std::time::Duration;

    // File renames on Windows are not atomic and sometimes fail with `PermissionDenied`.
    // This is extremely unlikely unless it's running in a loop on multiple threads.
    // Nevertheless, we guard against it by retrying the rename a fixed number of times.
    const MAX_TRIES: usize = 10;
    let mut rng = None;

    for _ in 0..MAX_TRIES {
        match file.persist(&path) {
            Ok(_) => break,
            Err(e) if e.error.kind() == io::ErrorKind::PermissionDenied => {
                let mut rng = rng.get_or_insert_with(rand::thread_rng);
                let between = Uniform::from(50..150);
                let duration = Duration::from_millis(between.sample(&mut rng));
                thread::sleep(duration);
                file = e.file;
            }
            Err(e) => return Err(e),
        }
    }

    Ok(())
}

#[cfg(unix)]
fn persist<P: AsRef<Path>>(file: NamedTempFile, path: P) -> Result<(), PersistError> {
    file.persist(&path)?;
    Ok(())
}

fn list_path<P: AsRef<Path>>(data_dir: P) -> std::path::PathBuf {
    const DB_FILENAME: &str = "clido.db";
    data_dir.as_ref().join(DB_FILENAME)
}

fn clido_dir() -> Result<PathBuf> {
    let data_dir = match std::env::var_os("_CLIDO_DIR") {
        Some(os_str) => PathBuf::from(os_str),
        None => match dirs_next::data_local_dir() {
            Some(mut dir) => {
                dir.push("clido");
                dir
            }
            None => bail!("Could not find data directory. Please set _CLIDO_DIR."),
        },
    };

    Ok(data_dir)
}
