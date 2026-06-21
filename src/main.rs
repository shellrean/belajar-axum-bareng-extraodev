mod error;
mod layers;
mod models;
mod modules;
mod state;

use crate::modules::auth::AuthService;
use crate::modules::task::TaskService;
use crate::state::AppState;
use axum::routing::{patch, post};
use axum::{Router, middleware};
use sqlx::PgPool;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower_governor::GovernorLayer;
use tower_governor::governor::GovernorConfigBuilder;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let storage_path = std::env::var("STORAGE_PATH").expect("STORAGE_PATH must be set");

    let db_pool = PgPool::connect(&database_url)
        .await
        .expect("Could not connect to PostgreSQL database");

    let app_state = AppState {
        storage_path,
        auth_service: AuthService::new(db_pool.clone()),
        task_service: TaskService::new(db_pool.clone()),
    };

    let governor_layer = GovernorConfigBuilder::default()
        .per_second(5)
        .burst_size(10)
        .finish()
        .expect("Could not build governor");

    let public_route = Router::new().route("/login", post(modules::auth::routes::login));

    let protected_route = Router::new()
        .route(
            "/tasks",
            post(modules::task::routes::create).get(modules::task::routes::list),
        )
        .route(
            "/tasks/{id}",
            patch(modules::task::routes::update).delete(modules::task::routes::delete),
        )
        .route("/upload", post(modules::task::routes::upload))
        .layer(middleware::from_fn(layers::auth::validate_token));

    let app = Router::new()
        .merge(public_route)
        .merge(protected_route)
        .with_state(app_state)
        .layer(GovernorLayer::new(governor_layer));

    let listener = TcpListener::bind("0.0.0.0:4001").await.unwrap();
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap()
}
