//! RÈGLE : Compte créé puis promu administrateur — MITRE T1136.001 / T1098
//! Compte ajouté à un groupe admin ET créé (account_created) dans les 60 min. critical (porte dérobée).
//! Doc détaillée (menace, télémétrie, faux positifs) : voir soutenance_ideas (racine du repo).

use crate::services::rule_context::RuleContext;

/// Fenêtre max (minutes) entre la création du compte et sa promotion admin.
const WINDOW_MIN: i64 = 60;
/// Noms de groupes considérés comme privilégiés (FR/EN, local et domaine).
const ADMIN_GROUPS: [&str; 4] = [
    "administrators",
    "administrateurs",
    "domain admins",
    "admins du domaine",
];

pub async fn check_new_account_admin(ctx: &RuleContext<'_>) {

    // 1. Télémétrie : un ajout de membre à un groupe administrateur.
    if !ctx.is_event("group_membership_change") {
        return;
    }
    if !ADMIN_GROUPS.iter().any(|g| ctx.message_contains(g)) {
        return;
    }

    // 2. Corrélation : CE compte a-t-il été créé dans la fenêtre récente ?
    if ctx.count_user("account_created", WINDOW_MIN).await < 1 {
        return;
    }

    // 3. Séquence création -> promotion admin = porte dérobée probable.
    let user = ctx.log.username.as_deref().unwrap_or("?");

    ctx.alert_once(
        "new_account_admin_group",
        "critical",
        &format!(
            "[MITRE T1136.001 + T1098] Porte dérobée probable : le compte '{}' \
             a été créé puis ajouté à un groupe administrateur en moins de {} min",
            user, WINDOW_MIN,
        ),
        WINDOW_MIN,
    )
    .await;
}
