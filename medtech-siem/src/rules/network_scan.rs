use crate::services::rule_context::RuleContext;

/// 🚨 Scan réseau : une même IP génère un grand nombre de connexions
/// bloquées en très peu de temps (balayage de ports / hôtes).
pub async fn check_network_scan(ctx: &RuleContext<'_>) {

    if !ctx.is_event("blocked_connection") {
        return;
    }

    if ctx.count_ip("blocked_connection", 1).await < 50 {
        return;
    }

    ctx.alert_once(
        "network_scan",
        "high",
        "Possible network scan: many blocked connections from one IP",
        5,
    )
    .await;
}
