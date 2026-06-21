use crate::modules::{auth::AuthService, task::TaskService};

#[derive(Clone)]
pub struct AppState {
    pub storage_path: String,
    pub auth_service: AuthService,
    pub task_service: TaskService,
}
