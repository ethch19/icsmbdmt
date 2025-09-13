use axum::{
    extract::{Path, State, Extension},
    http::StatusCode,
    routing::{get, post, delete, patch},  
    Json, Router,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;
use sqlx::Row; 

use crate::http::defaults::{default_time, default_uuid};
use crate::http::token::AccessClaims;
use crate::{Error, Result};

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct CreateSessionRequest {
    #[validate(length(min = 1, max = 50))]
    pub title: String,
    pub description: String,
    pub location: String,
    pub tier: i16,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub user_limit: Option<i16>,
}

#[derive(Debug, Serialize)]
pub struct SessionResponse {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub location: String,
    pub tier: i16,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub user_limit: Option<i16>,
    pub author_id: Uuid,
    pub created_at: DateTime<Utc>,
}

async fn get_session(
    State(pool): State<sqlx::PgPool>,
    Extension(claims): Extension<AccessClaims>,
    Path(id): Path<Uuid>,
) -> Result<Json<SessionResponse>> {
    let row = sqlx::query(
        r#"
        SELECT sf.id, sf.author_id, sf.title, sf.description, sf.location, sf.tier, 
               sf.start_time, sf.end_time, sf.user_limit, sf.created_at,
               u.first_name || ' ' || u.surname as author_name,
               COUNT(b.user_id) as current_bookings,
               EXISTS(SELECT 1 FROM records.bookings WHERE user_id = $2 AND form_id = sf.id) as is_booked
        FROM records.session_forms sf
        LEFT JOIN auth.users u ON sf.author_id = u.id
        LEFT JOIN records.bookings b ON sf.id = b.form_id
        WHERE sf.id = $1
        GROUP BY sf.id, u.first_name, u.surname
        "#
    )
    .bind(id)
    .bind(claims.user_id)
    .fetch_optional(&pool)
    .await?;

    if let Some(row) = row {
        Ok(Json(SessionResponse {
            id: row.get("id"),
            author_id: row.get("author_id"),
            title: row.get("title"),
            description: row.get("description"),
            location: row.get("location"),
            tier: row.get("tier"),
            start_time: row.get("start_time"),
            end_time: row.get("end_time"),
            user_limit: row.get("user_limit"),
            created_at: row.get("created_at"),
        }))
    } else {
        Err(Error::UnprocessableEntity("Session not found".to_string()))
    }
}

// Update session
async fn update_session(
    State(pool): State<sqlx::PgPool>,
    Extension(claims): Extension<AccessClaims>,
    Path(id): Path<Uuid>,
    Json(payload): Json<CreateSessionRequest>,
) -> Result<Json<SessionResponse>> {
    payload.validate()?;

    let result = sqlx::query(
        r#"
        UPDATE records.session_forms 
        SET title = $3, description = $4, location = $5, tier = $6, 
            start_time = $7, end_time = $8, user_limit = $9
        WHERE id = $1 AND author_id = $2
        "#
    )
    .bind(id)
    .bind(claims.user_id)
    .bind(&payload.title)
    .bind(&payload.description)
    .bind(&payload.location)
    .bind(payload.tier)
    .bind(payload.start_time)
    .bind(payload.end_time)
    .bind(payload.user_limit)
    .execute(&pool)
    .await?;

    if result.rows_affected() == 0 {
        return Err(Error::UnprocessableEntity("Session not found or access denied".to_string()));
    }

    Ok(Json(SessionResponse {
        id,
        title: payload.title,
        description: payload.description,
        location: payload.location,
        tier: payload.tier,
        start_time: payload.start_time,
        end_time: payload.end_time,
        user_limit: payload.user_limit,
        author_id: claims.user_id,
        created_at: chrono::Utc::now(), // This should be fetched from DB
    }))
}

// Update your router in sessions.rs:
pub fn router() -> Router<sqlx::PgPool> {
    Router::new()
        .route("/", post(create_session))
        .route("/", get(list_sessions))
        .route("/:id", get(get_session))          // Add this
        .route("/:id", patch(update_session))     // Add this  
        .route("/:id", delete(delete_session))
}

async fn create_session(
    State(pool): State<sqlx::PgPool>,
    Extension(claims): Extension<AccessClaims>,
    Json(payload): Json<CreateSessionRequest>,
) -> Result<Json<SessionResponse>> {
    payload.validate()?;

    let session_id = default_uuid();
    let created_at = default_time();

    sqlx::query(
        r#"
        INSERT INTO records.session_forms 
        (id, author_id, title, description, location, tier, start_time, end_time, user_limit, created_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
        "#
    )
    .bind(session_id)
    .bind(claims.user_id)
    .bind(&payload.title)
    .bind(&payload.description)
    .bind(&payload.location)
    .bind(payload.tier)
    .bind(payload.start_time)
    .bind(payload.end_time)
    .bind(payload.user_limit)
    .bind(created_at)
    .execute(&pool)
    .await?;

    Ok(Json(SessionResponse {
        id: session_id,
        title: payload.title,
        description: payload.description,
        location: payload.location,
        tier: payload.tier,
        start_time: payload.start_time,
        end_time: payload.end_time,
        user_limit: payload.user_limit,
        author_id: claims.user_id,
        created_at,
    }))
}

async fn list_sessions(
    State(pool): State<sqlx::PgPool>,
    Extension(_claims): Extension<AccessClaims>,
) -> Result<Json<Vec<SessionResponse>>> {
    let rows = sqlx::query(
        r#"
        SELECT id, author_id, title, description, location, tier, 
               start_time, end_time, user_limit, created_at
        FROM records.session_forms 
        ORDER BY start_time ASC
        LIMIT 50
        "#
    )
    .fetch_all(&pool)
    .await?;

    let sessions: Vec<SessionResponse> = rows.into_iter().map(|row| {
        SessionResponse {
            id: row.get("id"),
            author_id: row.get("author_id"),
            title: row.get("title"),
            description: row.get("description"),
            location: row.get("location"),
            tier: row.get("tier"),
            start_time: row.get("start_time"),
            end_time: row.get("end_time"),
            user_limit: row.get("user_limit"),
            created_at: row.get("created_at"),
        }
    }).collect();

    Ok(Json(sessions))
}

async fn delete_session(
    State(pool): State<sqlx::PgPool>,
    Extension(claims): Extension<AccessClaims>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode> {
    sqlx::query("DELETE FROM records.session_forms WHERE id = $1 AND author_id = $2")
        .bind(id)
        .bind(claims.user_id)
        .execute(&pool)
        .await?;

    Ok(StatusCode::NO_CONTENT)
}