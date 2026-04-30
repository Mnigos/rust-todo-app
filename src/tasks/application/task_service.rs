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

    pub fn list_all(&self) -> Result<Vec<Task>, TaskServiceError> {
        let tasks = self
            .repository
            .load()
            .map_err(TaskServiceError::RepositoryFailed)?;

        Ok(tasks)
    }

    pub fn add_task(&self, title: String) -> Result<Task, TaskServiceError> {
        let mut tasks = self
            .repository
            .load()
            .map_err(TaskServiceError::RepositoryFailed)?;

        let next_id = tasks
            .iter()
            .map(|task| task.id().value())
            .max()
            .unwrap_or(0)
            + 1;
        let task_id = TaskId::new(next_id);
        let task_title = TaskTitle::new(title).map_err(TaskServiceError::InvalidTaskTitle)?;
        let task = Task::new(task_id, task_title);

        tasks.push(task.clone());

        self.repository
            .save(&tasks)
            .map_err(TaskServiceError::RepositoryFailed)?;

        Ok(task)
    }

    pub fn complete_task(&self, id: &TaskId) -> Result<(), TaskServiceError> {
        let mut tasks = self
            .repository
            .load()
            .map_err(TaskServiceError::RepositoryFailed)?;

        let Some(task) = tasks.iter_mut().find(|task| task.id() == id) else {
            return Err(TaskServiceError::TaskNotFound);
        };

        task.complete();

        self.repository
            .save(&tasks)
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
