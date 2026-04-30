use std::{
    env::{self, VarError},
    path::{Path, PathBuf},
};

pub struct AppConfig {
    tasks_file_path: PathBuf,
}

impl AppConfig {
    pub fn from_env() -> Result<Self, AppConfigError> {
        let tasks_file_path = env::var("TASKS_FILE_PATH").map_err(AppConfigError::InvalidEnvVar)?;

        Ok(Self {
            tasks_file_path: PathBuf::from(tasks_file_path),
        })
    }

    pub fn tasks_file_path(&self) -> &Path {
        &self.tasks_file_path
    }
}

#[derive(Debug)]
pub enum AppConfigError {
    InvalidEnvVar(VarError),
}
