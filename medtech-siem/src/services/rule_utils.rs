use chrono::NaiveDateTime;
use sqlx::PgPool;

pub async fn count_events(
    db: &PgPool,
    source_name: &str,
    event_type: &str,
    since: NaiveDateTime,
) -> i64 {

    let result: (i64,) = sqlx::query_as(
        r#"
        SELECT COUNT(*)
        FROM logs
        WHERE source_name = $1
        AND event_type = $2
        AND created_at >= $3
        "#
    )
    .bind(source_name)
    .bind(event_type)
    .bind(since)
    .fetch_one(db)
    .await
    .unwrap();

    result.0
}

pub async fn count_events_by_ip(
    db: &PgPool,
    ip_address: &str,
    event_type: &str,
    since: NaiveDateTime,
) -> i64 {

    let result: (i64,) = sqlx::query_as(
        r#"
        SELECT COUNT(*)
        FROM logs
        WHERE ip_address = $1
        AND event_type = $2
        AND created_at >= $3
        "#
    )
    .bind(ip_address)
    .bind(event_type)
    .bind(since)
    .fetch_one(db)
    .await
    .unwrap();

    result.0
}

pub async fn count_distinct_users_by_ip(
    db: &PgPool,
    ip_address: &str,
    event_type: &str,
    since: NaiveDateTime,
) -> i64 {

    let result: (i64,) = sqlx::query_as(
        r#"
        SELECT COUNT(DISTINCT username)
        FROM logs
        WHERE ip_address = $1
        AND event_type = $2
        AND created_at >= $3
        "#
    )
    .bind(ip_address)
    .bind(event_type)
    .bind(since)
    .fetch_one(db)
    .await
    .unwrap();

    result.0
}

pub async fn count_events_by_username(
    db: &PgPool,
    username: &str,
    event_type: &str,
    since: NaiveDateTime,
) -> i64 {

    let result: (i64,) = sqlx::query_as(
        r#"
        SELECT COUNT(*)
        FROM logs
        WHERE username = $1
        AND event_type = $2
        AND created_at >= $3
        "#
    )
    .bind(username)
    .bind(event_type)
    .bind(since)
    .fetch_one(db)
    .await
    .unwrap();

    result.0
}