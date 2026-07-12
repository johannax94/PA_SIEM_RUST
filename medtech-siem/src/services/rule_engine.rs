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
    // account_compromise : remplacée par bruteforce_success (même logique,
    // sévérité/MITRE/fenêtre revus) — débranchée pour éviter la double alerte.

    // ---- Nouvelles règles ----
    rules::powershell_suspect::check_powershell_suspect(&ctx).await;
    rules::cmd_execution::check_cmd_execution(&ctx).await;
    rules::rdp_foreign_country::check_rdp_foreign_country(&ctx).await;
    rules::impossible_travel::check_impossible_travel(&ctx).await;
    rules::network_scan::check_network_scan(&ctx).await;
    rules::ransomware::check_ransomware(&ctx).await;
    rules::data_exfiltration::check_data_exfiltration(&ctx).await;
    rules::privilege_escalation::check_privilege_escalation(&ctx).await;

    // ---- Règle approfondie (detection engineering + MITRE) ----
    rules::rdp_bruteforce::check_rdp_bruteforce(&ctx).await;
    rules::new_account_admin::check_new_account_admin(&ctx).await;
    rules::bruteforce_success::check_bruteforce_success(&ctx).await;

    // ---- Defense Evasion / kill-chain pré-ransomware ----
    rules::log_cleared::check_log_cleared(&ctx).await;
    rules::shadow_copy_deletion::check_shadow_copy_deletion(&ctx).await;
    rules::defense_disabled::check_defense_disabled(&ctx).await;
    rules::office_spawns_shell::check_office_spawns_shell(&ctx).await;
}