use axum::{extract::State, Json};
use serde::Serialize;

use crate::services::risk;
use crate::state::AppState;

#[derive(Serialize)]
pub struct RiskEntity {
    pub entity: String,
    pub risk_score: i32,
    pub tier: String,
    pub distinct_rules: i64,
    pub rules: String,
    pub last_seen: chrono::NaiveDateTime,
}

#[derive(Serialize, sqlx::FromRow)]
pub struct RiskIncident {
    pub id: uuid::Uuid,
    pub entity: String,
    pub risk_score: i32,
    pub severity: String,
    pub status: String,
    pub rules_involved: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Serialize)]
pub struct RiskResponse {
    pub entities: Vec<RiskEntity>,
    pub incidents: Vec<RiskIncident>,
}

pub async fn get_risk(
    State(state): State<AppState>,
) -> Json<RiskResponse> {

    let entities = risk::entities_at_risk(&state.db)
        .await
        .into_iter()
        .map(|(entity, score, distinct_rules, rules, last_seen)| RiskEntity {
            entity,
            risk_score: score.round() as i32,
            tier: risk::tier_label(score).to_string(),
            distinct_rules,
            rules,
            last_seen,
        })
        .collect();

    let incidents = sqlx::query_as::<_, RiskIncident>(
        r#"
        SELECT id, entity, risk_score, severity, status, rules_involved,
               created_at, updated_at
        FROM risk_incidents
        ORDER BY updated_at DESC
        LIMIT 50
        "#,
    )
    .fetch_all(&state.db)
    .await
    .unwrap_or_default();

    Json(RiskResponse { entities, incidents })
}
