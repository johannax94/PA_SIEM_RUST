//! RÈGLE : Brute-force RDP depuis Internet — MITRE T1110.001 / T1021.001
//! 20 échecs RDP (logon_type 10) / 5 min depuis une IP externe. Sévérité high.
//! Doc détaillée (menace, télémétrie, faux positifs) : voir soutenance_ideas (racine du repo).

use crate::services::rule_context::RuleContext;

/// LogonType 10 = RemoteInteractive (session RDP).
const LOGON_TYPE_RDP: &str = "10";
/// Nombre d'échecs à partir duquel on déclenche.
const FAIL_THRESHOLD: i64 = 20;
/// Fenêtre d'observation (minutes).
const WINDOW_MIN: i64 = 5;

pub async fn check_rdp_bruteforce(ctx: &RuleContext<'_>) {

    // 1. Télémétrie primitive : échec de logon (Windows 4625).
    if !ctx.is_event("login_failed") {
        return;
    }

    // 2. On isole le RDP via le type de session (RemoteInteractive).
    if ctx.raw_str("logon_type").as_deref() != Some(LOGON_TYPE_RDP) {
        return;
    }

    // 3. On ne garde que le RDP réellement exposé (IP source externe).
    if !ctx.is_source_ip_external() {
        return;
    }

    // 4. Seuil : assez d'échecs RDP depuis cette IP sur la fenêtre ?
    if ctx.count_rdp_failures_ip(WINDOW_MIN).await < FAIL_THRESHOLD {
        return;
    }

    // 5. Alerte (dédupliquée sur la fenêtre).
    ctx.alert_once(
        "rdp_bruteforce_external",
        "high",
        &format!(
            "[MITRE T1110.001 / T1021.001] Brute-force RDP : {}+ échecs de logon RDP \
             depuis l'IP externe {} en {} min",
            FAIL_THRESHOLD,
            ctx.log.ip_address.as_deref().unwrap_or("?"),
            WINDOW_MIN,
        ),
        WINDOW_MIN,
    )
    .await;
}
