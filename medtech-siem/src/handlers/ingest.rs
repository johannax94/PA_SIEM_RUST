use axum::{extract::State, Json};
use chrono::Utc;
use uuid::Uuid;
use sqlx::query;

use crate::state::AppState;
use crate::models::log::IncomingLog;
use crate::services::rule_engine;

pub async fn receive_log(
    State(state): State<AppState>,
    Json(payload): Json<IncomingLog>,
) -> Json<&'static str> {

    let source_name = payload.source_name.clone();
    let event_type = payload.event_type.clone();

    let id = Uuid::new_v4();
    let now = Utc::now();

    let result = query(
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
    .bind(&event_type)
    .bind(payload.message)
    .bind(payload.metadata)
    .bind(now)
    .execute(&state.db)
    .await;

    match result {

        Ok(_) => {

            rule_engine::run_rules(
                &state.db,
                &source_name,
                &event_type
            ).await;

            Json("Log inserted")
        }

        Err(e) => {
            println!("DB error {:?}", e);
            Json("Error")
        }
    }
}