use axum::{extract::State, Json};
use sqlx::query_as;
use axum::extract::Query;
use crate::state::AppState;
use crate::models::log::LogEntry;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct LogFilters {
    pub q: Option<String>,
    pub severity: Option<String>,
}

pub async fn get_logs(
    State(state): State<AppState>,
    Query(filters): Query<LogFilters>,
) -> Json<Vec<LogEntry>> {

    let logs = query_as::<_, LogEntry>(
        r#"
        SELECT *
        FROM logs
        WHERE
        ($1 IS NULL OR (search_vector @@ plainto_tsquery('english', $1)))
        AND
        ($2 IS NULL OR severity = $2)
        ORDER BY created_at DESC
        LIMIT 100
        "#
    )
    .bind(filters.q)
    .bind(filters.severity)
    .fetch_all(&state.db)
    .await
    .expect("Erreur SQL logs");

    Json(logs)
}