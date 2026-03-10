mod handlers;
mod rules;
mod db;
mod models;
mod services;
mod parsers;
mod state;

use axum::{
    Router,
    routing::{get, post}
};

use db::connection::connect_db;
use state::AppState;

#[tokio::main]
async fn main() {

    let db_pool = connect_db().await;

    let state = AppState { db: db_pool };

    let app = Router::new()
        .route("/logs", post(handlers::ingest::receive_log))
        .route("/logs", get(handlers::logs::get_logs))
        .route("/alerts", get(handlers::alerts::get_alerts))
        .with_state(state);

    let listener =
        tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}