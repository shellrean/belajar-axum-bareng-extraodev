use axum::extract::{Multipart, Path, State};
use axum::http::StatusCode;
use axum::{Extension, Json};
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use uuid::Uuid;
use crate::{AppState, models::task::{CreateTask, Task, UpdateTask}};
use crate::error::AppError;
use crate::layers::auth::USER_CONTEXT;
use crate::models::auth::UserContext;

pub async fn create(
    State(app_state): State<AppState>,
    Json(payload): Json<CreateTask>,
) -> Result<Json<Task>, AppError>{
    let user_ctx = USER_CONTEXT.with(|u| u.clone());

    let data_result = sqlx::query_as!(
        Task,
        "INSERT INTO tasks (title, created_by) VALUES ($1, $2) RETURNING id, title, completed",
        payload.title,
        user_ctx.username
    ).fetch_one(&app_state.db).await?;
    Ok(Json(data_result))
}

pub async fn list(
    State(app_state): State<AppState>
) -> Result<Json<Vec<Task>>, AppError>{
    let data_result = sqlx::query_as!(
        Task,
        "SELECT id, title, completed FROM tasks ORDER BY id"
    ).fetch_all(&app_state.db).await?;
    Ok(Json(data_result))
}

pub async fn update(
    State(app_state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateTask>
) -> Result<StatusCode, AppError>{
    let updated = sqlx::query!(
        "UPDATE tasks SET title = COALESCE($1, title), completed = COALESCE($2, completed) WHERE id = $3",
        payload.title,
        payload.completed,
        id,
    ).execute(&app_state.db).await?;

    if updated.rows_affected() > 0 {
        Ok(StatusCode::OK)
    } else {
        Err(AppError::NotFound(format!("task with id {}", id)))
    }
}

pub async fn delete(
    State(app_state): State<AppState>,
    Path(id): Path<i32>
) -> Result<StatusCode, AppError> {
    let deleted = sqlx::query!("DELETE FROM tasks WHERE id = $1", id)
        .execute(&app_state.db)
        .await?;

    if deleted.rows_affected() > 0 {
        Ok(StatusCode::OK)
    } else {
        Err(AppError::NotFound(format!("task with id {}", id)))
    }
}

pub async fn upload(
    State(app_state): State<AppState>,
    mut multipart: Multipart,
) -> Result<StatusCode, AppError>{
    while let Some(mut field) = multipart.next_field().await? {
        let name = field.name().unwrap_or_default();
        if name != "image" {
            continue;
        }

        let content_type = field.content_type().unwrap_or_default();
        if !content_type.starts_with("image/") {
            continue;
        }

        let ext_opt = std::path::Path::new(field.file_name().unwrap_or_default())
            .extension()
            .and_then(std::ffi::OsStr::to_str);

        let ext = match ext_opt {
            Some(ext) => ext,
            None => continue,
        };

        let file_name = format!("{}.{}", Uuid::new_v4().to_string(), ext);

        let path = std::path::Path::new(&app_state.storage_path)
            .join(file_name);

        let mut file = File::create(path).await?;

        while let Some(chunk) = field.chunk().await? {
            file.write_all(&chunk).await?;
        }

        return Ok(StatusCode::CREATED)
    }
    Err(AppError::NotFound("file not found".to_string()))
}









