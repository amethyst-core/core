use crate::docker::version;
use axum::{
    http::{response, StatusCode},
    response::IntoResponse,
    Json,
};
use serde_json::json;

pub async fn get_docker_version() -> impl IntoResponse {
    match version().await {
        Ok(version) => (
            StatusCode::OK,
            Json(json!({
                "version": version,
                "success": true,
            })),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "error": "Failed to get Docker version",
                "errorDescription": e.to_string(),
                "success": false,
            })),
        ),
    }
}
