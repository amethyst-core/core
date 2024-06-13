#![allow(dead_code)]

use axum::extract::State;
use axum::{
    response::IntoResponse,
    Json,
};
use serde_json::json;

use crate::api::routes::AppState;
use crate::api::handlers::types::ManageInstanceRequest;

use crate::server::handlers::types;
use crate::server::handlers::players;

pub async fn list_players(State(state): State<AppState>, Json(payload): Json<ManageInstanceRequest>) -> impl IntoResponse {
    let pool = &state.pool;
    let docker = state.docker;
    let container_id = &payload.container_id;

    match players::list_players(&docker, &container_id, &pool).await {
        Ok(players_response) => {
            Json(json!({
                "status": "ok",
                "player_max": players_response.player_max.unwrap_or_default(),
                "player_active": players_response.player_active.unwrap_or_default(),
                "players": players_response.player_list.unwrap_or_default()
            }))
        },
        Err(err) => {
            // Return an error response
            Json(json!({
                "status": "error",
                "message": err.to_string()
            }))
        }
    }
}