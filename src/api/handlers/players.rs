use axum::extract::State;
use axum::{ response::IntoResponse, Json };
use serde_json::json;

use crate::api::routes::AppState;
use crate::api::handlers::types::ManageInstanceRequest;

use crate::server::handlers::players;

pub async fn list_players(
    State(state): State<AppState>,
    Json(payload): Json<ManageInstanceRequest>
) -> impl IntoResponse {

    match players::list_players(&state.docker, &payload.container_id).await {
        Ok(players_response) => {
            Json(
                json!({
                "status": "ok",
                "response": players_response
            })
            )
        }
        Err(err) => {
            // Return an error response
            Json(
                json!({
                "status": "error",
                "message": err.to_string()
            })
            )
        }
    }
}
