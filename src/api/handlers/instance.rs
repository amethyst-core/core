use axum::{
    response::IntoResponse,
    Json,
};
use serde_json::json;

pub async fn list_instance() -> impl IntoResponse {
    Json(json!({
        "status": "ok",
    }))
}

pub async fn get_instance(instance_id: axum::extract::Path<String>) -> impl IntoResponse {
    Json(json!({
        "status": "ok",
        "instance_id": instance_id.0,
    }))
}

pub async fn create_instance() -> impl IntoResponse {
    Json(json!({
        "status": "ok",
    }))
}

pub async fn update_instance() -> impl IntoResponse {
    Json(json!({
        "status": "ok",
    }))
}

pub async fn delete_instance() -> impl IntoResponse {
    Json(json!({
        "status": "ok",
    }))
}
