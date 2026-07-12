use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use uuid::Uuid;

use crate::services::notifier::RULE_CATALOG;
use crate::state::AppState;

#[derive(Serialize, sqlx::FromRow)]
pub struct AlertConfig {
    pub id: Uuid,
    pub rule_name: String,
    pub threshold: i32,
    pub window_minutes: i32,
    pub comment: String,
    pub enabled: bool,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Serialize)]
pub struct AlertConfigsResponse {
    pub configs: Vec<AlertConfig>,
    /// Catalogue des règles notifiables, pour le menu déroulant du formulaire.
    pub rules: Vec<&'static str>,
}

pub async fn list_alert_configs(
    State(state): State<AppState>,
) -> Json<AlertConfigsResponse> {

    let configs = sqlx::query_as::<_, AlertConfig>(
        r#"
        SELECT id, rule_name, threshold, window_minutes, comment, enabled, created_at
        FROM alert_configs
        ORDER BY created_at DESC
        "#,
    )
    .fetch_all(&state.db)
    .await
    .unwrap_or_default();

    Json(AlertConfigsResponse {
        configs,
        rules: RULE_CATALOG.to_vec(),
    })
}

#[derive(Deserialize)]
pub struct CreateAlertConfig {
    pub rule_name: String,
    pub threshold: i32,
    pub window_minutes: i32,
    pub comment: String,
}

pub async fn create_alert_config(
    State(state): State<AppState>,
    Json(payload): Json<CreateAlertConfig>,
) -> (StatusCode, Json<Value>) {

    // Validation : règle connue, seuil et fenêtre positifs et raisonnables.
    if !RULE_CATALOG.contains(&payload.rule_name.as_str()) {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "règle inconnue" })),
        );
    }
    if payload.threshold < 1 || payload.threshold > 1000 {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "seuil invalide (1 à 1000)" })),
        );
    }
    if payload.window_minutes < 1 || payload.window_minutes > 10080 {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "fenêtre invalide (1 à 10080 min)" })),
        );
    }
    if payload.comment.trim().is_empty() || payload.comment.len() > 2000 {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({ "error": "commentaire vide ou trop long" })),
        );
    }

    let result = sqlx::query(
        r#"
        INSERT INTO alert_configs
        (id, rule_name, threshold, window_minutes, comment, enabled, created_at)
        VALUES ($1, $2, $3, $4, $5, TRUE, $6)
        "#,
    )
    .bind(Uuid::new_v4())
    .bind(&payload.rule_name)
    .bind(payload.threshold)
    .bind(payload.window_minutes)
    .bind(payload.comment.trim())
    .bind(chrono::Utc::now().naive_utc())
    .execute(&state.db)
    .await;

    match result {
        Ok(_) => (StatusCode::OK, Json(json!({ "status": "created" }))),
        Err(e) => {
            tracing::error!("création config alerte échouée : {e}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": "création impossible" })),
            )
        }
    }
}

pub async fn delete_alert_config(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> (StatusCode, Json<Value>) {

    let _ = sqlx::query("DELETE FROM alert_configs WHERE id = $1")
        .bind(id)
        .execute(&state.db)
        .await;

    (StatusCode::OK, Json(json!({ "status": "deleted" })))
}
