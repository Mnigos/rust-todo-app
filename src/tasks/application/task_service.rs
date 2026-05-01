use crate::tasks::{
    domain::{Task, TaskId, TaskTitle, TaskTitleError},
    infrastructure::{TaskRepository, TaskRepositoryError},
};

pub struct TaskService {
    repository: TaskRepository,
}

impl TaskService {
    pub fn new(repository: TaskRepository) -> Self {
        Self { repository }
    }

    pub async fn list_all(&self) -> Result<Vec<Task>, TaskServiceError> {
        let tasks = self
            .repository
            .find_all()
            .await
            .map_err(TaskServiceError::RepositoryFailed)?;

        Ok(tasks)
    }

    pub async fn add_task(&self, title: String) -> Result<Task, TaskServiceError> {
        let task_title = TaskTitle::new(title).map_err(TaskServiceError::InvalidTaskTitle)?;

        let task = self
            .repository
            .insert(&task_title)
            .await
            .map_err(TaskServiceError::RepositoryFailed)?;

        Ok(task)
    }

    pub async fn complete_task(&self, id: &TaskId) -> Result<(), TaskServiceError> {
        let Some(mut task) = self
            .repository
            .find_by_id(id)
            .await
            .map_err(TaskServiceError::RepositoryFailed)?
        else {
            return Err(TaskServiceError::TaskNotFound);
        };

        task.complete();

        self.repository
            .update(&task)
            .await
            .map_err(TaskServiceError::RepositoryFailed)?;

        Ok(())
    }
}

#[derive(Debug)]
pub enum TaskServiceError {
    RepositoryFailed(TaskRepositoryError),
    InvalidTaskTitle(TaskTitleError),
    TaskNotFound,
}
