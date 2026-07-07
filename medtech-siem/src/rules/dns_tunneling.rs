use crate::services::rule_context::RuleContext;

/// 🚨 DNS tunneling : volume anormal de requêtes DNS depuis une même source
/// (exfiltration / canal caché via DNS).
pub async fn check_dns_tunneling(ctx: &RuleContext<'_>) {

    if !ctx.is_event("dns_query") {
        return;
    }

    if ctx.count_source("dns_query", 1).await < 100 {
        return;
    }

    ctx.alert_once(
        "dns_tunneling",
        "high",
        "Possible DNS tunneling: abnormal DNS query volume",
        5,
    )
    .await;
}
