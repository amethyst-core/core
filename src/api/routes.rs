use axum::{Router, routing::{get, post, put, delete}};
use crate::api::handlers::{
    status::get_status,
    instance::{list_instance, get_instance, create_instance, update_instance, delete_instance},
};

/// Create the router for the API.
pub fn router() -> Router {
    Router::new()
        // Server Status Routes
        .route("/status", get(get_status))
        // Instances Routes
        .route("/instance/all", get(list_instance))
        .route("/instance/create", post(create_instance))
        .route("/instance/:param", get(get_instance))
        .route("/instance/:param", put(update_instance))
        .route("/instance/:param", delete(delete_instance))
}
