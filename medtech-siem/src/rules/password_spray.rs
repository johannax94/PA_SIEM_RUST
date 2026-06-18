use crate::services::rule_context::RuleContext;

pub async fn check_password_spray(
    ctx: &RuleContext<'_>,
) {

    if !ctx.is_event("login_failed") {
        return;
    }

    let users = ctx
        .count_distinct_users(
            "login_failed",
            5,
        )
        .await;

    if users < 5 {
        return;
    }

    if ctx
        .alert_exists(
            "password_spray",
            5,
        )
        .await
    {
        return;
    }

    ctx.alert(
        "password_spray",
        "critical",
        "Multiple accounts targeted from one IP",
    )
    .await;
}