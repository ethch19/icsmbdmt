// backend/src/http/users.rs - Fixed version
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString, PasswordHash, PasswordVerifier},
    Argon2,
};
use axum::http::StatusCode;
use axum::{
    extract::{Json, Query, State, Extension, Path},
    response::{IntoResponse, Response},
    routing::{post, get, patch, delete},
    Router,
};
use once_cell::sync::Lazy;
use regex::Regex;
use serde::{Deserialize, Serialize};
use validator::Validate;
use sqlx::Row;

use crate::http::defaults::{default_time, default_uuid};
use crate::http::token::AccessClaims;
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

// NEW: Profile management structs
#[derive(Debug, Serialize)]
pub struct UserProfile {
    pub id: uuid::Uuid,
    pub first_name: String,
    pub surname: String,
    pub shortcode: String,
    pub cid: String,
    pub admin: bool,
    pub tier: i16,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub last_login: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateProfileRequest {
    #[validate(length(min=1, max=30), regex(path = *NAME_REGEX))]
    pub first_name: String,
    #[validate(length(min=1, max=30), regex(path = *NAME_REGEX))]
    pub surname: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdatePasswordRequest {
    pub current_password: String,
    #[validate(length(min=8, max=32), regex(path = *PASSWORD_REGEX))]
    pub new_password: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTierRequest {
    pub tier: i16,
}

#[derive(Debug, Deserialize)]
pub struct UpdateAdminRequest {
    pub admin: bool,
}

pub fn router() -> Router<sqlx::PgPool> {
    Router::new()
        .route("/register", post(PendingUser::create))
        .route("/verify", post(PendingUser::verify))
        .route("/profile", get(get_profile))
        .route("/profile", patch(update_profile))
        .route("/profile", delete(delete_profile))
        .route("/password", patch(update_password))
}

// Admin router (separate function)
pub fn admin_router() -> Router<sqlx::PgPool> {
    Router::new()
        .route("/users", get(list_all_users))
        .route("/users/:user_id/tier", patch(update_user_tier))
        .route("/users/:user_id/admin", patch(update_user_admin))
        .route("/users/:user_id", delete(delete_user))
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

// Get current user's profile
async fn get_profile(
    State(pool): State<sqlx::PgPool>,
    Extension(claims): Extension<AccessClaims>,
) -> Result<Json<UserProfile>> {
    let row = sqlx::query(
        "SELECT id, first_name, surname, shortcode, cid, admin, tier, created_at, last_login 
         FROM auth.users WHERE id = $1"
    )
    .bind(claims.user_id)
    .fetch_optional(&pool)
    .await?;

    if let Some(row) = row {
        Ok(Json(UserProfile {
            id: row.get("id"),
            first_name: row.get("first_name"),
            surname: row.get("surname"),
            shortcode: row.get("shortcode"),
            cid: row.get("cid"),
            admin: row.get("admin"),
            tier: row.get("tier"),
            created_at: row.get("created_at"),
            last_login: row.get("last_login"),
        }))
    } else {
        Err(Error::UnprocessableEntity("User not found".to_string()))
    }
}

// Update user's profile
async fn update_profile(
    State(pool): State<sqlx::PgPool>,
    Extension(claims): Extension<AccessClaims>,
    Json(req): Json<UpdateProfileRequest>,
) -> Result<Json<UserProfile>> {
    req.validate()?;

    let updated_at = chrono::Utc::now();
    
    sqlx::query(
        "UPDATE auth.users SET first_name = $1, surname = $2, last_login = $3 WHERE id = $4"
    )
    .bind(&req.first_name)
    .bind(&req.surname)
    .bind(updated_at)
    .bind(claims.user_id)
    .execute(&pool)
    .await?;

    // Return updated profile
    get_profile(State(pool), Extension(claims)).await
}

// Update user's password
async fn update_password(
    State(pool): State<sqlx::PgPool>,
    Extension(claims): Extension<AccessClaims>,
    Json(req): Json<UpdatePasswordRequest>,
) -> Result<StatusCode> {
    req.validate()?;

    // Get current password hash
    let current_hash: String = sqlx::query_scalar(
        "SELECT password FROM auth.users WHERE id = $1"
    )
    .bind(claims.user_id)
    .fetch_one(&pool)
    .await?;

    // Verify current password
    let parsed_hash = PasswordHash::new(&current_hash)
        .map_err(|_| Error::UnprocessableEntity("Invalid current password".to_string()))?;
    
    if Argon2::default()
        .verify_password(req.current_password.as_bytes(), &parsed_hash)
        .is_err()
    {
        return Err(Error::UnprocessableEntity("Current password is incorrect".to_string()));
    }

    // Hash new password
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let new_password_hash = argon2
        .hash_password(req.new_password.as_bytes(), &salt)?
        .to_string();

    // Update password and invalidate refresh tokens
    sqlx::query(
        "UPDATE auth.users SET password = $1, jti = NULL WHERE id = $2"
    )
    .bind(new_password_hash)
    .bind(claims.user_id)
    .execute(&pool)
    .await?;

    Ok(StatusCode::NO_CONTENT)
}

// Delete user's account
async fn delete_profile(
    State(pool): State<sqlx::PgPool>,
    Extension(claims): Extension<AccessClaims>,
) -> Result<StatusCode> {
    sqlx::query("DELETE FROM auth.users WHERE id = $1")
        .bind(claims.user_id)
        .execute(&pool)
        .await?;

    Ok(StatusCode::NO_CONTENT)
}

// Admin: List all users
async fn list_all_users(
    State(pool): State<sqlx::PgPool>,
    Extension(claims): Extension<AccessClaims>,
) -> Result<Json<Vec<UserProfile>>> {
    if !claims.admin {
        return Err(Error::UnprocessableEntity("Admin access required".to_string()));
    }

    let rows = sqlx::query(
        "SELECT id, first_name, surname, shortcode, cid, admin, tier, created_at, last_login 
         FROM auth.users ORDER BY created_at DESC"
    )
    .fetch_all(&pool)
    .await?;

    let users: Vec<UserProfile> = rows.into_iter().map(|row| {
        UserProfile {
            id: row.get("id"),
            first_name: row.get("first_name"),
            surname: row.get("surname"),
            shortcode: row.get("shortcode"),
            cid: row.get("cid"),
            admin: row.get("admin"),
            tier: row.get("tier"),
            created_at: row.get("created_at"),
            last_login: row.get("last_login"),
        }
    }).collect();

    Ok(Json(users))
}

// Admin: Update user tier
async fn update_user_tier(
    State(pool): State<sqlx::PgPool>,
    Extension(claims): Extension<AccessClaims>,
    Path(user_id): Path<uuid::Uuid>,
    Json(req): Json<UpdateTierRequest>,
) -> Result<StatusCode> {
    if !claims.admin {
        return Err(Error::UnprocessableEntity("Admin access required".to_string()));
    }

    if req.tier < 0 || req.tier > 2 {
        return Err(Error::UnprocessableEntity("Invalid tier value".to_string()));
    }

    let result = sqlx::query(
        "UPDATE auth.users SET tier = $1 WHERE id = $2"
    )
    .bind(req.tier)
    .bind(user_id)
    .execute(&pool)
    .await?;

    if result.rows_affected() == 0 {
        return Err(Error::UnprocessableEntity("User not found".to_string()));
    }

    Ok(StatusCode::NO_CONTENT)
}

// Admin: Update user admin status
async fn update_user_admin(
    State(pool): State<sqlx::PgPool>,
    Extension(claims): Extension<AccessClaims>,
    Path(user_id): Path<uuid::Uuid>,
    Json(req): Json<UpdateAdminRequest>,
) -> Result<StatusCode> {
    if !claims.admin {
        return Err(Error::UnprocessableEntity("Admin access required".to_string()));
    }

    // Don't allow users to remove their own admin status
    if user_id == claims.user_id && !req.admin {
        return Err(Error::UnprocessableEntity("Cannot remove your own admin privileges".to_string()));
    }

    let result = sqlx::query(
        "UPDATE auth.users SET admin = $1 WHERE id = $2"
    )
    .bind(req.admin)
    .bind(user_id)
    .execute(&pool)
    .await?;

    if result.rows_affected() == 0 {
        return Err(Error::UnprocessableEntity("User not found".to_string()));
    }

    Ok(StatusCode::NO_CONTENT)
}

// Admin: Delete user
async fn delete_user(
    State(pool): State<sqlx::PgPool>,
    Extension(claims): Extension<AccessClaims>,
    Path(user_id): Path<uuid::Uuid>,
) -> Result<StatusCode> {
    if !claims.admin {
        return Err(Error::UnprocessableEntity("Admin access required".to_string()));
    }

    // Don't allow users to delete themselves
    if user_id == claims.user_id {
        return Err(Error::UnprocessableEntity("Cannot delete your own account".to_string()));
    }

    let result = sqlx::query("DELETE FROM auth.users WHERE id = $1")
        .bind(user_id)
        .execute(&pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(Error::UnprocessableEntity("User not found".to_string()));
    }

    Ok(StatusCode::NO_CONTENT)
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