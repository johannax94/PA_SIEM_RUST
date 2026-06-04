use serde::{Deserialize, Serialize};
use serde_json::Value;
use chrono::NaiveDateTime;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct IncomingLog {
    pub source_name: String,
    pub event_type: String,
    pub severity: String,
    pub message: String,
    pub raw_log: Value,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct LogEntry {
    pub id: uuid::Uuid,
    pub source_name: String,
    pub event_type: String,
    pub severity: String,
    pub message: String,
    pub raw_log: Value,
    pub created_at: NaiveDateTime,
}