use crate::server::services::docker::DockerClient;

use axum::{Router, routing::{get, post, put, delete}};

#[derive(Clone)]
pub struct AppState {
    pub pool: sqlx::SqlitePool,
    pub docker: DockerClient,
}


use crate::api::handlers::{
    status::get_status,

    instance::{list_instance, get_instance, create_instance, update_instance, delete_instance},
};

/// Create the router for the API.
pub fn router(pool: sqlx::SqlitePool, docker: DockerClient) -> Router {
    Router::new()
        .route("/instance/create", post(create_instance))
        .with_state(AppState {
            pool: pool,
            docker: docker,
        })
}
