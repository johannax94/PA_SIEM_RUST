use axum::{extract::State, Json};

use crate::state::AppState;
use crate::models::alert::Alert;

// ===== ANCIENNE VERSION (cassée) =====
// La table `alerts` ne possède PAS de colonne `created_at` : son schéma réel est
// (id, rule_name, severity, description, source_name, timestamp). L'ancien handler
// faisait un SELECT sur `created_at` (inexistant) et oubliait `description`, ce qui
// faisait planter l'endpoint /alerts en 500 (panic sur .unwrap()).
//
// use serde::Serialize;
//
// #[derive(Serialize, sqlx::FromRow)]
// pub struct Alert {
//     pub id: String,
//     pub rule_name: String,
//     pub severity: String,
//     pub source_name: String,
//     pub created_at: chrono::NaiveDateTime,
// }
//
// pub async fn get_alerts(
//     State(state): State<AppState>,
// ) -> Json<Vec<Alert>> {
//     let alerts = sqlx::query_as::<_, Alert>(
//         r#"
//         SELECT id, rule_name, severity, source_name, created_at
//         FROM alerts
//         ORDER BY created_at DESC
//         LIMIT 50
//         "#
//     )
//     .fetch_all(&state.db)
//     .await
//     .unwrap();
//     Json(alerts)
// }
// =====================================

pub async fn get_alerts(
    State(state): State<AppState>,
) -> Json<Vec<Alert>> {

    let alerts = sqlx::query_as::<_, Alert>(
        r#"
        SELECT id, rule_name, severity, description, source_name, timestamp
        FROM alerts
        ORDER BY timestamp DESC
        LIMIT 50
        "#
    )
    .fetch_all(&state.db)
    .await
    .unwrap_or_default();

    Json(alerts)
}
