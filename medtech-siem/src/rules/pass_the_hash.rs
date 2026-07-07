use crate::services::rule_context::RuleContext;

/// 🚨 Pass-the-Hash : ouverture de session réseau NTLM de type 9
/// (NewCredentials), signature classique d'un rejeu de hash.
pub async fn check_pass_the_hash(ctx: &RuleContext<'_>) {

    let hit = ctx.is_event("pass_the_hash")
        || (ctx.is_event("logon")
            && ctx.raw_str("logon_type").as_deref() == Some("9")
            && ctx.message_contains("ntlm"));

    if !hit {
        return;
    }

    ctx.alert_once(
        "pass_the_hash",
        "critical",
        "Possible Pass-the-Hash (NTLM logon type 9)",
        10,
    )
    .await;
}
