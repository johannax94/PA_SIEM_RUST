use axum::{extract::State, Json};
use serde::Serialize;
use axum::extract::Extension;
use crate::auth::middleware::Claims;

use axum::extract::Path;
use crate::services::audit;


use crate::state::AppState;

#[derive(Serialize, sqlx::FromRow)]
pub struct UserDto {
    pub username: String,
    pub role: String,
}

use serde::Deserialize;

#[derive(Deserialize)]
pub struct UpdateRoleRequest {
    pub role: String,
}

pub async fn update_role(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(username): Path<String>,
    Json(payload): Json<UpdateRoleRequest>,
) -> Json<String> {

    if claims.role != "admin" {
        return Json(
            "forbidden".to_string()
        );
    }

    let user = sqlx::query_as::<_, crate::models::user::User>(
        r#"
        SELECT *
        FROM users
        WHERE username = $1
        "#
    )
    .bind(&username)
    .fetch_one(&state.db)
    .await
    .unwrap();

    let old_role = user.role.clone();

    if user.role == "admin"
        && payload.role != "admin"
    {
        let admin_count: (i64,) =
            sqlx::query_as(
                r#"
                SELECT COUNT(*)
                FROM users
                WHERE role = 'admin'
                "#
            )
            .fetch_one(&state.db)
            .await
            .unwrap();

        if admin_count.0 <= 1 {
            return Json(
                "cannot modify last admin".to_string()
            );
        }
    }

    sqlx::query(
        r#"
        UPDATE users
        SET role = $1
        WHERE username = $2
        "#
    )
    .bind(&payload.role)
    .bind(&username)
    .execute(&state.db)
    .await
    .unwrap();

    audit::log_admin_action(
        &state.db,
        "role_changed",
        format!(
            "{} changed role of {} from {} to {}",
            claims.sub,
            username,
            old_role,
            payload.role
        )
    )
    .await;

    Json(
        "role updated".to_string()
    )
}

pub async fn get_users(
    State(state): State<AppState>,
) -> Json<Vec<UserDto>> {

    let users =
        sqlx::query_as::<_, UserDto>(
            r#"
            SELECT
                username,
                role
            FROM users
            ORDER BY username
            "#
        )
        .fetch_all(&state.db)
        .await
        .unwrap();

    Json(users)
}

pub async fn delete_user(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(username): Path<String>,
) -> Json<String> {

    if claims.role != "admin" {
        return Json(
            "forbidden".to_string()
        );
    }
    if claims.sub == username {
        return Json(
            "cannot delete yourself".to_string()
        );
    }

    let user = sqlx::query_as::<_, crate::models::user::User>(
        r#"
        SELECT *
        FROM users
        WHERE username = $1
        "#
    )
    .bind(&username)
    .fetch_one(&state.db)
    .await
    .unwrap();

    if user.role == "admin" {

        let admin_count: (i64,) =
            sqlx::query_as(
                r#"
                SELECT COUNT(*)
                FROM users
                WHERE role = 'admin'
                "#
            )
            .fetch_one(&state.db)
            .await
            .unwrap();

        if admin_count.0 <= 1 {
            return Json(
                "cannot delete last admin".to_string()
            );
        }
    }

    sqlx::query(
        r#"
        DELETE FROM users
        WHERE username = $1
        "#
    )
    .bind(&username)
    .execute(&state.db)
    .await
    .unwrap();

    audit::log_admin_action(
        &state.db,
        "user_deleted",
        format!(
            "{} deleted user {}",
            claims.sub,
            username
        )
    )
    .await;
    Json(
        "user deleted".to_string()
    )
}

pub async fn create_user(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<CreateUserRequest>,
) -> Json<String> {

    // RBAC
    if claims.role != "admin" {
        return Json(
            "forbidden".to_string()
        );
    }

    // Validation
    if payload.username.trim().is_empty() {
        return Json(
            "username required".to_string()
        );
    }

    if payload.password.trim().is_empty() {
        return Json(
            "password required".to_string()
        );
    }

    // Hash mot de passe
    let salt =
        SaltString::generate(&mut OsRng);

    let password_hash =
        Argon2::default()
            .hash_password(
                payload.password.as_bytes(),
                &salt,
            )
            .unwrap()
            .to_string();

    // Insertion utilisateur
    let result = sqlx::query(
        r#"
        INSERT INTO users
        (
            id,
            username,
            password_hash,
            role
        )
        VALUES ($1,$2,$3,$4)
        "#
    )
    .bind(Uuid::new_v4())
    .bind(&payload.username)
    .bind(password_hash)
    .bind(&payload.role)
    .execute(&state.db)
    .await;

   match result {

        Ok(_) => {

            audit::log_admin_action(
                &state.db,
                "user_created",
                format!(
                    "{} created user {}",
                    claims.sub,
                    payload.username
                )
            )
            .await;

            Json(
                "user created".to_string()
            )
        }

        Err(e) => {

            println!(
                "Erreur création user : {:?}",
                e
            );

            Json(
                "user already exists".to_string()
            )
        }
    }
}

use argon2::{
    Argon2,
    PasswordHasher,
};

use argon2::password_hash::{
    SaltString,
    rand_core::OsRng,
};

use uuid::Uuid;

use crate::models::create_user::CreateUserRequest;