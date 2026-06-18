use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;
use serde_json::json;

pub async fn log_admin_action(
    db: &PgPool,
    event_type: &str,
    message: String,
) {

    let _ = sqlx::query(
        r#"
        INSERT INTO logs
        (
            id,
            source_name,
            event_type,
            severity,
            message,
            raw_log,
            created_at
        )
        VALUES ($1,$2,$3,$4,$5,$6,$7)
        "#
    )
    .bind(Uuid::new_v4())
    .bind("medtech-siem")
    .bind(event_type)
    .bind("low")
    .bind(message)
    .bind(json!({}))
    .bind(Utc::now().naive_utc())
    .execute(db)
    .await;
}