use crate::server::services::docker::DockerClient;

use axum::{Router, routing::{get, post, put, delete}};

#[derive(Clone)]
pub struct AppState {
    pub pool: sqlx::SqlitePool,
    pub docker: DockerClient,
}

use crate::api::handlers::{
    status::get_status,
    instance::{
        list_instance, 
        get_instance, 
        create_instance, 
        update_instance, 
        delete_instance,
        
        start_instance, 
        stop_instance,
        restart_instance,
    },
};

/// Create the router for the API.
pub fn router(pool: sqlx::SqlitePool, docker: DockerClient) -> Router {
    Router::new()
        .route("/instances/create", post(create_instance))
        .route("/instances/delete", post(delete_instance))
        .route("/instances/start", post(start_instance))
        .route("/instances/stop", post(stop_instance))
        .route("/instances/restart", post(restart_instance))
        .with_state(AppState {
            pool: pool,
            docker: docker,
        })
}
