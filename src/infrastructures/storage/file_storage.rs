use serde_json::{from_reader, to_writer_pretty};
use std::{
    error::Error,
    fs::{File, OpenOptions},
    io::{BufReader, BufWriter, ErrorKind},
};

use crate::domains::{common::storage::Storage, task::entity::TaskEntity};

pub struct FileStorage {
    file_path: String,
}

impl FileStorage {
    pub fn new(file_path: String) -> Self {
        Self { file_path }
    }
}

impl Storage for FileStorage {
    fn save(&self, tasks: &[TaskEntity]) -> Result<(), Box<dyn Error>> {
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&self.file_path)?;

        let writer = BufWriter::new(file);
        to_writer_pretty(writer, tasks)?;

        Ok(())
    }

    fn load(&self) -> Result<Vec<TaskEntity>, Box<dyn Error>> {
        let file = match File::open(&self.file_path) {
            Ok(f) => f,
            Err(e) if e.kind() == ErrorKind::NotFound => {
                // File doesn't exist → treat as empty
                return Ok(Vec::new());
            }
            Err(e) => return Err(Box::new(e)),
        };

        let reader = BufReader::new(file);
        match from_reader(reader) {
            Ok(tasks) => Ok(tasks),
            Err(e) if e.is_eof() => {
                // File is empty → treat as empty list
                Ok(Vec::new())
            }
            Err(e) => Err(Box::new(e)),
        }
    }
}
