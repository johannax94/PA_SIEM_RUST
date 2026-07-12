//! RÈGLE : Brute-force RÉUSSI (succès après rafale d'échecs) — MITRE T1110 / T1078
//! login_success après >= 5 échecs / 10 min sur le même compte. critical (compte présumé compromis).
//! Doc détaillée (menace, télémétrie, faux positifs) : voir soutenance_ideas (racine du repo).

use crate::services::rule_context::RuleContext;

/// Nombre d'échecs récents sur le compte à partir duquel un succès est suspect.
const FAILED_THRESHOLD: i64 = 5;
/// Fenêtre d'observation des échecs précédant le succès (minutes).
const WINDOW_MIN: i64 = 10;

pub async fn check_bruteforce_success(ctx: &RuleContext<'_>) {

    // 1. Télémétrie : une connexion réussie.
    if !ctx.is_event("login_success") {
        return;
    }

    // 2. Ce compte sort-il d'une rafale d'échecs ?
    let failures = ctx.count_user("login_failed", WINDOW_MIN).await;
    if failures < FAILED_THRESHOLD {
        return;
    }

    // 3. Succès après rafale = compromission présumée.
    let user = ctx.log.username.as_deref().unwrap_or("?");
    let ip = ctx.log.ip_address.as_deref().unwrap_or("?");

    ctx.alert_once(
        "bruteforce_success",
        "critical",
        &format!(
            "[MITRE T1110 + T1078] Brute-force RÉUSSI présumé : connexion de '{}' \
             (depuis {}) après {} échecs en {} min — compte à désactiver et \
             mot de passe à réinitialiser immédiatement",
            user, ip, failures, WINDOW_MIN,
        ),
        WINDOW_MIN,
    )
    .await;
}
