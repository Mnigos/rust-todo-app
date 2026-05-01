use crate::tasks::domain::Task;
#[cfg(feature = "ssr")]
use crate::tasks::{application::TaskService, domain::TaskId, infrastructure::TaskRepository};
use leptos::prelude::*;
#[cfg(feature = "ssr")]
use sqlx::PgPool;
use std::fmt::Debug;
use uuid::Uuid;

#[cfg(feature = "ssr")]
fn server_error(err: impl Debug) -> ServerFnError {
    ServerFnError::new(format!("{err:?}"))
}

#[cfg(feature = "ssr")]
fn create_task_service() -> Result<TaskService, ServerFnError> {
    let pool = use_context::<PgPool>()
        .ok_or_else(|| ServerFnError::new("database pool missing from context"))?;
    let repository = TaskRepository::new(pool);

    Ok(TaskService::new(repository))
}

#[server]
pub async fn list_tasks() -> Result<Vec<Task>, ServerFnError> {
    let service = create_task_service()?;

    service.list_all().await.map_err(server_error)
}

#[server]
pub async fn add_task(title: String) -> Result<Task, ServerFnError> {
    let service = create_task_service()?;

    service.add_task(title).await.map_err(server_error)
}

#[server]
pub async fn complete_task(id: Uuid) -> Result<(), ServerFnError> {
    let service = create_task_service()?;

    let task_id = TaskId::new(id);
    service.complete_task(&task_id).await.map_err(server_error)
}

#[server]
pub async fn reopen_task(id: Uuid) -> Result<(), ServerFnError> {
    let service = create_task_service()?;

    let task_id = TaskId::new(id);
    service.reopen_task(&task_id).await.map_err(server_error)
}
