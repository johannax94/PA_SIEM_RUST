//! ============================================================================
//! RÈGLE : Brute-force RDP depuis Internet
//! ============================================================================
//!
//! MENACE (contexte PME)
//!   Le RDP (Bureau à distance) laissé exposé sur Internet est le **vecteur
//!   d'accès initial n°1 des ransomwares** contre les PME. Un attaquant scanne
//!   le port 3389, puis tente des milliers de mots de passe automatiquement.
//!
//! MITRE ATT&CK
//!   - T1110.001  Brute Force: Password Guessing
//!   - T1021.001  Remote Services: Remote Desktop Protocol
//!
//! TÉLÉMÉTRIE PRIMITIVE (on dérive l'attaque, on ne la « lit » pas)
//!   Événement Windows Security **4625** (« An account failed to log on »),
//!   normalisé en event_type `login_failed`, avec dans raw_log :
//!     - `logon_type = 10`  -> RemoteInteractive (= une session RDP)
//!     - l'IP source (`ip_address`)
//!
//! LOGIQUE DE DÉTECTION
//!   1. échec de logon (4625)
//!   2. de type RDP (logon_type 10)
//!   3. depuis une IP **externe** (le RDP interne = l'IT, pas la menace)
//!   4. >= SEUIL échecs depuis la **même IP** sur la fenêtre
//!
//! SEUIL — JUSTIFICATION
//!   20 échecs / 5 min depuis une seule IP externe. Un utilisateur légitime se
//!   trompe 2-3 fois ; 20 tentatives en 5 min = outil automatisé. La fenêtre
//!   courte + le regroupement par IP ciblent le comportement machine.
//!
//! FAUX POSITIFS & LIMITES
//!   - Un scanner de vulnérabilités autorisé -> à mettre en liste blanche d'IP.
//!   - Plusieurs employés derrière une même IP publique (NAT) -> rare en RDP.
//!   - IPv6 : on considère externe tout ce qui n'est pas loopback.
//!
//! SÉVÉRITÉ : high (précurseur direct de ransomware).
//! ============================================================================

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
