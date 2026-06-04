use axum::{extract::State, Json};
use crate::state::AppState;
use serde::Serialize;

#[derive(Serialize, sqlx::FromRow)]
pub struct Alert {
    pub id: String,
    pub rule_name: String,
    pub severity: String,
    pub source_name: String,
    pub created_at: chrono::NaiveDateTime,
}

pub async fn get_alerts(
    State(state): State<AppState>,
) -> Json<Vec<Alert>> {

    let alerts = sqlx::query_as::<_, Alert>(
        r#"
        SELECT id, rule_name, severity, source_name, created_at
        FROM alerts
        ORDER BY created_at DESC
        LIMIT 50
        "#
    )
    .fetch_all(&state.db)
    .await
    .unwrap();

    Json(alerts)
}