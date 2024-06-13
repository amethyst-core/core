use axum::extract::State;
use crate::api::routes::AppState;
use crate::api::handlers::types::{ CreateInstanceRequest, ManageInstanceRequest };

use crate::common::types;
use crate::common::errors;

use axum::{ response::IntoResponse, Json };
use serde_json::json;

pub async fn list_instance(State(state): State<AppState>) -> impl IntoResponse {
    let docker = state.docker;
    let containers = docker.list_containers().await.unwrap();

    Json(json!({
        "status": "ok",
        "containers": containers
    }))
}

pub async fn get_instance(instance_id: axum::extract::Path<String>) -> impl IntoResponse {
    // TODO
    Json(json!({
        "status": "ok",
        "instance_id": instance_id.0,
    }))
}

pub async fn create_instance(
    State(state): State<AppState>,
    Json(payload): Json<CreateInstanceRequest>
) -> impl IntoResponse {
    let pool = state.pool;
    let docker = state.docker;

    let server_type;

     match types::ServerTypes::from_str(&payload.server_type.to_lowercase()) {
        Some(server_type_enum) => {
            server_type = server_type_enum;
        },
        None => {
            return Json (
                json!({
                "status": "error",
                "error": errors::Errors::InvalidServerType(payload.server_type).to_string(),
            })
            )
        }
    }

    match
        docker.create_container(
            server_type,
            &payload.server_version,
            &payload.port,
            &pool
        ).await
    {
        Ok(container) => {
            Json(
                json!({
                "status": "ok",
                "instance": container,
            })
            )
        }
        Err(e) => {
            Json(
                json!({
                "status": "error",
                "error": e.to_string(),
            })
            )
        }
    }
}

pub async fn delete_instance(
    State(state): State<AppState>,
    Json(payload): Json<ManageInstanceRequest>
) -> impl IntoResponse {
    let pool = state.pool;
    let docker = state.docker;

    match docker.delete_container(&payload.container_id, &pool).await {
        Ok(_) => { Json(json!({
                "status": "ok",
            })) }
        Err(e) => {
            Json(
                json!({
                "status": "error",
                "error": e.to_string(),
            })
            )
        }
    }
}

pub async fn start_instance(
    State(state): State<AppState>,
    Json(payload): Json<ManageInstanceRequest>
) -> impl IntoResponse {
    let docker = state.docker;

    match docker.start_container(&payload.container_id).await {
        Ok(_) => { Json(json!({
                "status": "ok",
            })) }
        Err(e) => {
            Json(
                json!({
                "status": "error",
                "error": e.to_string(),
            })
            )
        }
    }
}

pub async fn stop_instance(
    State(state): State<AppState>,
    Json(payload): Json<ManageInstanceRequest>
) -> impl IntoResponse {
    let docker = state.docker;

    match docker.stop_container(&payload.container_id).await {
        Ok(_) => { Json(json!({
                "status": "ok",
            })) }
        Err(e) => {
            Json(
                json!({
                "status": "error",
                "error": e.to_string(),
            })
            )
        }
    }
}

pub async fn restart_instance(
    State(state): State<AppState>,
    Json(payload): Json<ManageInstanceRequest>
) -> impl IntoResponse {
    let docker = state.docker;

    match docker.restart_container(&payload.container_id).await {
        Ok(_) => { Json(json!({
                "status": "ok",
            })) }
        Err(e) => {
            Json(
                json!({
                "status": "error",
                "error": e.to_string(),
            })
            )
        }
    }
}
