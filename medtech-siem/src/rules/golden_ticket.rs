use crate::services::rule_context::RuleContext;

/// 🚨 Golden Ticket : ticket TGT Kerberos forgé (anomalie de durée de vie /
/// chiffrement sur l'émission d'un TGT).
pub async fn check_golden_ticket(ctx: &RuleContext<'_>) {

    let hit = ctx.is_event("golden_ticket")
        || (ctx.is_event("kerberos_tgt") && ctx.message_contains("anomaly"));

    if !hit {
        return;
    }

    ctx.alert_once(
        "golden_ticket",
        "critical",
        "Possible Golden Ticket (forged Kerberos TGT) detected",
        30,
    )
    .await;
}
