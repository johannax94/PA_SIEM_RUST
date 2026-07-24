//! RÈGLE : RDP depuis un pays inhabituel — MITRE T1021.001 / T1133
//! Connexion RDP dont le pays (GeoIP) n'est pas dans la liste autorisée (FR). Sévérité high.
//! Doc détaillée (menace, télémétrie, faux positifs) : voir soutenance_ideas (racine du repo).

use crate::services::rule_context::RuleContext;

/// Pays autorisés pour une connexion RDP (à adapter au périmètre de la PME).
const ALLOWED_COUNTRIES: [&str; 2] = ["FR", "FRANCE"];

pub async fn check_rdp_foreign_country(ctx: &RuleContext<'_>) {

    // 1. Télémétrie : une connexion RDP (Bureau à distance).
    if !ctx.is_event("rdp_login") {
        return;
    }

    // 2. Pays de l'IP source (GeoIP), fourni dans raw_log->>'country'.
    let country = match ctx.raw_str("country") {
        Some(c) => c,
        None => return,
    };

    // 3. Hors de la liste autorisée -> accès distant géographiquement anormal.
    if ALLOWED_COUNTRIES.contains(&country.to_uppercase().as_str()) {
        return;
    }

    let user = ctx.log.username.as_deref().unwrap_or("?");
    ctx.alert_once(
        "rdp_foreign_country",
        "high",
        &format!(
            "[MITRE T1021.001 / T1133] Connexion RDP du compte '{}' depuis un pays \
             inhabituel ({}) — accès distant potentiellement malveillant",
            user, country,
        ),
        60,
    )
    .await;
}
