use std::sync::{Arc, Mutex};
use crate::models::Log;

pub type SharedLogs = Arc<Mutex<Vec<Log>>>;