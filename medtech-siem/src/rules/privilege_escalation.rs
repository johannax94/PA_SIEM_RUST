use crate::services::rule_context::RuleContext;

/// 🚨 Privilege Escalation : obtention de privilèges élevés ou ajout d'un
/// compte au groupe administrateurs.
pub async fn check_privilege_escalation(ctx: &RuleContext<'_>) {

    let hit = ctx.is_event("privilege_escalation")
        || ctx.message_contains("sedebugprivilege")
        || (ctx.is_event("group_membership_change")
            && ctx.message_contains("administrators"));

    if !hit {
        return;
    }

    ctx.alert_once(
        "privilege_escalation",
        "high",
        "Privilege escalation activity detected",
        10,
    )
    .await;
}
