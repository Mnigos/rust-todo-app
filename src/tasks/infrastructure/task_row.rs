use uuid::Uuid;

use crate::tasks::domain::{Task, TaskId, TaskTitle, TaskTitleError};

pub struct TaskRow {
    pub id: Uuid,
    pub title: String,
    pub is_completed: bool,
}

impl TryFrom<TaskRow> for Task {
    type Error = TaskRowMappingError;

    fn try_from(row: TaskRow) -> Result<Self, Self::Error> {
        let id = TaskId::new(row.id);
        let title = TaskTitle::new(row.title).map_err(TaskRowMappingError::InvalidTitle)?;

        Ok(Task::restore(id, title, row.is_completed))
    }
}

impl From<&Task> for TaskRow {
    fn from(task: &Task) -> Self {
        Self {
            id: task.id().value(),
            title: task.title().value().to_string(),
            is_completed: task.is_completed(),
        }
    }
}

#[derive(Debug)]
pub enum TaskRowMappingError {
    InvalidTitle(TaskTitleError),
}
