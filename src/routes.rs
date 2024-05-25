use crate::handlers::get_docker_version;
use axum::{routing::get, Router};

pub fn create_router() -> Router {
    Router::new().route("/docker/version", get(get_docker_version))
}
