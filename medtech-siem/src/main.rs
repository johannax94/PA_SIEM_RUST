mod handlers;
mod rules;
mod db;
mod models;
mod services;
mod parsers;
mod state;
mod auth;

use axum::{

    Router,
    routing::{get, post, delete, put}
};

use tokio::sync::mpsc;

use crate::db::connection::connect_db;
use crate::state::AppState;
use crate::services::ingestion::ingestion_worker;
use crate::handlers::{alerts, auth_handler};
use axum::middleware;
use crate::auth::middleware::auth_middleware;
use tower_http::cors::CorsLayer;



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

    .route(
        "/logs",
        post(handlers::ingest::receive_log)
    )

    .route(
        "/logs",
        get(handlers::logs::get_logs)
            .route_layer(
                middleware::from_fn(auth_middleware)
            )
    )

    .route(
        "/alerts",
        get(alerts::get_alerts)
            .route_layer(
                middleware::from_fn(auth_middleware)
            )
    )

    .route(
        "/auth/register",
        post(auth_handler::register)
    )

    .route(
        "/auth/login",
        post(auth_handler::login)
    )
    .route(
        "/dashboard",
        get(handlers::dashboard::get_dashboard)
    )
    .route(
        "/users",
        get(handlers::users::get_users)
            .route_layer(
                middleware::from_fn(auth_middleware)
            )
    )
    .route(
        "/users/:username",
        delete(handlers::users::delete_user)
            .route_layer(
                middleware::from_fn(auth_middleware)
            )
    )
    .route(
        "/users/:username/role",
        put(handlers::users::update_role)
            .route_layer(
                middleware::from_fn(auth_middleware)
            )
    )
    .route(
        "/users",
        post(handlers::users::create_user)
            .route_layer(
                middleware::from_fn(auth_middleware)
            )
    )
        .layer(cors)
    .with_state(state);
    let listener =
        tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}