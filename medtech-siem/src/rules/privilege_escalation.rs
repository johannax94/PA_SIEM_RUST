//! RÈGLE : Élévation de privilèges — MITRE T1134 / T1098
//! Privilège sensible activé (SeDebugPrivilege) OU ajout d'un compte à un groupe admin. high.
//! Doc détaillée (menace, télémétrie, faux positifs) : voir soutenance_ideas (racine du repo).

use crate::services::rule_context::RuleContext;

/// Groupes privilégiés dont l'ajout d'un membre est sensible (FR/EN, local/domaine).
const ADMIN_GROUPS: [&str; 4] = [
    "administrators",
    "administrateurs",
    "domain admins",
    "admins du domaine",
];

pub async fn check_privilege_escalation(ctx: &RuleContext<'_>) {

    // Deux voies d'élévation : abus de jeton de privilège, ou ajout à un
    // groupe administrateur.
    let token_abuse = ctx.is_event("privilege_escalation")
        || ctx.message_contains("sedebugprivilege");
    let admin_group_add = ctx.is_event("group_membership_change")
        && ADMIN_GROUPS.iter().any(|g| ctx.message_contains(g));

    if !(token_abuse || admin_group_add) {
        return;
    }

    let host = ctx.log.hostname.as_deref().unwrap_or("?");
    let user = ctx.log.username.as_deref().unwrap_or("?");
    let detail = if token_abuse {
        "activation d'un privilège sensible (SeDebugPrivilege)"
    } else {
        "ajout d'un compte à un groupe administrateur"
    };

    // NB : la séquence "compte CRÉÉ puis promu admin" est couverte, plus
    // sévèrement (critical), par la règle new_account_admin.
    ctx.alert_once(
        "privilege_escalation",
        "high",
        &format!(
            "[MITRE T1134 / T1098] Élévation de privilèges sur '{}' (user '{}') : {}",
            host, user, detail,
        ),
        10,
    )
    .await;
}
