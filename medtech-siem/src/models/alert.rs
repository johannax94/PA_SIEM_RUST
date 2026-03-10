use serde::Serialize;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use sqlx::FromRow;

#[derive(Serialize, FromRow)]
pub struct Alert {
    pub id: Uuid,
    pub rule_name: String,
    pub severity: String,
    pub description: String,
    pub source_name: Option<String>,
    pub timestamp: DateTime<Utc>,
}