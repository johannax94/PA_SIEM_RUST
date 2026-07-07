//! ============================================================================
//! RÈGLE : Exfiltration de données vers l'externe
//! ============================================================================
//!
//! MENACE (contexte PME)
//!   Vol de données par un salarié sur le départ, un compte compromis, ou la
//!   « double extorsion » d'un ransomware (les données sont exfiltrées AVANT
//!   d'être chiffrées, pour faire pression). C'est l'une des pertes les plus
//!   coûteuses pour une PME (fuite RGPD, secrets clients).
//!
//! MITRE ATT&CK
//!   - T1048  Exfiltration Over Alternative Protocol
//!
//! TÉLÉMÉTRIE PRIMITIVE
//!   Logs de flux réseau proxy/pare-feu normalisés (event_type `network_flow`),
//!   avec dans raw_log :
//!     - `bytes_out`  : octets envoyés
//!     - `dest_ip`    : IP de destination
//!
//! LOGIQUE DE DÉTECTION
//!   On ne regarde PAS un transfert isolé (trop de faux positifs : sauvegardes,
//!   visio, envois ponctuels). On agrège le **volume cumulé sortant vers des
//!   destinations EXTERNES** (IP publiques) pour un même poste sur 1 heure, et
//!   on alerte au-dessus d'un seuil.
//!
//! SEUIL — JUSTIFICATION
//!   500 Mo cumulés vers l'externe / poste / heure. Un poste bureautique envoie
//!   quelques Ko-Mo vers Internet (mails, navigation) ; 500 Mo/h vers des IP
//!   publiques est anormal et mérite investigation.
//!
//! AVANT (version faible) : `if bytes > 100 Mo (transfert unique) -> alerte`
//!   -> déclenchait sur n'importe quelle sauvegarde/visio/upload légitime.
//!
//! FAUX POSITIFS & LIMITES
//!   - Sauvegarde cloud légitime (Backup SaaS) -> mettre l'IP en liste blanche.
//!   - Gros envoi ponctuel (WeTransfer) -> le seuil horaire limite le bruit.
//!   - Idéalement, affiner avec une base de comportement par utilisateur.
//!
//! SÉVÉRITÉ : high.
//! ============================================================================

use crate::services::rule_context::RuleContext;

/// Seuil de volume cumulé sortant vers l'externe (octets) : 500 Mo.
const BYTES_THRESHOLD: i64 = 500_000_000;
/// Fenêtre d'agrégation (minutes).
const WINDOW_MIN: i64 = 60;

pub async fn check_data_exfiltration(ctx: &RuleContext<'_>) {

    // 1. Télémétrie : un flux réseau sortant (proxy/pare-feu).
    if !ctx.is_event("network_flow") {
        return;
    }

    // 2. Volume cumulé vers des destinations EXTERNES depuis ce poste sur la
    //    fenêtre (le helper SQL ne somme que le trafic vers IP publiques).
    let total = ctx.sum_external_bytes_out_source(WINDOW_MIN).await;

    if total < BYTES_THRESHOLD {
        return;
    }

    // 3. Alerte (dédupliquée sur la fenêtre).
    ctx.alert_once(
        "data_exfiltration",
        "high",
        &format!(
            "[MITRE T1048] Exfiltration probable : {} Mo sortis vers des \
             destinations externes depuis {} en {} min",
            total / 1_000_000,
            ctx.log.source_name,
            WINDOW_MIN,
        ),
        WINDOW_MIN,
    )
    .await;
}
