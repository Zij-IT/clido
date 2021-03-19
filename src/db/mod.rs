mod todo;
use todo::*;
pub use todo::{Priority, Status, ToDo};

use anyhow::{Context, Result};
use tempfile::{NamedTempFile, PersistError};

use std::io::{self, Write};
use std::path::{Path, PathBuf};

use prettytable::Table;

// This part of the code is *heavily* based on the work of a GitHub user ajeetdsouza.
// Clido is using many of the same features to Zoxide (their tool) in order to make things
// work efficiently, and quickly. I highly recommend you give their repo a look
// here: https://github.com/ajeetdsouza/zoxide/

pub struct Database<'a> {
    todos: ToDoList,
    data_dir: &'a Path,
    dirty: bool,
}

impl Database<'_> {
    pub fn save(&mut self) -> Result<()> {
        if !self.dirty {
            return Ok(());
        }

        // Create temp file
        let mut file = NamedTempFile::new_in(&self.data_dir).with_context(|| {
            format!(
                "Could not create temp. database in: {}",
                self.data_dir.display()
            )
        })?;

        let bytes = ron::ser::to_string(&self.todos)?.into_bytes();

        //Attempt to preallocate enough space!
        let _ = file.as_file().set_len(bytes.len() as u64);
        file.write_all(&bytes).with_context(|| {
            format!("Couldn't write to temp database: {}", file.path().display(),)
        })?;

        let path = list_path(&self.data_dir);

        // Replace old with new
        persist(file, &path)
            .with_context(|| format!("Couldn't replace temp database: {}", path.display(),))?;

        self.dirty = false;
        Ok(())
    }

    pub fn add(&mut self, to_add: ToDo) {
        if self.todos.iter_mut().find(|todo| **todo == to_add) == None {
            self.todos.push(to_add);
            self.dirty = true;
        }
        // else let the user know that a similar task already exists
    }

    pub fn delete(&mut self, id: usize) -> bool {
        if let Some(idx) = self.todos.iter().enumerate().position(|(i, _todo)| i == id) {
            self.todos.swap_remove(idx);
            self.dirty = true;
            return true;
        }
        false
    }

    pub fn mark_complete(&mut self, id: usize) -> bool {
        if let Some((_, todo)) = self.todos.iter_mut().enumerate().find(|(i, _todo)| *i == id) {
            todo.status = Status::Complete;
            self.dirty = true;
            return true;
        }
        false
    }

    pub fn list(&self) {
        let mut table = Table::new();
        table.add_row(row![
            "ID",
            "Status",
            "Description",
            "Start",
            "Priority",
            "Due Date"
        ]);

        for (id, todo) in self.todos.iter().enumerate() {
            let priority = match todo.prio {
                Some(Priority::High) => "High",
                Some(Priority::Medium) => "Medium",
                Some(Priority::Low) => "Low",
                _ => "None",
            };

            let status = match todo.status {
                Status::Complete => "✓",
                Status::Pending => "x",
            };

            let start = todo.start.date().naive_local().to_string();

            let id: String = id.to_string();

            table.add_row(row![c->id, c->status, l->todo.desc, c->start, c->priority, c->"None",]);
        }

        table.printstd();
    }
}

impl Drop for Database<'_> {
    fn drop(&mut self) {
        if let Err(e) = self.save() {
            let _ = writeln!(io::stderr(), "Clido: {:?}", e);
        }
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

pub struct DatabaseFile {
    data_dir: PathBuf,
    buffer: Vec<u8>,
}

impl DatabaseFile {
    pub fn new<P: Into<PathBuf>>(data_dir: P) -> Self {
        Self {
            data_dir: data_dir.into(),
            buffer: Vec::new(),
        }
    }

    pub fn open(&mut self) -> Result<Database> {
        let path = list_path(&self.data_dir);
        match std::fs::read(&path) {
            Ok(buffer) => {
                self.buffer = buffer;
                let todos = ron::de::from_bytes(&self.buffer).with_context(|| {
                    format!("Could not deserialize todo-list: {}", path.display())
                })?;

                Ok(Database {
                    todos,
                    data_dir: &self.data_dir,
                    dirty: false,
                })
            }
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                std::fs::create_dir_all(&self.data_dir).expect("Unable to create dir");

                Ok(Database {
                    todos: Vec::new().into(),
                    data_dir: &self.data_dir,
                    dirty: false,
                })
            }
            Err(e) => {
                Err(e).with_context(|| format!("could not read from database: {}", path.display()))
            }
        }
    }
}

fn list_path<P: AsRef<Path>>(data_dir: P) -> std::path::PathBuf {
    const DB_FILENAME: &str = "clido.db";
    data_dir.as_ref().join(DB_FILENAME)
}
