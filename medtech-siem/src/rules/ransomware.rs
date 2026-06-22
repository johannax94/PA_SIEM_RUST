use crate::services::rule_context::RuleContext;

/// 🚨 Détection de ransomware : modification massive de fichiers depuis une
/// même source sur une fenêtre très courte (chiffrement en cours).
pub async fn check_ransomware(ctx: &RuleContext<'_>) {

    if !ctx.is_event("file_modified") {
        return;
    }

    if ctx.count_source("file_modified", 1).await < 100 {
        return;
    }

    ctx.alert_once(
        "ransomware",
        "critical",
        "Possible ransomware: mass file modification detected",
        5,
    )
    .await;
}
