use sqlx::PgPool;

pub mod failed_login;

pub async fn run_rules(
    db: &PgPool,
    source_name: &str,
    event_type: &str,
) {

    // On appelle les règles pertinentes

    if event_type == "login_failed" {
        failed_login::check_failed_login_rule(
            db,
            source_name
        ).await;
    }

}