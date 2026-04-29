use crate::tasks::domain::task::Task;
use std::{fs, io, path::PathBuf};

pub struct TaskRepository {
    file_path: PathBuf,
}

impl TaskRepository {
    pub fn new(file_path: PathBuf) -> Self {
        Self { file_path }
    }

    pub fn load(&self) -> Result<Vec<Task>, TaskRepositoryError> {
        self.ensure_file_exists()?;

        let json = fs::read_to_string(&self.file_path).map_err(TaskRepositoryError::ReadFailed)?;

        serde_json::from_str(&json).map_err(TaskRepositoryError::DeserializeFailed)
    }

    pub fn save(&self, tasks: &[Task]) -> Result<(), TaskRepositoryError> {
        self.ensure_file_exists()?;

        let json =
            serde_json::to_string_pretty(tasks).map_err(TaskRepositoryError::SerializeFailed)?;

        fs::write(&self.file_path, json).map_err(TaskRepositoryError::WriteFailed)?;

        Ok(())
    }

    fn ensure_file_exists(&self) -> Result<(), TaskRepositoryError> {
        if let Some(parent_dir) = self.file_path.parent() {
            fs::create_dir_all(parent_dir).map_err(TaskRepositoryError::WriteFailed)?;
        }

        if !self.file_path.exists() {
            fs::write(&self.file_path, "[]").map_err(TaskRepositoryError::WriteFailed)?;
        }

        Ok(())
    }
}

#[derive(Debug)]
pub enum TaskRepositoryError {
    ReadFailed(io::Error),
    WriteFailed(io::Error),
    SerializeFailed(serde_json::Error),
    DeserializeFailed(serde_json::Error),
}
