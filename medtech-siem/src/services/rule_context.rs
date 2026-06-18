use chrono::{Duration, NaiveDateTime, Utc};
use sqlx::PgPool;

use crate::models::log::IncomingLog;
use crate::services::{
    alerting,
    rule_utils,
};

pub struct RuleContext<'a> {
    pub db: &'a PgPool,
    pub log: &'a IncomingLog,
}

impl<'a> RuleContext<'a> {

    pub fn new(
        db: &'a PgPool,
        log: &'a IncomingLog,
    ) -> Self {
        Self { db, log }
    }

    pub fn since_minutes(
        &self,
        minutes: i64,
    ) -> NaiveDateTime {

        Utc::now().naive_utc()
            - Duration::minutes(minutes)
    }

    pub fn is_event(
        &self,
        event: &str,
    ) -> bool {

        self.log.event_type == event
    }

    pub async fn count_source(
        &self,
        event: &str,
        minutes: i64,
    ) -> i64 {

        rule_utils::count_events(
            self.db,
            &self.log.source_name,
            event,
            self.since_minutes(minutes),
        )
        .await
    }

    pub async fn count_ip(
        &self,
        event: &str,
        minutes: i64,
    ) -> i64 {

        match &self.log.ip_address {

            Some(ip) => {
                rule_utils::count_events_by_ip(
                    self.db,
                    ip,
                    event,
                    self.since_minutes(minutes),
                )
                .await
            }

            None => 0,
        }
    }

    pub async fn count_distinct_users(
        &self,
        event: &str,
        minutes: i64,
    ) -> i64 {

        match &self.log.ip_address {

            Some(ip) => {
                rule_utils::count_distinct_users_by_ip(
                    self.db,
                    ip,
                    event,
                    self.since_minutes(minutes),
                )
                .await
            }

            None => 0,
        }
    }

    pub async fn count_user(
        &self,
        event: &str,
        minutes: i64,
    ) -> i64 {

        match &self.log.username {

            Some(username) => {
                rule_utils::count_events_by_username(
                    self.db,
                    username,
                    event,
                    self.since_minutes(minutes),
                )
                .await
            }

            None => 0,
        }
    }

    pub async fn alert_exists(
        &self,
        rule: &str,
        minutes: i64,
    ) -> bool {

        alerting::alert_exists(
            self.db,
            rule,
            &self.log.source_name,
            self.since_minutes(minutes),
        )
        .await
    }

    pub async fn alert(
        &self,
        rule: &str,
        severity: &str,
        message: &str,
    ) {

        alerting::create_alert(
            self.db,
            rule,
            severity,
            &self.log.source_name,
            message,
        )
        .await;
    }
}