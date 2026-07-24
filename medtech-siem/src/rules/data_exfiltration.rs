//! RÈGLE : Exfiltration de données vers l'externe — MITRE T1048
//! 500 Mo cumulés sortant vers des IP externes / poste / heure. Sévérité high.
//! Doc détaillée (menace, télémétrie, faux positifs) : voir soutenance_ideas (racine du repo).

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
