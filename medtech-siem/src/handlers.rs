use axum::{
    extract::State,
    Json,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;
use chrono::{Utc, DateTime};
use sqlx::FromRow;

use crate::state::AppState;

//
// ===== STRUCTURE REÇUE EN POST =====
//

#[derive(Deserialize)]
pub struct IncomingLog {
    pub source_type: String,
    pub source_name: String,
    pub level: String,
    pub event_type: String,
    pub message: String,
    pub metadata: Option<Value>,
}

//
// ===== STRUCTURE RETOURNÉE EN GET =====
//

#[derive(Serialize, FromRow)]
pub struct LogEntry {
    pub id: Uuid,
    pub source_type: String,
    pub source_name: String,
    pub level: String,
    pub event_type: String,
    pub message: String,
    pub metadata: Option<Value>,
    pub timestamp: DateTime<Utc>,
}

//
// ===== POST /logs =====
//

pub async fn receive_log(
    State(state): State<AppState>,
    Json(payload): Json<IncomingLog>,
) -> Json<&'static str> {
    let id = Uuid::new_v4();
    let now = Utc::now();

    let source_name = payload.source_name.clone();


    let result = sqlx::query(
        r#"
        INSERT INTO logs (
            id,
            source_type,
            source_name,
            level,
            event_type,
            message,
            metadata,
            timestamp
        )
        VALUES ($1,$2,$3,$4,$5,$6,$7,$8)
        "#
    )
    .bind(id)
    .bind(payload.source_type)
    .bind(&source_name)
    .bind(payload.level)
    .bind(payload.event_type)
    .bind(payload.message)
    .bind(payload.metadata)
    .bind(now)
    .execute(&state.db)
    .await;


    crate::rules::check_failed_login_rule(
        &state.db,
        &source_name
    ).await;

    match result {
        Ok(_) => {
            println!("Log inséré en base !");
            Json("Log inséré")
        }
        Err(e) => {
            println!("Erreur insertion: {:?}", e);
            Json("Erreur insertion")
        }
    }
}

//
// ===== GET /logs =====
//

pub async fn get_logs(
    State(state): State<AppState>,
) -> Json<Vec<LogEntry>> {

    let logs = sqlx::query_as::<_, LogEntry>(
        r#"
        SELECT *
        FROM logs
        ORDER BY timestamp DESC
        LIMIT 100
        "#
    )
    .fetch_all(&state.db)
    .await
    .unwrap_or_else(|_| vec![]);

    Json(logs)
}