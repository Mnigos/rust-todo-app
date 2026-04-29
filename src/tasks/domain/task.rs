use super::task_id::TaskId;
use super::task_title::{TaskTitle, TaskTitleError};

#[derive(Clone)]
pub struct Task {
    id: TaskId,
    title: TaskTitle,
    is_completed: bool,
}

impl Task {
    pub fn new(id: TaskId, title: String) -> Result<Self, TaskTitleError> {
        Ok(Self {
            id,
            title: TaskTitle::new(title)?,
            is_completed: false,
        })
    }

    pub fn id(&self) -> &TaskId {
        &self.id
    }

    pub fn title(&self) -> &TaskTitle {
        &self.title
    }

    pub fn is_completed(&self) -> bool {
        self.is_completed
    }

    pub fn complete(&mut self) {
        self.is_completed = true
    }
}
