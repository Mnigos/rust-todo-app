use std::env::{self, VarError};

pub struct AppConfig {
    database_url: String,
}

impl AppConfig {
    pub fn from_env() -> Result<Self, AppConfigError> {
        let database_url = env::var("DATABASE_URL").map_err(AppConfigError::InvalidEnvVar)?;

        Ok(Self { database_url })
    }

    pub fn database_url(&self) -> &str {
        &self.database_url
    }
}

#[derive(Debug)]
pub enum AppConfigError {
    InvalidEnvVar(VarError),
}
