//! RÈGLE : Brute-force ciblé sur un compte — MITRE T1110.001
//! 10 échecs / 5 min sur un MÊME compte (username). critical si privilégié, sinon high.
//! Doc détaillée (menace, télémétrie, faux positifs) : voir soutenance_ideas (racine du repo).

use crate::services::rule_context::RuleContext;

/// Nombre d'échecs sur un même compte à partir duquel on alerte.
const THRESHOLD: i64 = 10;
/// Fenêtre d'observation (minutes).
const WINDOW_MIN: i64 = 5;
/// Comptes considérés comme privilégiés (élève la sévérité).
const PRIVILEGED: [&str; 6] = ["admin", "administrator", "administrateur", "root", "sa", "svc-admin"];

pub async fn check_failed_login_rule(ctx: &RuleContext<'_>) {

    // 1. Télémétrie : un échec d'authentification.
    if !ctx.is_event("login_failed") {
        return;
    }

    // 2. Assez d'échecs sur CE compte sur la fenêtre ?
    if ctx.count_user("login_failed", WINDOW_MIN).await < THRESHOLD {
        return;
    }

    // 3. Sévérité basée sur le risque : le compte visé est-il privilégié ?
    let user = ctx.log.username.as_deref().unwrap_or("?");
    let privileged = PRIVILEGED.iter().any(|p| user.eq_ignore_ascii_case(p));
    let severity = if privileged { "critical" } else { "high" };

    // 4. Alerte (dédupliquée sur la fenêtre).
    ctx.alert_once(
        "multiple_failed_logins",
        severity,
        &format!(
            "[MITRE T1110.001] Brute-force ciblé : {} échecs de connexion sur le compte '{}' en {} min{}",
            THRESHOLD,
            user,
            WINDOW_MIN,
            if privileged { " (compte privilégié)" } else { "" },
        ),
        WINDOW_MIN,
    )
    .await;
}
