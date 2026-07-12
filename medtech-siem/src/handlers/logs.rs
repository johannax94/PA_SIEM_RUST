use axum::{extract::State, Json};
use sqlx::query_as;
use axum::extract::Query;
use crate::state::AppState;
use crate::models::log::LogEntry;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct LogFilters {
    pub q: Option<String>,
    pub severity: Option<String>,
}

pub async fn get_logs(
    State(state): State<AppState>,
    Query(filters): Query<LogFilters>,
) -> Json<Vec<LogEntry>> {

    // Recherche par sous-chaîne (ILIKE) et non full-text : "power" trouve
    // "powershell", "192.168" trouve l'IP complète. Chaque terme (séparé par
    // des espaces) doit matcher quelque part dans le log (ET logique).
    let logs = query_as::<_, LogEntry>(
        r#"
        SELECT *
        FROM logs
        WHERE
        ($1 IS NULL OR COALESCE((
            SELECT bool_and(
                concat_ws(' ',
                    source_name,
                    COALESCE(vendor, ''),
                    COALESCE(hostname, ''),
                    COALESCE(username, ''),
                    COALESCE(ip_address, ''),
                    event_type,
                    severity,
                    message,
                    raw_log::text
                ) ILIKE '%' || term || '%'
            )
            FROM unnest(string_to_array(trim($1), ' ')) AS term
        ), TRUE))
        AND
        ($2 IS NULL OR severity = $2)
        ORDER BY created_at DESC
        LIMIT 100
        "#
    )
    .bind(filters.q)
    .bind(filters.severity)
    .fetch_all(&state.db)
    .await
    .expect("Erreur SQL logs");

    Json(logs)
}