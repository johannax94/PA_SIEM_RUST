use crate::services::rule_context::RuleContext;

/// 🚨 PowerShell suspect : ligne de commande PowerShell avec options
/// d'obfuscation / téléchargement typiques d'un usage malveillant.
pub async fn check_powershell_suspect(ctx: &RuleContext<'_>) {

    if !(ctx.is_event("process_create") && ctx.message_contains("powershell")) {
        return;
    }

    let suspicious = ctx.message_contains("-enc")
        || ctx.message_contains("encodedcommand")
        || ctx.message_contains("-nop")
        || ctx.message_contains("bypass")
        || ctx.message_contains("hidden")
        || ctx.message_contains("downloadstring")
        || ctx.message_contains("iex");

    if !suspicious {
        return;
    }

    ctx.alert_once(
        "powershell_suspect",
        "high",
        "Suspicious PowerShell command line detected",
        5,
    )
    .await;
}
