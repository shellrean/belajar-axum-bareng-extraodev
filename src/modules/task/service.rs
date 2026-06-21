use sqlx::PgPool;

use crate::{
    error::AppError,
    layers::auth::USER_CONTEXT,
    modules::task::types::{CreateTask, Task, UpdateTask},
};

use super::repositories::TaskRepository;

#[derive(Clone)]
pub struct TaskService {
    repo: TaskRepository,
}

impl TaskService {
    pub fn new(db: PgPool) -> Self {
        Self {
            repo: TaskRepository::new(db),
        }
    }

    pub async fn create(&self, payload: CreateTask) -> Result<Task, AppError> {
        let user_ctx = USER_CONTEXT.with(|u| u.clone());
        let data_result = self.repo.create(&payload.title, &user_ctx.username).await?;
        Ok(Task {
            id: data_result.id,
            title: data_result.title,
            completed: data_result.completed,
        })
    }

    pub async fn list(&self) -> Result<Vec<Task>, AppError> {
        let data_result = self
            .repo
            .list()
            .await?
            .into_iter()
            .map(|row| Task {
                id: row.id,
                title: row.title,
                completed: row.completed,
            })
            .collect();

        Ok(data_result)
    }

    pub async fn update(&self, id: i32, payload: UpdateTask) -> Result<(), AppError> {
        let updated = self
            .repo
            .update(id, payload.title, payload.completed)
            .await?;
        if updated < 1 {
            return Err(AppError::NotFound(format!("task with id {}", id)));
        }
        Ok(())
    }

    pub async fn delete(&self, id: i32) -> Result<(), AppError> {
        let deleted = self.repo.delete(id).await?;
        if deleted < 1 {
            return Err(AppError::NotFound(format!("task with id {}", id)));
        }
        Ok(())
    }
}
