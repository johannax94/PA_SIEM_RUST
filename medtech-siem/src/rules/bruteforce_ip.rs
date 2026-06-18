use chrono::{Duration, Utc};
use sqlx::PgPool;

use crate::models::log::IncomingLog;
use crate::services::{
    alerting,
    rule_utils,
};

pub async fn check_bruteforce_ip(
    db: &PgPool,
    log: &IncomingLog,
) {

    if log.event_type != "login_failed" {
        return;
    }

    let ip = match &log.ip_address {
        Some(ip) => ip,
        None => return,
    };

    let since =
        Utc::now().naive_utc()
        - Duration::minutes(2);

    let count =
        rule_utils::count_events_by_ip(
            db,
            ip,
            "login_failed",
            since,
        )
        .await;

    if count < 10 {
        return;
    }

    if alerting::alert_exists(
        db,
        "bruteforce_ip",
        &log.source_name,
        since,
    )
    .await
    {
        return;
    }

    alerting::create_alert(
        db,
        "bruteforce_ip",
        "critical",
        &log.source_name,
        &format!(
            "{} failed logins from IP {}",
            count,
            ip
        ),
    )
    .await;
}