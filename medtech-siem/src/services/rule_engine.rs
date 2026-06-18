use sqlx::PgPool;

use crate::models::log::IncomingLog;
use crate::rules;
use crate::services::rule_context::RuleContext;

pub async fn run_rules(
    db: &PgPool,
    log: &IncomingLog,
) {

    let ctx =
        RuleContext::new(
            db,
            log,
        );

        rules::failed_login::check_failed_login_rule(&ctx).await;

        rules::password_spray::check_password_spray(&ctx).await;

        rules::account_compromise::check_account_compromise(&ctx).await;

//    rules::bruteforce_ip::check_bruteforce_ip(
  //      &ctx,
 //   )
 //   .await;
}