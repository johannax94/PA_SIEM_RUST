//! RÈGLE : Ransomware (chiffrement massif) — MITRE T1486
//! 20 fichiers "marqués" (extension de chiffrement / note de rançon) / 3 min / poste. critical.
//! Doc détaillée (menace, télémétrie, faux positifs) : voir soutenance_ideas (racine du repo).

use crate::services::rule_context::RuleContext;

/// Nombre de fichiers « marqués » à partir duquel on alerte.
const THRESHOLD: i64 = 20;
/// Fenêtre d'observation (minutes).
const WINDOW_MIN: i64 = 3;

pub async fn check_ransomware(ctx: &RuleContext<'_>) {

    // 1. Télémétrie : un événement d'écriture fichier.
    if !ctx.is_any_event(&["file_modified", "file_renamed"]) {
        return;
    }

    // 2. Assez de fichiers portant un MARQUEUR de ransomware sur la fenêtre ?
    if ctx.count_ransomware_writes_source(WINDOW_MIN).await < THRESHOLD {
        return;
    }

    // 3. Alerte (dédupliquée sur la fenêtre).
    ctx.alert_once(
        "ransomware",
        "critical",
        &format!(
            "[MITRE T1486] Ransomware probable : {}+ fichiers chiffrés/renommés \
             (extension suspecte ou note de rançon) sur {} en {} min",
            THRESHOLD,
            ctx.log.source_name,
            WINDOW_MIN,
        ),
        WINDOW_MIN,
    )
    .await;
}
