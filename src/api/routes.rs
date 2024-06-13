use crate::server::services::docker::DockerClient;

use axum::{Router, routing::{get, post, put, delete}};

#[derive(Clone)]
pub struct AppState {
    pub pool: sqlx::SqlitePool,
    pub docker: DockerClient,
}

use crate::api::handlers::{
    status,
    instance::{
        get_instance, 
        list_instance, 

        create_instance, 
        delete_instance,
        
        start_instance, 
        stop_instance,
        restart_instance,
    },
    players,
    images
};

/// Create the router for the API.
pub fn router(pool: sqlx::SqlitePool, docker: DockerClient) -> Router {
    Router::new()
        .route("/images/pull", get(images::pull))

        .route("/instances", get(list_instance))
        .route("/instances/get", post(get_instance))

        .route("/instances/create", post(create_instance))
        .route("/instances/delete", post(delete_instance))
        
        .route("/instances/start", post(start_instance))
        .route("/instances/stop", post(stop_instance))
        .route("/instances/restart", post(restart_instance))
        
        .route("/instances/players", post(players::list_players)) // Not implemented yet
        .with_state(AppState {
            pool: pool,
            docker: docker,
        })
}
