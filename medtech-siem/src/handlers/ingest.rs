use axum::{extract::State, Json};

use crate::state::AppState;
use crate::models::log::IncomingLog;

pub async fn receive_log(
    State(state): State<AppState>,
    Json(payload): Json<IncomingLog>,
) -> Json<&'static str> {

    let _ = state.sender.send(payload).await;

    Json("queued")
}