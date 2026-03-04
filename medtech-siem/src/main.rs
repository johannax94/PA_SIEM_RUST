mod models;
mod handlers;
mod state;
mod rules;
mod db;

use axum::{routing::{get, post}, Router};
use std::net::SocketAddr;
use state::AppState;

#[tokio::main]
async fn main() {
    let db_pool = db::connect_db().await;

    let state = AppState { db: db_pool };

    let app = Router::new()
        .route("/logs", post(handlers::receive_log))
        .route("/logs", get(handlers::get_logs))
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Serveur lancé sur http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .unwrap();

    axum::serve(listener, app)
        .await
        .unwrap();
}