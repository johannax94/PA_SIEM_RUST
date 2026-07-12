//! RÈGLE : Password spraying (1 IP, beaucoup de comptes) — MITRE T1110.003
//! 5 comptes DISTINCTS échouant depuis la MÊME IP / 5 min. Sévérité high.
//! Doc détaillée (menace, télémétrie, faux positifs) : voir soutenance_ideas (racine du repo).

use crate::services::rule_context::RuleContext;

/// Nombre de comptes DISTINCTS visés depuis une même IP à partir duquel on alerte.
const DISTINCT_USERS_THRESHOLD: i64 = 5;
/// Fenêtre d'observation (minutes).
const WINDOW_MIN: i64 = 5;

pub async fn check_password_spray(ctx: &RuleContext<'_>) {

    // 1. Télémétrie : un échec d'authentification.
    if !ctx.is_event("login_failed") {
        return;
    }

    // 2. Signal de spray : combien de comptes DISTINCTS échouent depuis CETTE IP ?
    //    (count_distinct_users est scopé par l'IP source du log courant — c'est
    //    ce qui distingue le spray du brute-force ciblé, qui martèle UN compte.)
    let users = ctx.count_distinct_users("login_failed", WINDOW_MIN).await;
    if users < DISTINCT_USERS_THRESHOLD {
        return;
    }

    // 3. Alerte (dédupliquée sur la fenêtre).
    let ip = ctx.log.ip_address.as_deref().unwrap_or("?");
    ctx.alert_once(
        "password_spray",
        "high",
        &format!(
            "[MITRE T1110.003] Password spraying probable : {} comptes distincts \
             visés depuis l'IP {} en {} min (peu d'essais par compte pour éviter \
             le verrouillage)",
            users, ip, WINDOW_MIN,
        ),
        WINDOW_MIN,
    )
    .await;
}
