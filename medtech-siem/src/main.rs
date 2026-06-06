use axum::{
    Router,
    routing::{get, post}
};

use tokio::sync::mpsc;

use crate::db::connection::connect_db;
use crate::state::AppState;
use crate::services::ingestion::ingestion_worker;
use crate::handlers::alerts;
use tower_http::cors::CorsLayer;



mod handlers;
mod rules;
mod db;
mod models;
mod services;
mod parsers;
mod state;

#[tokio::main]
async fn main() {

    let (tx, rx) = mpsc::channel(10000);
    let cors = CorsLayer::permissive();

    let db_pool = connect_db().await;

    let state = AppState {
        db: db_pool.clone(),
        sender: tx,
    };

    tokio::spawn(async move {
        ingestion_worker(rx, db_pool).await;
    });

    let app = Router::new()
        .route("/logs", post(handlers::ingest::receive_log))
        .route("/logs", get(handlers::logs::get_logs))
        .route("/alerts", get(alerts::get_alerts))
        .route("/register", post(auth::handlers::register))
        .layer(cors)
        .with_state(state);

    let listener =
        tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}