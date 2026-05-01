use super::task_id::TaskId;
use super::task_title::TaskTitle;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Task {
    id: TaskId,
    title: TaskTitle,
    is_completed: bool,
}

impl Task {
    pub fn new(id: TaskId, title: TaskTitle) -> Self {
        Self {
            id,
            title,
            is_completed: false,
        }
    }

    #[cfg(feature = "ssr")]
    pub(in crate::tasks) fn restore(id: TaskId, title: TaskTitle, is_completed: bool) -> Self {
        Self {
            id,
            title,
            is_completed,
        }
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
