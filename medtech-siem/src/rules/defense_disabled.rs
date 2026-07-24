//! RÈGLE : Défenses de sécurité désactivées — MITRE T1562.001
//! AV Defender désactivé / exclusion ajoutée / service WinDefend arrêté / pare-feu coupé. high.
//! Doc détaillée (menace, télémétrie, faux positifs) : voir soutenance_ideas (racine du repo).

use crate::services::rule_context::RuleContext;

pub async fn check_defense_disabled(ctx: &RuleContext<'_>) {

    if !ctx.is_event("process_create") {
        return;
    }

    // Désactivation de Defender, exclusion, arrêt de service, ou pare-feu off.
    let hit =
        ctx.message_contains("disablerealtimemonitoring")
        || ctx.message_contains("disableantispyware")
        || ctx.message_contains("-exclusionpath")
        || ctx.message_contains("add-mppreference")
        || (ctx.message_contains("windefend")
            && (ctx.message_contains("stop") || ctx.message_contains("disable")))
        || (ctx.message_contains("netsh advfirewall")
            && ctx.message_contains("off"));

    if !hit {
        return;
    }

    let host = ctx.log.hostname.as_deref().unwrap_or("?");
    let user = ctx.log.username.as_deref().unwrap_or("?");

    ctx.alert_once(
        "defense_disabled",
        "high",
        &format!(
            "[MITRE T1562.001] Désactivation d'une protection de sécurité \
             (antivirus/pare-feu) sur '{}' (user '{}')",
            host, user,
        ),
        30,
    )
    .await;
}
