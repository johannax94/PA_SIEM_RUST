//! RÈGLE : Suppression des shadow copies (précurseur ransomware) — MITRE T1490
//! vssadmin/wmic/wbadmin delete ou bcdedit (récupération off). Un seul suffit. critical.
//! Doc détaillée (menace, télémétrie, faux positifs) : voir soutenance_ideas (racine du repo).

use crate::services::rule_context::RuleContext;

pub async fn check_shadow_copy_deletion(ctx: &RuleContext<'_>) {

    if !ctx.is_event("process_create") {
        return;
    }

    // Un des marqueurs de sabotage de la récupération.
    let hit =
        (ctx.message_contains("vssadmin") && ctx.message_contains("delete"))
        || (ctx.message_contains("wmic")
            && ctx.message_contains("shadowcopy")
            && ctx.message_contains("delete"))
        || (ctx.message_contains("wbadmin") && ctx.message_contains("delete"))
        || (ctx.message_contains("bcdedit")
            && (ctx.message_contains("recoveryenabled no")
                || ctx.message_contains("ignoreallfailures")));

    if !hit {
        return;
    }

    let host = ctx.log.hostname.as_deref().unwrap_or("?");

    ctx.alert_once(
        "shadow_copy_deletion",
        "critical",
        &format!(
            "[MITRE T1490] Suppression des shadow copies / sabotage de la \
             récupération sur '{}' — précurseur direct de ransomware",
            host,
        ),
        30,
    )
    .await;
}
