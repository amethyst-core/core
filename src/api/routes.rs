use axum::{Router, routing::get};
use crate::api::handlers::status;

/// Create the router for the API.
pub fn router() -> Router {
    Router::new()
        .route("/status", get(status::status))
}
