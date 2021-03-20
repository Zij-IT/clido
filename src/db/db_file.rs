use std::path::PathBuf;
use super::{Database, list_path};

use anyhow::{Result, Context};

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

    pub fn open(&mut self) -> Result<Database<'_>> {
        let path = list_path(&self.data_dir);
        match std::fs::read(&path) {
            Ok(buffer) => {
                self.buffer = buffer;
                let todos = bincode::deserialize(&self.buffer).with_context(|| {
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