use sqlx::{PgPool, prelude::FromRow};

use crate::error::AppError;

#[derive(FromRow)]
pub struct Task {
    pub id: i32,
    pub title: String,
    pub completed: bool,
}

#[derive(Clone)]
pub struct TaskRepository {
    db: PgPool,
}

impl TaskRepository {
    pub fn new(db: PgPool) -> Self {
        Self { db }
    }

    pub async fn create(&self, title: &str, username: &str) -> Result<Task, AppError> {
        let data_result = sqlx::query_as::<_, Task>(
            "INSERT INTO tasks (title, created_by) VALUES ($1, $2) RETURNING id, title, completed",
        )
        .bind(title)
        .bind(username)
        .fetch_one(&self.db)
        .await?;

        Ok(data_result)
    }

    pub async fn list(&self) -> Result<Vec<Task>, AppError> {
        let data_result =
            sqlx::query_as::<_, Task>("SELECT id, title, completed FROM tasks ORDER BY id")
                .fetch_all(&self.db)
                .await?;
        Ok(data_result)
    }

    pub async fn update(
        &self,
        id: i32,
        title: Option<String>,
        completed: Option<bool>,
    ) -> Result<u64, AppError> {
        let updated = sqlx::query!(
            "UPDATE tasks SET title = COALESCE($1, title), completed = COALESCE($2, completed) WHERE id = $3",
            title,
            completed,
            id,
        ).execute(&self.db).await?;

        Ok(updated.rows_affected())
    }

    pub async fn delete(&self, id: i32) -> Result<u64, AppError> {
        let deleted = sqlx::query("DELETE FROM tasks WHERE id = $1")
            .bind(id)
            .execute(&self.db)
            .await?;

        Ok(deleted.rows_affected())
    }
}
