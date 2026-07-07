use crate::services::rule_context::RuleContext;

/// 🚨 Beaconing C2 : connexions sortantes répétées et régulières depuis une
/// même IP (communication périodique vers un serveur de commande). Version
/// simplifiée par volume sur une fenêtre élargie.
pub async fn check_beaconing_c2(ctx: &RuleContext<'_>) {

    if !ctx.is_event("outbound_connection") {
        return;
    }

    if ctx.count_ip("outbound_connection", 10).await < 30 {
        return;
    }

    ctx.alert_once(
        "beaconing_c2",
        "medium",
        "Possible C2 beaconing: repeated regular outbound connections",
        30,
    )
    .await;
}
