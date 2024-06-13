use axum::extract::State;
use axum::{
    response::IntoResponse,
    Json,
};
use serde_json::json;
use tokio::task;

use crate::api::routes::AppState;

pub async fn pull(State(state): State<AppState>) -> impl IntoResponse {
    let docker = state.docker;

    task::spawn(async move {
        println!("Pulling image...");
        let _ = docker.pull_image().await;
        println!("Image pulled.");
    });

    Json(json!({
        "status": "ok",
    }))
}
