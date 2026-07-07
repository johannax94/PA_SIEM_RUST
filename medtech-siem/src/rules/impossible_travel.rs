use crate::services::rule_context::RuleContext;

/// 🚨 Impossible Travel : un même utilisateur se connecte avec succès depuis
/// au moins 2 pays différents sur une courte fenêtre de temps.
pub async fn check_impossible_travel(ctx: &RuleContext<'_>) {

    if !ctx.is_event("login_success") {
        return;
    }

    if ctx.count_distinct_countries_user(60).await < 2 {
        return;
    }

    ctx.alert_once(
        "impossible_travel",
        "critical",
        "Logins from 2+ countries for the same user in a short time",
        60,
    )
    .await;
}
