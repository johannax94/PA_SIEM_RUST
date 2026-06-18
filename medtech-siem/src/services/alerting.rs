use chrono::NaiveDateTime;
use sqlx::PgPool;
use uuid::Uuid;
use chrono::Utc;

pub async fn create_alert(
    db: &PgPool,
    rule: &str,
    severity: &str,
    source: &str,
    message: &str,
) {

    let _ = sqlx::query(
        r#"
        INSERT INTO alerts
        (
            id,
            rule_name,
            severity,
            source_name,
            message,
            created_at
        )
        VALUES
        (
            $1,$2,$3,$4,$5,$6
        )
        "#
    )
    .bind(Uuid::new_v4())
    .bind(rule)
    .bind(severity)
    .bind(source)
    .bind(message)
    .bind(Utc::now().naive_utc())
    .execute(db)
    .await;
}

pub async fn alert_exists(
    db: &PgPool,
    rule_name: &str,
    source_name: &str,
    since: NaiveDateTime,
) -> bool {

    let count: (i64,) = sqlx::query_as(
        r#"
        SELECT COUNT(*)
        FROM alerts
        WHERE
            rule_name = $1
        AND source_name = $2
        AND created_at >= $3
        "#
    )
    .bind(rule_name)
    .bind(source_name)
    .bind(since)
    .fetch_one(db)
    .await
    .unwrap();

    count.0 > 0
}