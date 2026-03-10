use sqlx::PgPool;

use crate::rules;

pub async fn run_rules(
    db: &PgPool,
    source_name: &str,
    event_type: &str,
) {

    if event_type == "login_failed" {

        rules::failed_login::check_failed_login_rule(
            db,
            source_name
        ).await;

    }
}