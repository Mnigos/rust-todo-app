use crate::tasks::domain::Task;
#[cfg(feature = "ssr")]
use crate::tasks::{application::TaskService, domain::TaskId, infrastructure::TaskRepository};
use leptos::prelude::*;
#[cfg(feature = "ssr")]
use std::path::PathBuf;

#[cfg(feature = "ssr")]
fn create_task_service() -> TaskService {
    let repository = TaskRepository::new(PathBuf::from("data/tasks.json"));

    TaskService::new(repository)
}

#[server]
pub async fn list_tasks() -> Result<Vec<Task>, ServerFnError> {
    let service = create_task_service();

    service
        .list_all()
        .map_err(|err| ServerFnError::new(format!("{err:?}")))
}

#[server]
pub async fn add_task(title: String) -> Result<Task, ServerFnError> {
    let service = create_task_service();

    service
        .add_task(title)
        .map_err(|err| ServerFnError::new(format!("{err:?}")))
}

#[server]
pub async fn complete_task(id: u64) -> Result<(), ServerFnError> {
    let service = create_task_service();

    let task_id = TaskId::new(id);
    service
        .complete_task(&task_id)
        .map_err(|err| ServerFnError::new(format!("{err:?}")))
}
