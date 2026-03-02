use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Log {
    pub source: String,
    pub message: String,
    pub level: String,
    pub timestamp: DateTime<Utc>,
}