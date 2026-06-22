use crate::services::rule_context::RuleContext;

const ALLOWED_COUNTRIES: [&str; 2] = ["FR", "FRANCE"];

/// 🚨 RDP depuis un pays inhabituel : connexion Bureau à distance dont le
/// pays (raw_log->>'country') n'est pas dans la liste autorisée.
pub async fn check_rdp_foreign_country(ctx: &RuleContext<'_>) {

    if !ctx.is_event("rdp_login") {
        return;
    }

    let country = match ctx.raw_str("country") {
        Some(c) => c,
        None => return,
    };

    if ALLOWED_COUNTRIES.contains(&country.to_uppercase().as_str()) {
        return;
    }

    ctx.alert_once(
        "rdp_foreign_country",
        "high",
        &format!("RDP login from unusual country: {}", country),
        60,
    )
    .await;
}
