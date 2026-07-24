//! RÈGLE : Scan de ports (reconnaissance) — MITRE T1595.001 (ext) / T1046 (int)
//! 20 ports DISTINCTS / 2 min depuis une même IP. high (externe) / medium (interne).
//! Doc détaillée (menace, télémétrie, faux positifs) : voir soutenance_ideas (racine du repo).

use crate::services::rule_context::RuleContext;

/// Nombre de ports distincts à partir duquel on considère un scan.
const DISTINCT_PORTS_THRESHOLD: i64 = 20;
/// Fenêtre d'observation (minutes).
const WINDOW_MIN: i64 = 2;

pub async fn check_network_scan(ctx: &RuleContext<'_>) {

    // 1. Télémétrie : une connexion bloquée par le pare-feu.
    if !ctx.is_event("blocked_connection") {
        return;
    }

    // 2. Signal de balayage : assez de ports DISTINCTS depuis cette IP ?
    if ctx.count_distinct_ports_ip(WINDOW_MIN).await < DISTINCT_PORTS_THRESHOLD {
        return;
    }

    // 3. Sévérité + MITRE selon l'origine (externe = recon Internet).
    let external = ctx.is_source_ip_external();
    let severity = if external { "high" } else { "medium" };
    let mitre = if external { "T1595.001" } else { "T1046" };

    ctx.alert_once(
        "network_scan",
        severity,
        &format!(
            "[MITRE {}] Scan de ports probable : l'IP {} a sondé {}+ ports distincts en {} min ({})",
            mitre,
            ctx.log.ip_address.as_deref().unwrap_or("?"),
            DISTINCT_PORTS_THRESHOLD,
            WINDOW_MIN,
            if external { "source externe" } else { "source interne" },
        ),
        WINDOW_MIN,
    )
    .await;
}
