use axum::{extract::State, Json};
use sqlx::query_as;

use crate::state::AppState;
use crate::models::alert::Alert;

pub async fn get_alerts(
    State(state): State<AppState>,
) -> Json<Vec<Alert>> {

    let alerts = query_as::<_, Alert>(
        r#"
        SELECT *
        FROM alerts
        ORDER BY timestamp DESC
        LIMIT 100
        "#
    )
    .fetch_all(&state.db)
    .await
    .unwrap_or_else(|_| vec![]);

    Json(alerts)
}