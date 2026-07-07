use crate::services::rule_context::RuleContext;

/// 🚨 Exécution de cmd.exe : lancement d'un interpréteur de commandes.
pub async fn check_cmd_execution(ctx: &RuleContext<'_>) {

    if !(ctx.is_event("process_create") && ctx.message_contains("cmd.exe")) {
        return;
    }

    ctx.alert_once(
        "cmd_execution",
        "medium",
        "cmd.exe execution detected",
        5,
    )
    .await;
}
