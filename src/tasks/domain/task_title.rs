use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TaskTitle(String);

impl TaskTitle {
    pub fn new(value: String) -> Result<Self, TaskTitleError> {
        let trimmed_value = value.trim();

        if trimmed_value.is_empty() {
            return Err(TaskTitleError::Empty);
        }

        Ok(Self(trimmed_value.to_string()))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

#[derive(Debug)]
pub enum TaskTitleError {
    Empty,
}
