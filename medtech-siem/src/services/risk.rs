//! ============================================================================
//! RBA — Risk-Based Alerting
//! ============================================================================
//!
//! PRINCIPE (inspiré de Splunk RBA / QRadar "offenses")
//!   Chaque détection ne crie plus toute seule : elle dépose une observation
//!   de risque scorée sur une entité (la machine source). Un INCIDENT n'est
//!   créé que lorsque le score cumulé de l'entité sur la fenêtre glissante
//!   franchit un seuil — c'est le cumul qui raconte l'attaque, pas la
//!   détection isolée.
//!
//! SCORES PAR DÉTECTION — barème Elastic Security
//!   Mapping officiel sévérité -> risk_score d'Elastic (échelle 0-100) :
//!   low = 21, medium = 47, high = 73, critical = 99.
//!
//! SCORE D'ENTITÉ NORMALISÉ SUR 100 (comme l'Entity Risk Score d'Elastic)
//!   Le cumul brut (somme plafonnée x diversité) est ramené sur une échelle
//!   0-100 : normalisé = brut / 4, plafonné à 100. Bandes de risque :
//!     <= 25  -> low      (une détection critical isolée ~ 25)
//!     <= 75  -> medium
//!      > 75  -> high     (équivaut à 300 points bruts, l'ancien seuil max)
//!   Un incident est ouvert dès que le score atteint MIN_INCIDENT_SCORE.
//!
//! CALCUL DU SCORE BRUT — deux garde-fous
//!   1. Plafond par règle : une même règle qui redéclenche 50 fois ne compte
//!      que MAX_HITS_PER_RULE fois (anti règle bavarde).
//!   2. Bonus de diversité : +25 % par règle DISTINCTE supplémentaire — une
//!      vraie attaque enchaîne des tactiques différentes (recon, exécution,
//!      persistence...), un faux positif se répète à l'identique.
//!
//!   Exemple : multiple_failed_logins (99) + network_scan (73) +
//!   powershell_suspect (73) = 245 x 1,5 (diversité) = 368 brut
//!   -> 92 / 100 -> incident high.
//! ============================================================================

use chrono::{Duration, NaiveDateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

/// Fenêtre glissante d'accumulation du risque (heures) — défaut Splunk RBA.
const WINDOW_HOURS: i64 = 24;
/// Une même règle ne contribue au score qu'au plus N fois sur la fenêtre.
const MAX_HITS_PER_RULE: i32 = 2;
/// Bonus multiplicatif par règle distincte supplémentaire (+25 %).
const DIVERSITY_BONUS: f64 = 0.25;

/// Diviseur de normalisation : score brut / 4, plafonné à 100.
/// (300 points bruts — l'ancien seuil maximal — tombent ainsi sur 75.)
const RAW_TO_100: f64 = 4.0;

/// Bandes de risque sur l'échelle normalisée 0-100.
const BAND_LOW_MAX: f64 = 25.0; // <= 25 : low
const BAND_MEDIUM_MAX: f64 = 75.0; // <= 75 : medium ; au-delà : high

/// Score normalisé minimal pour ouvrir un incident (une détection critical
/// isolée ~ 25 suffit ; en dessous, le risque reste silencieux).
const MIN_INCIDENT_SCORE: f64 = 20.0;

/// Contribution d'une détection au score de risque : mapping officiel
/// sévérité -> risk_score d'Elastic Security (échelle 0-100).
pub fn severity_score(severity: &str) -> i32 {
    match severity {
        "critical" => 99,
        "high" => 73,
        "medium" => 47,
        "low" => 21,
        _ => 5, // info et sévérités inconnues
    }
}

/// Score brut -> score normalisé sur 100.
fn normalize(raw: f64) -> f64 {
    (raw / RAW_TO_100).min(100.0)
}

/// Bande de risque d'un score normalisé (0-100).
fn risk_band(score: f64) -> &'static str {
    if score > BAND_MEDIUM_MAX {
        "high"
    } else if score > BAND_LOW_MAX {
        "medium"
    } else {
        "low"
    }
}

fn window_start() -> NaiveDateTime {
    Utc::now().naive_utc() - Duration::hours(WINDOW_HOURS)
}

