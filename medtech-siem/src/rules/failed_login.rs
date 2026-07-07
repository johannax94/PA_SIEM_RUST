//! ============================================================================
//! RÈGLE : Brute-force ciblé sur un compte
//! ============================================================================
//!
//! MENACE (contexte PME)
//!   Un attaquant s'acharne sur UN compte précis — typiquement l'admin, la
//!   compta ou le dirigeant, dont l'identifiant a fuité ou a été deviné (OSINT).
//!   À distinguer du password spraying (beaucoup de comptes, peu d'essais) :
//!   ici c'est un seul compte qui est martelé.
//!
//! MITRE ATT&CK
//!   - T1110.001  Brute Force: Password Guessing
//!
//! TÉLÉMÉTRIE PRIMITIVE
//!   Échec d'authentification (Windows Event 4625, échec SSH/VPN/webmail…),
//!   normalisé en event_type `login_failed`, portant le `username` visé.
//!
//! LOGIQUE DE DÉTECTION
//!   Compter les échecs pour un MÊME compte sur une fenêtre courte, puis
//!   pondérer la sévérité selon que le compte est privilégié ou non.
//!
//! SEUIL — JUSTIFICATION
//!   10 échecs / 5 min sur le même compte. Un utilisateur légitime se trompe
//!   2-3 fois puis réinitialise ; 10 essais rapprochés = outil automatisé ou
//!   attaque ciblée.
//!
//! SÉVÉRITÉ — BASÉE SUR LE RISQUE
//!   critical si le compte est privilégié (admin/root…), sinon high.
//!
//! FAUX POSITIFS & LIMITES
//!   - Mot de passe oublié / client mal configuré qui reboucle -> le seuil
//!     limite le bruit ; on peut exclure des comptes de service connus.
//!   - Le verrouillage de compte (lockout) peut stopper avant le seuil.
//! ============================================================================

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
