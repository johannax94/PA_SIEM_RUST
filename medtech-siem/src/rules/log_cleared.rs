//! RÈGLE : Journal d'audit effacé — MITRE T1070.001
//! Event dédié 1102/104 OU wevtutil cl / Clear-EventLog. Un seul suffit (pas de seuil). critical.
//! Doc détaillée (menace, télémétrie, faux positifs) : voir soutenance_ideas (racine du repo).

use crate::services::rule_context::RuleContext;

pub async fn check_log_cleared(ctx: &RuleContext<'_>) {

    // Événement dédié (1102/104) OU effacement via ligne de commande.
    let via_event = ctx.is_event("audit_log_cleared");
    let via_cmd = ctx.is_event("process_create")
        && (ctx.message_contains("wevtutil cl")
            || ctx.message_contains("wevtutil clear-log")
            || ctx.message_contains("clear-eventlog")
            || ctx.message_contains("clear-winevent"));

    if !(via_event || via_cmd) {
        return;
    }

    let host = ctx.log.hostname.as_deref().unwrap_or("?");
    let user = ctx.log.username.as_deref().unwrap_or("?");

    ctx.alert_once(
        "audit_log_cleared",
        "critical",
        &format!(
            "[MITRE T1070.001] Journal d'audit effacé sur '{}' (user '{}') — \
             effacement des traces, compromission probable",
            host, user,
        ),
        60,
    )
    .await;
}
