use axum::{extract::State, Json};
use serde::Serialize;

use crate::state::AppState;

#[derive(Serialize)]
pub struct DashboardStats {
    total_logs: i64,
    total_alerts: i64,
}

pub async fn get_dashboard(
    State(state): State<AppState>,
) -> Json<DashboardStats> {

    let total_logs: (i64,) =
        sqlx::query_as(
            "SELECT COUNT(*) FROM logs"
        )
        .fetch_one(&state.db)
        .await
        .unwrap();

    let total_alerts: (i64,) =
        sqlx::query_as(
            "SELECT COUNT(*) FROM alerts"
        )
        .fetch_one(&state.db)
        .await
        .unwrap();

    Json(
        DashboardStats {
            total_logs: total_logs.0,
            total_alerts: total_alerts.0,
        }
    )
}