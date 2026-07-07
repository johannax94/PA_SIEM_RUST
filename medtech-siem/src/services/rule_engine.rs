use sqlx::PgPool;

use crate::models::log::IncomingLog;
use crate::rules;
use crate::services::rule_context::RuleContext;

pub async fn run_rules(
    db: &PgPool,
    log: &IncomingLog,
) {

    let ctx = RuleContext::new(db, log);

    // ---- Règles existantes ----
    rules::failed_login::check_failed_login_rule(&ctx).await;
    rules::password_spray::check_password_spray(&ctx).await;
    rules::account_compromise::check_account_compromise(&ctx).await;

    // ---- Nouvelles règles ----
    rules::powershell_suspect::check_powershell_suspect(&ctx).await;
    rules::cmd_execution::check_cmd_execution(&ctx).await;
    rules::rdp_foreign_country::check_rdp_foreign_country(&ctx).await;
    rules::impossible_travel::check_impossible_travel(&ctx).await;
    rules::network_scan::check_network_scan(&ctx).await;
    rules::ransomware::check_ransomware(&ctx).await;
    rules::beaconing_c2::check_beaconing_c2(&ctx).await;
    rules::dns_tunneling::check_dns_tunneling(&ctx).await;
    rules::data_exfiltration::check_data_exfiltration(&ctx).await;
    rules::privilege_escalation::check_privilege_escalation(&ctx).await;
    rules::pass_the_hash::check_pass_the_hash(&ctx).await;

    // ---- Règle approfondie (detection engineering + MITRE) ----
    rules::rdp_bruteforce::check_rdp_bruteforce(&ctx).await;
}

