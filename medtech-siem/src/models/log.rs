use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use sqlx::FromRow;

#[derive(Deserialize)]
pub struct IncomingLog {
    pub source_type: String,
    pub source_name: String,
    pub level: String,
    pub event_type: String,
    pub message: String,
    pub metadata: Option<Value>,
}

#[derive(Serialize, FromRow)]
pub struct LogEntry {
    pub id: Uuid,
    pub source_type: String,
    pub source_name: String,
    pub level: String,
    pub event_type: String,
    pub message: String,
    pub metadata: Option<Value>,
    pub timestamp: DateTime<Utc>,
}