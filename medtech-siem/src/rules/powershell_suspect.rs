//! RÈGLE : PowerShell suspect — MITRE T1059.001 / T1027 / T1105
//! Marqueurs pondérés : 1 fort (-enc, DownloadString...) OU 2 faibles (-nop, IEX...). critical / high.
//! Doc détaillée (menace, télémétrie, faux positifs) : voir soutenance_ideas (racine du repo).

use crate::services::rule_context::RuleContext;

/// Marqueurs FORTS : un seul suffit, pas d'usage légitime courant.
const STRONG_MARKERS: [&str; 7] = [
    "-enc",
    "encodedcommand",
    "frombase64string",
    "downloadstring",
    "downloadfile",
    "invoke-mimikatz",
    "amsiutils",
];

/// Marqueurs FAIBLES : usage admin légitime possible, il en faut au moins 2.
const WEAK_MARKERS: [&str; 8] = [
    "-nop",
    "noprofile",
    "bypass",
    "hidden",
    "iex",
    "invoke-expression",
    "invoke-webrequest",
    "net.webclient",
];

/// Nombre de marqueurs faibles requis en l'absence de marqueur fort.
const WEAK_THRESHOLD: usize = 2;

pub async fn check_powershell_suspect(ctx: &RuleContext<'_>) {

    // 1. Télémétrie : une création de processus PowerShell.
    if !ctx.is_event("process_create") {
        return;
    }
    if !(ctx.message_contains("powershell") || ctx.message_contains("pwsh")) {
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

    // 3. Sévérité basée sur le risque : obfuscation avérée vs évasion probable.
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
        "powershell_suspect",
        severity,
        &format!(
            "[MITRE T1059.001 + T1027] PowerShell suspect sur '{}' (user '{}') : \
             marqueurs [{}] dans la ligne de commande",
            host, user, markers,
        ),
        5,
    )
    .await;
}
