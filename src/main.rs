mod error;
mod routes;
mod state;
mod layers;
mod models;

use std::net::SocketAddr;
use crate::state::AppState;
use axum::routing::{patch, post};
use axum::{middleware, Router, ServiceExt};
use serde::{Serialize};
use sqlx::PgPool;
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_governor::governor::GovernorConfigBuilder;
use tower_governor::GovernorLayer;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let storage_path = std::env::var("STORAGE_PATH")
        .expect("STORAGE_PATH must be set");
    
    let db_pool = PgPool::connect(&database_url)
        .await
        .expect("Could not connect to PostgreSQL database");

    let app_state = AppState {
        db: db_pool,
        storage_path
    };

    let governor_layer = GovernorConfigBuilder::default()
        .per_second(5)
        .burst_size(10)
        .finish()
        .expect("Could not build governor");

    let public_route = Router::new()
        .route("/login", post(routes::auth::login));

    let protected_route = Router::new()
        .route("/tasks", post(routes::task::create).get(routes::task::list))
        .route("/tasks/{id}", patch(routes::task::update).delete(routes::task::delete))
        .route("/upload", post(routes::task::upload))
        .layer(middleware::from_fn(layers::auth::validate_token));

    let app = Router::new()
        .merge(public_route)
        .merge(protected_route)
        .with_state(app_state)
        .layer(GovernorLayer::new(governor_layer));
    
    let listener = TcpListener::bind("0.0.0.0:4000")
        .await
        .unwrap();
    axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>()).await.unwrap()
}
