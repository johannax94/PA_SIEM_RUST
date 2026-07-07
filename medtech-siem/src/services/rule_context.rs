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

    // ----- Helpers réutilisables (signature / contenu) -----

    /// Vrai si l'event courant correspond à l'un des types fournis.
    pub fn is_any_event(&self, events: &[&str]) -> bool {
        events.iter().any(|e| self.log.event_type == *e)
    }

    /// Recherche insensible à la casse dans le message du log.
    pub fn message_contains(&self, needle: &str) -> bool {
        self.log
            .message
            .to_lowercase()
            .contains(&needle.to_lowercase())
    }

    /// Recherche insensible à la casse dans le hostname (ex. "dc" pour un DC).
    pub fn hostname_contains(&self, needle: &str) -> bool {
        self.log
            .hostname
            .as_deref()
            .map(|h| h.to_lowercase().contains(&needle.to_lowercase()))
            .unwrap_or(false)
    }

    /// Lit un champ texte du raw_log (JSON), ex. "country", "dest_ip".
    pub fn raw_str(&self, key: &str) -> Option<String> {
        self.log
            .raw_log
            .get(key)
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
    }

    /// Lit un champ entier du raw_log (JSON), ex. "bytes_out".
    pub fn raw_i64(&self, key: &str) -> Option<i64> {
        self.log.raw_log.get(key).and_then(|v| v.as_i64())
    }

    /// Crée l'alerte uniquement si elle n'existe pas déjà sur la fenêtre donnée
    /// (factorise le couple alert_exists + alert présent dans chaque règle).
    pub async fn alert_once(
        &self,
        rule: &str,
        severity: &str,
        message: &str,
        minutes: i64,
    ) {
        if self.alert_exists(rule, minutes).await {
            return;
        }
        self.alert(rule, severity, message).await;
    }

    /// Nombre de pays distincts vus pour cet utilisateur sur la fenêtre
    /// (basé sur raw_log->>'country') — utilisé par l'Impossible Travel.
    pub async fn count_distinct_countries_user(&self, minutes: i64) -> i64 {
        match &self.log.username {
            Some(username) => {
                rule_utils::count_distinct_countries_by_username(
                    self.db,
                    username,
                    self.since_minutes(minutes),
                )
                .await
            }
            None => 0,
        }
    }

    /// Nombre d'échecs de logon RDP (Windows 4625, logon_type 10) depuis l'IP
    /// source du log courant, sur la fenêtre donnée.
    pub async fn count_rdp_failures_ip(&self, minutes: i64) -> i64 {
        match &self.log.ip_address {
            Some(ip) => {
                rule_utils::count_rdp_failures_by_ip(
                    self.db,
                    ip,
                    self.since_minutes(minutes),
                )
                .await
            }
            None => 0,
        }
    }

    /// Vrai si l'IP source du log est publique (externe), c.-à-d. ni privée
    /// (RFC 1918), ni loopback, ni link-local. Sert à ne cibler que le RDP
    /// réellement exposé sur Internet et à écarter les faux positifs internes.
    pub fn is_source_ip_external(&self) -> bool {
        use std::net::IpAddr;
        match self
            .log
            .ip_address
            .as_deref()
            .and_then(|s| s.parse::<IpAddr>().ok())
        {
            Some(IpAddr::V4(ip)) => {
                !(ip.is_private() || ip.is_loopback() || ip.is_link_local())
            }
            Some(IpAddr::V6(ip)) => !ip.is_loopback(),
            None => false,
        }
    }

    /// Volume cumulé (octets) sortant de cette source vers des destinations
    /// externes sur la fenêtre — utilisé par la détection d'exfiltration.
    pub async fn sum_external_bytes_out_source(&self, minutes: i64) -> i64 {
        rule_utils::sum_external_bytes_out_by_source(
            self.db,
            &self.log.source_name,
            self.since_minutes(minutes),
        )
        .await
    }

    /// Nombre d'écritures fichiers portant un marqueur de ransomware (extension
    /// de chiffrement ou note de rançon) pour cette source sur la fenêtre.
    pub async fn count_ransomware_writes_source(&self, minutes: i64) -> i64 {
        rule_utils::count_ransomware_writes_by_source(
            self.db,
            &self.log.source_name,
            self.since_minutes(minutes),
        )
        .await
    }
}