/// Point d'entrée appelé par `create_alert` : enregistre l'observation de
/// risque puis réévalue le score cumulé de l'entité.
pub async fn record_risk_event(
    db: &PgPool,
    entity: &str,
    rule_name: &str,
    severity: &str,
    message: &str,
) {
    let _ = sqlx::query(
        r#"
        INSERT INTO risk_events
        (id, entity, rule_name, severity, score, message, created_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        "#,
    )
    .bind(Uuid::new_v4())
    .bind(entity)
    .bind(rule_name)
    .bind(severity)
    .bind(severity_score(severity))
    .bind(message)
    .bind(Utc::now().naive_utc())
    .execute(db)
    .await;

    evaluate_entity(db, entity).await;
}

/// Score cumulé d'une entité sur la fenêtre : somme des contributions
/// plafonnées par règle, multipliée par le bonus de diversité.
async fn compute_entity_score(db: &PgPool, entity: &str) -> (f64, Vec<String>) {
    let per_rule: Vec<(String, i64)> = sqlx::query_as(
        r#"
        SELECT rule_name,
               (MAX(score) * LEAST(COUNT(*), $3))::BIGINT AS capped
        FROM risk_events
        WHERE entity = $1 AND created_at >= $2
        GROUP BY rule_name
        "#,
    )
    .bind(entity)
    .bind(window_start())
    .bind(MAX_HITS_PER_RULE as i64)
    .fetch_all(db)
    .await
    .unwrap_or_default();

    if per_rule.is_empty() {
        return (0.0, vec![]);
    }

    let base: i64 = per_rule.iter().map(|(_, capped)| capped).sum();
    let diversity = 1.0 + DIVERSITY_BONUS * (per_rule.len() as f64 - 1.0);
    let rules = per_rule.into_iter().map(|(rule, _)| rule).collect();

    (base as f64 * diversity, rules)
}

/// Crée ou met à jour l'incident de l'entité si le seuil est franchi.
async fn evaluate_entity(db: &PgPool, entity: &str) {
    let (raw, rules) = compute_entity_score(db, entity).await;

    let score = normalize(raw);
    if score < MIN_INCIDENT_SCORE {
        return;
    }
    let severity = risk_band(score);

    let rules_involved = rules.join(", ");
    let now = Utc::now().naive_utc();

    // Anti-spam : un seul incident ouvert par entité — on le met à jour
    // au lieu d'en empiler de nouveaux à chaque détection suivante.
    let open_incident: Option<(Uuid,)> = sqlx::query_as(
        r#"
        SELECT id FROM risk_incidents
        WHERE entity = $1 AND status = 'open'
        ORDER BY created_at DESC
        LIMIT 1
        "#,
    )
    .bind(entity)
    .fetch_optional(db)
    .await
    .unwrap_or(None);

    match open_incident {
        Some((id,)) => {
            let _ = sqlx::query(
                r#"
                UPDATE risk_incidents
                SET risk_score = $2,
                    severity = $3,
                    rules_involved = $4,
                    updated_at = $5
                WHERE id = $1
                "#,
            )
            .bind(id)
            .bind(score.round() as i32)
            .bind(severity)
            .bind(&rules_involved)
            .bind(now)
            .execute(db)
            .await;
        }
        None => {
            let _ = sqlx::query(
                r#"
                INSERT INTO risk_incidents
                (id, entity, risk_score, severity, status, rules_involved,
                 created_at, updated_at)
                VALUES ($1, $2, $3, $4, 'open', $5, $6, $6)
                "#,
            )
            .bind(Uuid::new_v4())
            .bind(entity)
            .bind(score.round() as i32)
            .bind(severity)
            .bind(&rules_involved)
            .bind(now)
            .execute(db)
            .await;

            tracing::warn!(
                "RBA : incident {severity} ouvert pour '{entity}' \
                 (score {score:.0}, règles : {rules_involved})"
            );
        }
    }
}

/// Vue "entités à risque" pour l'API : score courant de chaque entité ayant
/// au moins une observation sur la fenêtre, calculé avec la même formule.
pub async fn entities_at_risk(
    db: &PgPool,
) -> Vec<(String, f64, i64, String, NaiveDateTime)> {
    let rows: Vec<(String, i64, i64, String, NaiveDateTime)> = sqlx::query_as(
        r#"
        SELECT entity,
               SUM(capped)::BIGINT AS base,
               COUNT(*)::BIGINT AS distinct_rules,
               STRING_AGG(rule_name, ', ' ORDER BY rule_name) AS rules,
               MAX(last_seen) AS last_seen
        FROM (
            SELECT entity,
                   rule_name,
                   (MAX(score) * LEAST(COUNT(*), $2))::BIGINT AS capped,
                   MAX(created_at) AS last_seen
            FROM risk_events
            WHERE created_at >= $1
            GROUP BY entity, rule_name
        ) per_rule
        GROUP BY entity
        "#,
    )
    .bind(window_start())
    .bind(MAX_HITS_PER_RULE as i64)
    .fetch_all(db)
    .await
    .unwrap_or_default();

    let mut entities: Vec<(String, f64, i64, String, NaiveDateTime)> = rows
        .into_iter()
        .map(|(entity, base, distinct_rules, rules, last_seen)| {
            let diversity = 1.0 + DIVERSITY_BONUS * (distinct_rules as f64 - 1.0);
            let score = normalize(base as f64 * diversity);
            (entity, score, distinct_rules, rules, last_seen)
        })
        .collect();

    entities.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    entities
}

/// Bande de risque pour un score normalisé donné, exposée pour l'API.
pub fn tier_label(score: f64) -> &'static str {
    risk_band(score)
}
