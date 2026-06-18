#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    role: String,
    exp: usize,
}


use axum::{
    extract::State,
    Json,
};

use argon2::{
    Argon2,
    PasswordHasher,
    PasswordVerifier,
    PasswordHash,
};

use argon2::password_hash::{
    SaltString,
    rand_core::OsRng,
};
use jsonwebtoken::{
    encode,
    EncodingKey,
    Header,
};

use serde::{Serialize, Deserialize};

use crate::models::auth::{
    RegisterRequest,
    LoginRequest,
    AuthResponse,
};
use uuid::Uuid;

use crate::state::AppState;


pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> Json<String> {

    let salt = SaltString::generate(&mut OsRng);

    let password_hash = Argon2::default()
        .hash_password(
            payload.password.as_bytes(),
            &salt,
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

pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Json<AuthResponse> {

    let user = sqlx::query_as::<_, crate::models::user::User>(
        r#"
        SELECT *
        FROM users
        WHERE username = $1
        "#
    )
    .bind(&payload.username)
    .fetch_one(&state.db)
    .await
    .unwrap();

    let parsed_hash =
        PasswordHash::new(&user.password_hash)
        .unwrap();

    let is_valid = Argon2::default()
        .verify_password(
            payload.password.as_bytes(),
            &parsed_hash,
        )
        .is_ok();

    if !is_valid {
        panic!("invalid password");
    }

    let role = user.role.clone();

    let claims = Claims {
        sub: user.username,
        role: role.clone(),
        exp: 2000000000,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(
            "super-secret-key".as_ref()
        ),
    )
    .unwrap();

    Json(AuthResponse {
        token,
        role,
    })
}