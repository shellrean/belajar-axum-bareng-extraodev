use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct Task {
    pub id: i32,
    pub title: String,
    pub completed: bool
}

#[derive(Deserialize)]
pub struct CreateTask {
    pub title: String,
}

#[derive(Deserialize)]
pub struct UpdateTask {
    pub title: Option<String>,
    pub completed: Option<bool>
}
