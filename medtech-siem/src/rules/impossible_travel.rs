//! RÈGLE : Impossible travel (même compte, 2 pays en peu de temps) — MITRE T1078
//! Même utilisateur connecté avec succès depuis >= 2 pays distincts / 60 min. Sévérité high.
//! Doc détaillée (menace, télémétrie, faux positifs) : voir soutenance_ideas (racine du repo).

use crate::services::rule_context::RuleContext;

/// Nombre de pays distincts (sur des logins réussis) au-delà duquel on alerte.
const DISTINCT_COUNTRIES_THRESHOLD: i64 = 2;
/// Fenêtre d'observation (minutes).
const WINDOW_MIN: i64 = 60;

pub async fn check_impossible_travel(ctx: &RuleContext<'_>) {

    // 1. Télémétrie : une connexion RÉUSSIE (l'anomalie porte sur des accès réels).
    if !ctx.is_event("login_success") {
        return;
    }

    // 2. Le même compte s'est-il connecté depuis >= 2 pays sur la fenêtre ?
    //    (helper scopé par username, via raw_log->>'country'.)
    let countries = ctx.count_distinct_countries_user(WINDOW_MIN).await;
    if countries < DISTINCT_COUNTRIES_THRESHOLD {
        return;
    }

    // 3. Alerte : un humain ne peut pas être physiquement dans 2 pays en 1 h.
    let user = ctx.log.username.as_deref().unwrap_or("?");
    ctx.alert_once(
        "impossible_travel",
        "high",
        &format!(
            "[MITRE T1078] Impossible travel : le compte '{}' s'est connecté avec \
             succès depuis {} pays différents en {} min — compte potentiellement \
             partagé ou compromis",
            user, countries, WINDOW_MIN,
        ),
        WINDOW_MIN,
    )
    .await;
}
