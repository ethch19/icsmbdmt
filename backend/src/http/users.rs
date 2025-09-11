use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};
use axum::http::StatusCode;
use axum::{
    extract::{Json, Query, State},
    response::{IntoResponse, Response},
    routing::post,
    Router,
};
use once_cell::sync::Lazy;
use regex::Regex;
use serde::{Deserialize, Serialize};
use validator::Validate;
use sqlx::Row;  // Add this import

use crate::http::defaults::{default_time, default_uuid};
use crate::{Error, Result};

static USERNAME_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[0-9A-Za-z_]+$").unwrap());
static PASSWORD_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^.(.*[A-Za-z0-9])(.*\d).+$").unwrap());
static NAME_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[A-Za-z]+$").unwrap());

#[derive(Debug, Validate, Deserialize, Serialize)]
pub struct PendingUser {
    #[serde(default = "default_uuid")]
    pub id: uuid::Uuid,
    #[serde(default = "default_uuid")]
    pub verification_token: uuid::Uuid,
    #[validate(length(min=1, max=20), regex(path = *NAME_REGEX))]
    pub first_name: String,
    #[validate(length(min=1, max=20), regex(path = *NAME_REGEX))]
    pub surname: String,
    pub shortcode: String,
    pub cid: String,
    #[validate(length(min=8, max=32), regex(path = *PASSWORD_REGEX))]
    pub password: String,
    #[serde(default = "default_time")]
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Validate, Deserialize, Serialize)]
pub struct User {
    #[serde(default = "default_uuid")]
    pub id: uuid::Uuid,
    #[validate(length(min=1, max=20), regex(path = *NAME_REGEX))]
    pub first_name: String,
    #[validate(length(min=1, max=20), regex(path = *NAME_REGEX))]
    pub surname: String,
    pub shortcode: String,
    pub cid: String,
    #[validate(length(min=8, max=32), regex(path = *PASSWORD_REGEX))]
    pub password: String,
    pub admin: bool,
    pub tier: i16,
    pub jti: Option<uuid::Uuid>,
    #[serde(default = "default_time")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub last_login: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Validate, Deserialize, Serialize)]
pub struct UserAuth {
    pub shortcode: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct VerificationToken {
    pub token: uuid::Uuid,
}

pub fn router() -> Router<sqlx::PgPool> {
    Router::new()
        .route("/register", post(PendingUser::create))
        .route("/verify", post(PendingUser::verify))
}

// Simplified function - just return a default tier for testing
async fn check_tier(_pool: &sqlx::PgPool, _cid: &str, _shortcode: &str) -> Result<i16> {
    // For testing, just return tier 1 (Member)
    // In production, this would check against the eActivities API
    Ok(1)
}

// Simplified get_members function for testing
pub async fn get_members(_pool: &sqlx::PgPool) -> Result<()> {
    // For testing, we'll skip the eActivities API integration
    // In production, this would fetch from the API
    println!("Skipping eActivities API sync in test mode");
    Ok(())
}

impl PendingUser {
    pub async fn create(
        State(pool): State<sqlx::PgPool>,
        Json(req): Json<PendingUser>,
    ) -> Result<Response> {
        req.validate()?;

        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password_hash = argon2
            .hash_password(req.password.as_bytes(), &salt)?
            .to_string();

        let tier = check_tier(&pool, &req.cid, &req.shortcode).await?;

        if tier != 0 {
            // Direct registration for members/team members
            sqlx::query(
                "INSERT INTO auth.users (id, first_name, surname, shortcode, cid, password, admin, tier, created_at) VALUES($1, $2, $3, $4, $5, $6, $7, $8, $9)"
            )
            .bind(req.id)
            .bind(&req.first_name)
            .bind(&req.surname)
            .bind(&req.shortcode)
            .bind(&req.cid)
            .bind(&password_hash)
            .bind(false)
            .bind(tier)
            .bind(req.created_at)
            .execute(&pool)
            .await?;
            
            Ok(StatusCode::CREATED.into_response())
        } else {
            // Pending verification for non-members
            sqlx::query(
                "INSERT INTO auth.pending_users(id, verification_token, first_name, surname, shortcode, cid, password, created_at) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)"
            )
            .bind(req.id)
            .bind(req.verification_token)
            .bind(&req.first_name)
            .bind(&req.surname)
            .bind(&req.shortcode)
            .bind(&req.cid)
            .bind(&password_hash)
            .bind(req.created_at)
            .execute(&pool)
            .await?;
            
            // NEED TO IMPLEMENT EMAIL VERIFICATION
            Ok((StatusCode::CREATED, req.verification_token.to_string()).into_response())
        }
    }

    pub async fn verify(
        State(pool): State<sqlx::PgPool>,
        Query(token): Query<VerificationToken>,
    ) -> Result<StatusCode> {
        let pending_user_row = sqlx::query(
            "SELECT id, verification_token, first_name, surname, shortcode, cid, password, created_at FROM auth.pending_users WHERE verification_token = $1"
        )
        .bind(token.token)
        .fetch_optional(&pool)
        .await?;

        if let Some(row) = pending_user_row {
            let user_id: uuid::Uuid = row.get("id");
            let first_name: String = row.get("first_name");
            let surname: String = row.get("surname");
            let shortcode: String = row.get("shortcode");
            let cid: String = row.get("cid");
            let password: String = row.get("password");

            let tier = check_tier(&pool, &cid, &shortcode).await?;
            
            // Move user from pending to active
            sqlx::query("DELETE FROM auth.pending_users WHERE verification_token = $1")
                .bind(token.token)
                .execute(&pool)
                .await?;

            sqlx::query(
                "INSERT INTO auth.users(id, first_name, surname, shortcode, cid, password, admin, tier) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)"
            )
            .bind(user_id)
            .bind(first_name)
            .bind(surname)
            .bind(shortcode)
            .bind(cid)
            .bind(password)
            .bind(false)
            .bind(tier)
            .execute(&pool)
            .await?;

            return Ok(StatusCode::CREATED);
        }
        Err(Error::UnprocessableEntity("Invalid Token".into()))
    }
}