use axum::{extract::State, Json};
use argon2::{
    password_hash::{
        rand_core::OsRng,
        SaltString
    },
    Argon2,
    PasswordHasher,
};

use uuid::Uuid;

use crate::state::AppState;
use crate::models::user::RegisterRequest;

pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> Json<String> {

    let salt = SaltString::generate(&mut OsRng);

    let password_hash = Argon2::default()
        .hash_password(
            payload.password.as_bytes(),
            &salt
        )
        .unwrap()
        .to_string();

    sqlx::query(
        r#"
        INSERT INTO users
        (id, username, password_hash, role)
        VALUES ($1,$2,$3,$4)
        "#
    )
    .bind(Uuid::new_v4())
    .bind(payload.username)
    .bind(password_hash)
    .bind("admin")
    .execute(&state.db)
    .await
    .unwrap();

    Json("user created".to_string())
}