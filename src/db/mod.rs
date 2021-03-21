mod db_file;
mod todo;

pub use db_file::DatabaseFile;
use todo::ToDoList;
pub use todo::{Priority, Status, ToDo};

use anyhow::{Context, Result};
use prettytable::{format, Table};
use tempfile::{NamedTempFile, PersistError};

use std::io::{self, Write};
use std::path::Path;

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
        if let Some((_, todo)) = self
            .todos
            .iter_mut()
            .enumerate()
            .find(|(i, _todo)| *i == id)
        {
            todo.status = Status::Complete;
            self.dirty = true;
            return true;
        }
        false
    }

    pub fn list(&self) {
        if self.todos.len() == 0 {
            println!("\nThere were no To-Dos to print! Good job!\n");
            return;
        }

        let mut table = Table::new();
        table.set_format(*format::consts::FORMAT_NO_BORDER);

        table.set_titles(row![
            bc-> "ID",
            bc-> "Status",
            bl->"Description",
            bc-> "Start",
            bc-> "Priority",
            bc-> "Due Date"
        ]);

        println!();
        for (id, todo) in self.todos.iter().enumerate() {
            let priority = match todo.prio {
                Some(Priority::High) => "High",
                Some(Priority::Medium) => "Medium",
                Some(Priority::Low) => "Low",
                _ => "None",
            };

            let status = match todo.status {
                Status::Complete => "\u{2713}",
                Status::Pending => "x",
            };

            let start = todo.start.date().to_string();

            let id: String = id.to_string();

            let due_date = todo
                .due
                .map_or_else(|| String::from("None"), |d| d.date().to_string());

            table
                .add_row(row![c->id, c->status, l->todo.desc, c->start, c->priority, c->due_date,]);
        }
        table.printstd();
        println!();
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

fn list_path<P: AsRef<Path>>(data_dir: P) -> std::path::PathBuf {
    const DB_FILENAME: &str = "clido.db";
    data_dir.as_ref().join(DB_FILENAME)
}
