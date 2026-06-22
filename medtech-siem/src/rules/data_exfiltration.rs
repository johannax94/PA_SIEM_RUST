use crate::services::rule_context::RuleContext;

/// Seuil de volume sortant considéré comme suspect : 100 Mo.
const BYTES_THRESHOLD: i64 = 100_000_000;

/// 🚨 Exfiltration de données : transfert sortant volumineux
/// (raw_log->>'bytes_out' au-dessus du seuil).
pub async fn check_data_exfiltration(ctx: &RuleContext<'_>) {

    if !ctx.is_event("data_transfer") {
        return;
    }

    let bytes = ctx.raw_i64("bytes_out").unwrap_or(0);

    if bytes < BYTES_THRESHOLD {
        return;
    }

    ctx.alert_once(
        "data_exfiltration",
        "critical",
        &format!("Large outbound data transfer: {} bytes", bytes),
        10,
    )
    .await;
}
