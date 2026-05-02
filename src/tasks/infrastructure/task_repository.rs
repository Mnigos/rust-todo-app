use crate::tasks::{
    domain::{Task, TaskId, TaskTitle},
    infrastructure::{TaskRow, TaskRowMappingError},
};
use sqlx::PgPool;

pub struct TaskRepository {
    pool: PgPool,
}

impl TaskRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn find_all(&self) -> Result<Vec<Task>, TaskRepositoryError> {
        let rows = sqlx::query_as!(
            TaskRow,
            "SELECT id, title, is_completed FROM tasks ORDER BY title ASC"
        )
        .fetch_all(&self.pool)
        .await
        .map_err(TaskRepositoryError::QueryFailed)?;

        rows.into_iter()
            .map(Task::try_from)
            .collect::<Result<Vec<_>, _>>()
            .map_err(TaskRepositoryError::MappingFailed)
    }

    pub async fn find_by_id(&self, id: &TaskId) -> Result<Option<Task>, TaskRepositoryError> {
        let row = sqlx::query_as!(
            TaskRow,
            r#"
            SELECT id, title, is_completed FROM tasks WHERE id = $1
            "#,
            id.value()
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(TaskRepositoryError::QueryFailed)?;

        row.map(Task::try_from)
            .transpose()
            .map_err(TaskRepositoryError::MappingFailed)
    }

    pub async fn insert(&self, title: &TaskTitle) -> Result<Task, TaskRepositoryError> {
        let row = sqlx::query_as!(
            TaskRow,
            r#"
            INSERT INTO tasks (title, is_completed)
            VALUES ($1, false)
            RETURNING id, title, is_completed
            "#,
            title.value()
        )
        .fetch_one(&self.pool)
        .await
        .map_err(TaskRepositoryError::QueryFailed)?;

        Task::try_from(row).map_err(TaskRepositoryError::MappingFailed)
    }

    pub async fn update(&self, task: &Task) -> Result<(), TaskRepositoryError> {
        let row = TaskRow::from(task);

        sqlx::query!(
            r#"
            UPDATE tasks
            SET
                title = $2,
                is_completed = $3
            WHERE id = $1
            "#,
            row.id,
            row.title,
            row.is_completed
        )
        .execute(&self.pool)
        .await
        .map_err(TaskRepositoryError::QueryFailed)?;

        Ok(())
    }

    pub async fn save(&self, task: &Task) -> Result<(), TaskRepositoryError> {
        let row = TaskRow::from(task);

        sqlx::query!(
            r#"
            INSERT INTO tasks (id, title, is_completed)
            VALUES ($1, $2, $3)
            ON CONFLICT (id) DO UPDATE SET
                title = EXCLUDED.title,
                is_completed = EXCLUDED.is_completed
            "#,
            row.id,
            row.title,
            row.is_completed
        )
        .execute(&self.pool)
        .await
        .map_err(TaskRepositoryError::QueryFailed)?;

        Ok(())
    }
}

#[derive(Debug)]
pub enum TaskRepositoryError {
    QueryFailed(sqlx::Error),
    MappingFailed(TaskRowMappingError),
}
