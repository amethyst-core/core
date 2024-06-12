#![allow(dead_code)]

use axum::{
    response::IntoResponse,
    Json,
};
use serde_json::json;

/// This function is used to provide the health of the API.
pub async fn get_status() -> impl IntoResponse {
    Json(json!({
        "status": "ok",
    }))
}
