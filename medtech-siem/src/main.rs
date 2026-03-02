mod models;
mod handlers;
mod state;
mod rules;

use axum::{routing::{get, post}, Router};
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use state::SharedLogs;

#[tokio::main]
async fn main() {
    let logs: SharedLogs = Arc::new(Mutex::new(Vec::new()));

    let app = Router::new()
        .route("/logs", post(handlers::receive_log))
        .route("/logs", get(handlers::get_logs))
        .with_state(logs);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Serveur lancé sur http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .unwrap();

    axum::serve(listener, app)
        .await
        .unwrap();
}