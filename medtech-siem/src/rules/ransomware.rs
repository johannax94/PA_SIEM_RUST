//! ============================================================================
//! RÈGLE : Ransomware (chiffrement massif de fichiers)
//! ============================================================================
//!
//! MENACE (contexte PME)
//!   Le ransomware est la menace n°1 des PME : il chiffre tous les fichiers
//!   partagés et paralyse l'activité (souvent avec double extorsion). Détecter
//!   le chiffrement EN COURS permet d'isoler le poste avant qu'il ne finisse.
//!
//! MITRE ATT&CK
//!   - T1486  Data Encrypted for Impact
//!
//! TÉLÉMÉTRIE PRIMITIVE
//!   Événements du système de fichiers / EDR (event_type `file_modified` ou
//!   `file_renamed`), avec dans raw_log :
//!     - `new_extension` : l'extension après renommage
//!     - `filename`      : le nom du fichier écrit
//!
//! LOGIQUE DE DÉTECTION
//!   On ne compte PAS le simple volume de fichiers modifiés (une sauvegarde ou
//!   une copie en modifie beaucoup, sans être une attaque). On compte les
//!   écritures portant un MARQUEUR de ransomware :
//!     - extension de chiffrement connue (.locked, .encrypted, .lockbit…) ou
//!       extension en hexadécimal aléatoire, OU
//!     - nom de fichier de note de rançon (READ_ME, HOW_TO_DECRYPT…).
//!   Volume + marqueur = chiffrement en cours.
//!
//! SEUIL — JUSTIFICATION
//!   20 fichiers « marqués » / 3 min sur un même poste. Le ransomware chiffre
//!   vite et en masse ; une copie légitime n'a AUCUN de ces marqueurs -> pas de
//!   faux positif de volume.
//!
//! AVANT (version faible) : `if 100 fichiers modifiés -> alerte`
//!   -> se déclenchait sur les sauvegardes, copies de dossiers, scans antivirus.
//!
//! FAUX POSITIFS & LIMITES
//!   - Outil de chiffrement légitime (ex. archivage sécurisé) -> liste blanche
//!     du poste/processus.
//!   - Ne détecte pas un ransomware sans changement d'extension ni note (rare) ;
//!     un signal d'entropie (EDR) serait complémentaire.
//!
//! SÉVÉRITÉ : critical (impact business immédiat).
//! ============================================================================

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
