use sqlx::PgPool;
use tokio::sync::mpsc;

use crate::models::log::IncomingLog;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub sender: mpsc::Sender<IncomingLog>,
}