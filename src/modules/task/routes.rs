use tokio::fs::File;
use tokio::io::AsyncWriteExt;

use crate::{
    error::AppError,
    modules::task::types::{CreateTask, Task, UpdateTask},
    state::AppState,
};
use axum::{
    Json,
    extract::{Multipart, Path, State},
    http::StatusCode,
};
use uuid::Uuid;

pub async fn create(
    State(app_state): State<AppState>,
    Json(payload): Json<CreateTask>,
) -> Result<Json<Task>, AppError> {
    let data_result = app_state.task_service.create(payload).await?;

    Ok(Json(data_result))
}

pub async fn list(State(app_state): State<AppState>) -> Result<Json<Vec<Task>>, AppError> {
    let data_result = app_state.task_service.list().await?;
    Ok(Json(data_result))
}

pub async fn update(
    State(app_state): State<AppState>,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateTask>,
) -> Result<StatusCode, AppError> {
    app_state.task_service.update(id, payload).await?;

    Ok(StatusCode::OK)
}

pub async fn delete(
    State(app_state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<StatusCode, AppError> {
    app_state.task_service.delete(id).await?;
    Ok(StatusCode::OK)
}

pub async fn upload(
    State(app_state): State<AppState>,
    mut multipart: Multipart,
) -> Result<StatusCode, AppError> {
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

        let path = std::path::Path::new(&app_state.storage_path).join(file_name);

        let mut file = File::create(path).await?;

        while let Some(chunk) = field.chunk().await? {
            file.write_all(&chunk).await?;
        }

        return Ok(StatusCode::CREATED);
    }
    Err(AppError::NotFound("file not found".to_string()))
}
