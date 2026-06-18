use tokio::sync::mpsc::Receiver;
use sqlx::PgPool;
use uuid::Uuid;
use chrono::Utc;

use crate::models::log::IncomingLog;
use crate::services::rule_engine;

pub async fn ingestion_worker(
    mut rx: Receiver<IncomingLog>,
    db: PgPool,
) {

    while let Some(log) = rx.recv().await {

        let source_name =
            log.source_name.clone();

        let event_type =
            log.event_type.clone();

        let message =
            log.message.clone();

        let severity =
            log.severity.clone();

        let raw_log =
            log.raw_log.clone();

        let _ = sqlx::query(
            r#"
            INSERT INTO logs
            (
                id,
                source_name,
                vendor,
                hostname,
                username,
                ip_address,
                event_type,
                severity,
                message,
                raw_log,
                created_at,
                search_vector
            )
            VALUES
            (
                $1,
                $2,
                $3,
                $4,
                $5,
                $6,
                $7,
                $8,
                $9,
                $10,
                $11,
                to_tsvector(
                    'english',
                    coalesce($2,'') || ' ' ||
                    coalesce($3,'') || ' ' ||
                    coalesce($4,'') || ' ' ||
                    coalesce($5,'') || ' ' ||
                    coalesce($6,'') || ' ' ||
                    $7 || ' ' ||
                    $8 || ' ' ||
                    $9
                )
            )
            "#
        )
        .bind(Uuid::new_v4())
        .bind(&source_name)
        .bind(&log.vendor)
        .bind(&log.hostname)
        .bind(&log.username)
        .bind(&log.ip_address)
        .bind(&event_type)
        .bind(&severity)
        .bind(&message)
        .bind(&raw_log)
        .bind(Utc::now().naive_utc())
        .execute(&db)
        .await;

        rule_engine::run_rules(
            &db,
            &log,
        )
        .await;
            }
}