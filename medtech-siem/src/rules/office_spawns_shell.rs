//! RÈGLE : Application Office engendre un shell — MITRE T1566.001 / T1204.002
//! Parent Office (Word/Excel/Outlook...) engendre un shell (PowerShell/cmd/wscript...). high.
//! Doc détaillée (menace, télémétrie, faux positifs) : voir soutenance_ideas (racine du repo).

use crate::services::rule_context::RuleContext;

/// Applications Office considérées comme parents suspects.
const OFFICE_PARENTS: [&str; 6] = [
    "winword.exe",
    "excel.exe",
    "powerpnt.exe",
    "outlook.exe",
    "msaccess.exe",
    "mspub.exe",
];

/// Interpréteurs / shells considérés comme enfants suspects.
const SHELL_CHILDREN: [&str; 6] = [
    "powershell",
    "cmd.exe",
    "wscript",
    "cscript",
    "mshta",
    "rundll32",
];

pub async fn check_office_spawns_shell(ctx: &RuleContext<'_>) {

    if !ctx.is_event("process_create") {
        return;
    }

    // Parent = application Office ?
    let parent = ctx.raw_str("parent_process").unwrap_or_default().to_lowercase();
    if !OFFICE_PARENTS.iter().any(|p| parent.contains(p)) {
        return;
    }

    // Enfant = interpréteur / shell ?
    let child = SHELL_CHILDREN.iter().find(|c| ctx.message_contains(c));
    let Some(child) = child else {
        return;
    };

    let host = ctx.log.hostname.as_deref().unwrap_or("?");
    let user = ctx.log.username.as_deref().unwrap_or("?");

    ctx.alert_once(
        "office_spawns_shell",
        "high",
        &format!(
            "[MITRE T1566.001 + T1204.002] '{}' a engendré un shell ({}) sur '{}' \
             (user '{}') — pièce jointe Office malveillante probable",
            parent, child, host, user,
        ),
        10,
    )
    .await;
}
