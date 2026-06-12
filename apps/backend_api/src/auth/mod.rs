pub mod middleware;

use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use argon2::password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString};
use argon2::Argon2;
use chrono::{Utc, Duration};
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use uuid::Uuid;

const ALLOWED_EMAILS: &[&str] = &["Ibestechub@gmail.com"];
const JWT_SECRET: &[u8] = b"techub-comms-jwt-secret-change-in-production";
const TOKEN_DURATION_HOURS: i64 = 24;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub email: String,
    pub exp: usize,
}

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
    pub display_name: Option<String>,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

pub fn hash_password(password: &str) -> Result<String, String> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| format!("Password hash error: {}", e))?
        .to_string();
    Ok(hash)
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, String> {
    let parsed_hash = PasswordHash::new(hash)
        .map_err(|e| format!("Parse hash error: {}", e))?;
    Ok(Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok())
}

pub fn generate_token(user_id: &str, email: &str) -> Result<String, String> {
    let exp = (Utc::now() + Duration::hours(TOKEN_DURATION_HOURS)).timestamp() as usize;
    let claims = Claims {
        sub: user_id.to_string(),
        email: email.to_string(),
        exp,
    };
    encode(&Header::default(), &claims, &EncodingKey::from_secret(JWT_SECRET))
        .map_err(|e| format!("Token error: {}", e))
}

pub fn validate_token(token: &str) -> Result<Claims, String> {
    decode::<Claims>(token, &DecodingKey::from_secret(JWT_SECRET), &Validation::default())
        .map(|data| data.claims)
        .map_err(|e| format!("Invalid token: {}", e))
}

pub fn is_email_allowed(email: &str) -> bool {
    ALLOWED_EMAILS.iter().any(|allowed| allowed.eq_ignore_ascii_case(email))
}

pub async fn register_user(pool: &PgPool, req: &RegisterRequest) -> Result<String, String> {
    let email = req.email.trim().to_lowercase();
    if !is_email_allowed(&email) {
        return Err("This email is not authorized to register".into());
    }

    let existing: Option<(Uuid,)> = sqlx::query_as("SELECT id FROM users WHERE email = $1")
        .bind(&email)
        .fetch_optional(pool)
        .await
        .map_err(|e| format!("DB error: {}", e))?;

    if existing.is_some() {
        return Err("Account already exists. Please login.".into());
    }

    let hash = hash_password(&req.password)?;
    let display = req.display_name.as_deref().unwrap_or("User");
    let id = Uuid::new_v4();

    sqlx::query("INSERT INTO users (id, email, password_hash, display_name) VALUES ($1, $2, $3, $4)")
        .bind(id)
        .bind(&email)
        .bind(&hash)
        .bind(display)
        .execute(pool)
        .await
        .map_err(|e| format!("DB insert error: {}", e))?;

    let token = generate_token(&id.to_string(), &email)?;
    store_session(pool, &id, &token).await?;
    Ok(token)
}

pub async fn login_user(pool: &PgPool, req: &LoginRequest) -> Result<String, String> {
    let email = req.email.trim().to_lowercase();

    let user: Option<(Uuid, String, String)> = sqlx::query_as(
        "SELECT id, password_hash, display_name FROM users WHERE email = $1"
    )
    .bind(&email)
    .fetch_optional(pool)
    .await
    .map_err(|e| format!("DB error: {}", e))?;

    let (user_id, hash, _name) = user.ok_or("Invalid email or password")?;

    if !verify_password(&req.password, &hash)? {
        return Err("Invalid email or password".into());
    }

    sqlx::query("UPDATE users SET last_login = NOW() WHERE id = $1")
        .bind(user_id)
        .execute(pool)
        .await
        .ok();

    let token = generate_token(&user_id.to_string(), &email)?;
    store_session(pool, &user_id, &token).await?;
    Ok(token)
}

async fn store_session(pool: &PgPool, user_id: &Uuid, token: &str) -> Result<(), String> {
    let expires = Utc::now() + Duration::hours(TOKEN_DURATION_HOURS);
    sqlx::query(
        "INSERT INTO auth_sessions (user_id, token, expires_at) VALUES ($1, $2, $3)"
    )
    .bind(user_id)
    .bind(token)
    .bind(expires)
    .execute(pool)
    .await
    .map_err(|e| format!("Session store error: {}", e))?;
    Ok(())
}

pub async fn get_user_from_token(pool: &PgPool, token: &str) -> Result<(Uuid, String, String), String> {
    let claims = validate_token(token)?;

    let session: Option<(Uuid,)> = sqlx::query_as(
        "SELECT user_id FROM auth_sessions WHERE token = $1 AND expires_at > NOW()"
    )
    .bind(token)
    .fetch_optional(pool)
    .await
    .map_err(|e| format!("DB error: {}", e))?;

    let _session = session.ok_or("Session expired")?;

    let user_id = Uuid::parse_str(&claims.sub).map_err(|_| "Invalid user ID")?;
    let user: Option<(String, String)> = sqlx::query_as(
        "SELECT email, display_name FROM users WHERE id = $1"
    )
    .bind(user_id)
    .fetch_optional(pool)
    .await
    .map_err(|e| format!("DB error: {}", e))?;

    let (email, name) = user.ok_or("User not found")?;
    Ok((user_id, email, name))
}

pub async fn logout(pool: &PgPool, token: &str) -> Result<(), String> {
    sqlx::query("DELETE FROM auth_sessions WHERE token = $1")
        .bind(token)
        .execute(pool)
        .await
        .map_err(|e| format!("Logout error: {}", e))?;
    Ok(())
}
