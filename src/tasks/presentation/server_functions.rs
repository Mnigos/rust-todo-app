use crate::tasks::domain::Task;
#[cfg(feature = "ssr")]
use crate::{
    config::AppConfig,
    tasks::{application::TaskService, domain::TaskId, infrastructure::TaskRepository},
};
use leptos::prelude::*;
use std::fmt::Debug;

fn server_error(err: impl Debug) -> ServerFnError {
    ServerFnError::new(format!("{err:?}"))
}

#[cfg(feature = "ssr")]
fn create_task_service() -> Result<TaskService, ServerFnError> {
    let app_config = AppConfig::from_env().map_err(server_error)?;
    let repository = TaskRepository::new(app_config.tasks_file_path());

    Ok(TaskService::new(repository))
}

#[server]
pub async fn list_tasks() -> Result<Vec<Task>, ServerFnError> {
    let service = create_task_service()?;

    service.list_all().map_err(server_error)
}

#[server]
pub async fn add_task(title: String) -> Result<Task, ServerFnError> {
    let service = create_task_service()?;

    service.add_task(title).map_err(server_error)
}

#[server]
pub async fn complete_task(id: u64) -> Result<(), ServerFnError> {
    let service = create_task_service()?;

    let task_id = TaskId::new(id);
    service.complete_task(&task_id).map_err(server_error)
}
