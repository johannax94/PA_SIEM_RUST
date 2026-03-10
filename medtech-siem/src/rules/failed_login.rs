use sqlx::PgPool;
use uuid::Uuid;
use chrono::{Utc, Duration};

pub async fn check_failed_login_rule(
    db: &PgPool,
    source_name: &str,
) {

    let one_minute_ago = Utc::now() - Duration::minutes(1);

    let count: (i64,) = sqlx::query_as(
        r#"
        SELECT COUNT(*) as count
        FROM logs
        WHERE source_name = $1
        AND event_type = 'login_failed'
        AND timestamp >= $2
        "#
    )
    .bind(source_name)
    .bind(one_minute_ago)
    .fetch_one(db)
    .await
    .unwrap();

    if count.0 >= 5 {

        let existing_alert: (i64,) = sqlx::query_as(
            r#"
            SELECT COUNT(*) as count
            FROM alerts
            WHERE rule_name = 'multiple_failed_logins'
            AND source_name = $1
            AND timestamp >= $2
            "#
        )
        .bind(source_name)
        .bind(one_minute_ago)
        .fetch_one(db)
        .await
        .unwrap();

        if existing_alert.0 == 0 {

            println!("🚨 Brute force détectée sur {}", source_name);

            let alert_id = Uuid::new_v4();

            let _ = sqlx::query(
                r#"
                INSERT INTO alerts (
                    id,
                    rule_name,
                    severity,
                    description,
                    source_name,
                    timestamp
                )
                VALUES ($1,$2,$3,$4,$5,$6)
                "#
            )
            .bind(alert_id)
            .bind("multiple_failed_logins")
            .bind("high")
            .bind("5 failed logins in less than 1 minute")
            .bind(source_name)
            .bind(Utc::now())
            .execute(db)
            .await;
        }
    }
}