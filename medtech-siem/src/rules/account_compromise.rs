use crate::services::rule_context::RuleContext;

pub async fn check_account_compromise(
    ctx: &RuleContext<'_>,
) {

    if !ctx.is_event("login_success") {
        return;
    }

    let failures = ctx
        .count_user(
            "login_failed",
            5,
        )
        .await;

    if failures < 5 {
        return;
    }

    if ctx
        .alert_exists(
            "account_compromise",
            5,
        )
        .await
    {
        return;
    }

    ctx.alert(
        "account_compromise",
        "high",
        "Successful login after multiple failed logins",
    )
    .await;
}