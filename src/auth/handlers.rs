use axum::{
    Json,
    http::StatusCode,
    extract::State,
};
use serde_json::{json, Value};
use uuid::Uuid;
use argon2::{self, Argon2, PasswordHasher, PasswordVerifier};
use argon2::password_hash::{SaltString, rand_core::OsRng, PasswordHash};
use sqlx::PgPool;

use crate::models::user::{RegisterPayload, LoginPayload, User};
use crate::auth::jwt::create_jwt;

pub async fn register_user(
    State(pool): State<PgPool>,
    Json(payload): Json<RegisterPayload>,
) -> Result<Json<User>, (StatusCode, String)> {
    let hashed_pwd = hash_password(&payload.password);
    let user_id = Uuid::new_v4();

    let user = sqlx::query_as!(
        User,
        r#"
        INSERT INTO users (id, email, password)
        VALUES ($1, $2, $3)
        RETURNING id, email, password, role, created_at
        "#,
        user_id,
        payload.email,
        hashed_pwd
    )
    .fetch_one(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(user))
}

pub async fn login_user(
    State(pool): State<PgPool>,
    Json(payload): Json<LoginPayload>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let record = sqlx::query_as!(
        User,
        r#"
        SELECT id, email, password, role, created_at
        FROM users
        WHERE email = $1
        "#,
        payload.email
    )
    .fetch_one(&pool)
    .await
    .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid email".to_string()))?;

    let is_valid = verify_password(&payload.password, &record.password);
    if !is_valid {
        return Err((StatusCode::UNAUTHORIZED, "Invalid password".to_string()));
    }

    let token = create_jwt(&record.id.to_string())
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("JWT Error: {}", e)))?;

    Ok(Json(json!({
        "token": token,
        "user": {
            "id": record.id.to_string(),
            "email": record.email,
            "role": record.role
        }
    })))
}

fn hash_password(password: &str) -> String {
    let salt = SaltString::generate(&mut OsRng);
    Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .unwrap()
        .to_string()
}

fn verify_password(password: &str, hashed: &str) -> bool {
    let parsed_hash = PasswordHash::new(hashed).unwrap();
    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok()
}
