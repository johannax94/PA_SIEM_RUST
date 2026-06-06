use axum::{extract::State, Json};
use sqlx::query_as;

use crate::state::AppState;
use crate::models::log::LogEntry;

pub async fn get_logs(
    State(state): State<AppState>,
) -> Json<Vec<LogEntry>> {

    let logs = query_as::<_, LogEntry>(
        r#"
        SELECT *
        FROM logs
        ORDER BY created_at DESC
        LIMIT 100
        "#
    )
    .fetch_all(&state.db)
    .await
    .expect("Erreur SQL logs");

    Json(logs)
}