use tokio::sync::mpsc::Receiver;
use crate::models::log::IncomingLog;
use sqlx::PgPool;
use uuid::Uuid;
use chrono::Utc;

use crate::services::rule_engine;

pub async fn ingestion_worker(
    mut rx: Receiver<IncomingLog>,
    db: PgPool,
) {

while let Some(log) = rx.recv().await {

    let source_name = log.source_name.clone();
    let event_type = log.event_type.clone();

   let _ = sqlx::query(
    r#"
    INSERT INTO logs (
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
.bind(&source_name)
.bind(&event_type)
.bind(log.severity)
.bind(log.message)
.bind(log.raw_log)
.bind(Utc::now().naive_utc())
.execute(&db)
.await;
}
}