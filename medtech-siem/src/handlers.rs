use axum::{extract::State, Json};
use chrono::Utc;
use crate::models::Log;
use crate::state::SharedLogs;
use crate::rules;

#[derive(serde::Deserialize)]
pub struct IncomingLog {
    pub source: String,
    pub message: String,
    pub level: String,
}

pub async fn receive_log(
    State(logs): State<SharedLogs>,
    Json(payload): Json<IncomingLog>,
) {
    let log = Log {
        source: payload.source,
        message: payload.message,
        level: payload.level,
        timestamp: Utc::now(),
    };

    println!("Log reçu : {:?}", log);

    let mut logs = logs.lock().unwrap();
    logs.push(log);

    if let Some(alert) = rules::check_bruteforce(&logs) {
    println!("{}", alert);
    
    }
}

pub async fn get_logs(
    State(logs): State<SharedLogs>,
) -> Json<Vec<Log>> {
    let logs = logs.lock().unwrap();
    Json(logs.clone())
}