use crate::services::rule_context::RuleContext;

pub async fn check_failed_login_rule(
    ctx: &RuleContext<'_>,
) {

    if !ctx.is_event("login_failed") {
        return;
    }

    if ctx
        .count_source(
            "login_failed",
            1,
        )
        .await
        < 5
    {
        return;
    }

    if ctx
        .alert_exists(
            "multiple_failed_logins",
            1,
        )
        .await
    {
        return;
    }

    ctx.alert(
        "multiple_failed_logins",
        "high",
        "5 failed logins detected in less than one minute",
    )
    .await;
}