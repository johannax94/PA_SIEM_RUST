//! RÈGLE : cmd.exe suspect — MITRE T1059.003 (+ Discovery / Persistence / T1105)
//! Marqueurs pondérés : 1 fort (certutil, /add...) OU 2 faibles (whoami, net user...). critical / high.
//! Doc détaillée (menace, télémétrie, faux positifs) : voir soutenance_ideas (racine du repo).

use crate::services::rule_context::RuleContext;

/// Marqueurs FORTS : un seul suffit (transfert d'outil ou persistence).
/// `/add` couvre `net user /add` comme `net localgroup administrators ... /add`.
const STRONG_MARKERS: [&str; 5] = [
    "certutil",
    "bitsadmin",
    "sc create",
    "schtasks /create",
    "/add",
];

/// Marqueurs FAIBLES : commande de reconnaissance, il en faut au moins 2.
const WEAK_MARKERS: [&str; 8] = [
    "whoami",
    "net user",
    "net group",
    "ipconfig",
    "systeminfo",
    "tasklist",
    "nltest",
    "reg query",
];

/// Nombre de marqueurs faibles requis en l'absence de marqueur fort.
const WEAK_THRESHOLD: usize = 2;

pub async fn check_cmd_execution(ctx: &RuleContext<'_>) {

    // 1. Télémétrie : une création de processus impliquant le shell Windows.
    if !ctx.is_event("process_create") {
        return;
    }
    if !(ctx.message_contains("cmd.exe") || ctx.message_contains("cmd /")) {
        return;
    }

    // 2. Marqueurs pondérés : un fort suffit, les faibles vont par deux.
    let strong: Vec<&str> = STRONG_MARKERS
        .iter()
        .filter(|m| ctx.message_contains(m))
        .copied()
        .collect();

    let weak: Vec<&str> = WEAK_MARKERS
        .iter()
        .filter(|m| ctx.message_contains(m))
        .copied()
        .collect();

    if strong.is_empty() && weak.len() < WEAK_THRESHOLD {
        return;
    }

    // 3. Sévérité basée sur le risque : transfert/persistence vs reconnaissance.
    let severity = if strong.is_empty() { "high" } else { "critical" };
    let markers = strong
        .iter()
        .chain(weak.iter())
        .copied()
        .collect::<Vec<_>>()
        .join(", ");

    let host = ctx.log.hostname.as_deref().unwrap_or("?");
    let user = ctx.log.username.as_deref().unwrap_or("?");

    ctx.alert_once(
        "cmd_suspect",
        severity,
        &format!(
            "[MITRE T1059.003] Ligne de commande cmd.exe suspecte sur '{}' \
             (user '{}') : marqueurs [{}]",
            host, user, markers,
        ),
        5,
    )
    .await;
}
