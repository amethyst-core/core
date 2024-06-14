use axum::extract::State;
use crate::api::routes::AppState;
use crate::api::handlers::types::{ CreateInstanceRequest, ManageInstanceRequest };

use crate::common::types;
use crate::common::errors;

use crate::server::handlers::instance;

use axum::{ response::IntoResponse, Json };
use serde_json::json;

pub async fn list_instance(State(state): State<AppState>) -> impl IntoResponse {

    match instance::list_instances(&state.docker, &state.pool).await {
        Ok(instances) => {
            Json(
                json!({
                "status": "ok",
                "response": instances
            })
            )
        }
        Err(err) => {
            Json(
                json!({
                "status": "error",
                "message": err.to_string()
            })
            )
        }
    }
}

pub async fn get_instance(
        State(state): State<AppState>,
        Json(payload): Json<ManageInstanceRequest>
    ) -> impl IntoResponse {

    match instance::get_instance(&state.docker, &state.pool, &payload.container_id).await {
        Ok(instance) => return Json(
            json!({
                "status": "ok",
                "response": instance
            }
        )),
        Err(err) => {
            return Json(
                json!({
                "status": "error",
                "message": err.to_string()
            }))
        }
    };  

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
        Ok(_) => {
            Json(
                json!({
                "status": "ok",
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
