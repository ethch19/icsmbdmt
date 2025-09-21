use axum::{
    extract::{Path, State, Extension},
    http::StatusCode,
    routing::{get, post, delete},
    Json, Router,
};
use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;
use sqlx::Row;

use crate::http::token::AccessClaims;
use crate::{Error, Result};

#[derive(Debug, Serialize)]
pub struct BookingResponse {
    pub user_id: Uuid,
    pub session_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub session_title: String,
    pub session_start_time: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct BookingUserResponse {
    pub user_id: Uuid,
    pub user_name: String,
    pub shortcode: String,
    pub tier: i16,
    pub created_at: DateTime<Utc>,
}

pub fn router() -> Router<sqlx::PgPool> {
    Router::new()
        .route("/", get(list_user_bookings))
        .route("/sessions/:session_id", post(book_session))
        .route("/sessions/:session_id", delete(cancel_booking))
        .route("/sessions/:session_id/bookings", get(list_session_bookings)) // NEW
}

async fn book_session(
    State(pool): State<sqlx::PgPool>,
    Extension(claims): Extension<AccessClaims>,
    Path(session_id): Path<Uuid>,
) -> Result<StatusCode> {
    // Simple check - does session exist?
    let session_exists = sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS(SELECT 1 FROM records.session_forms WHERE id = $1)"
    )
    .bind(session_id)
    .fetch_one(&pool)
    .await?;

    if !session_exists {
        return Err(Error::UnprocessableEntity("Session not found".to_string()));
    }

    // Check if already booked
    let already_booked = sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS(SELECT 1 FROM records.bookings WHERE user_id = $1 AND form_id = $2)"
    )
    .bind(claims.user_id)
    .bind(session_id)
    .fetch_one(&pool)
    .await?;

    if already_booked {
        return Err(Error::Conflict("Already booked for this session".to_string()));
    }

    // Create booking
    let created_at = chrono::Utc::now();
    sqlx::query(
        "INSERT INTO records.bookings (user_id, form_id, created_at) VALUES ($1, $2, $3)"
    )
    .bind(claims.user_id)
    .bind(session_id)
    .bind(created_at)
    .execute(&pool)
    .await?;

    Ok(StatusCode::CREATED)
}

async fn cancel_booking(
    State(pool): State<sqlx::PgPool>,
    Extension(claims): Extension<AccessClaims>,
    Path(session_id): Path<Uuid>,
) -> Result<StatusCode> {
    let result = sqlx::query(
        "DELETE FROM records.bookings WHERE user_id = $1 AND form_id = $2"
    )
    .bind(claims.user_id)
    .bind(session_id)
    .execute(&pool)
    .await?;

    if result.rows_affected() == 0 {
        return Err(Error::UnprocessableEntity("Booking not found".to_string()));
    }

    Ok(StatusCode::NO_CONTENT)
}

async fn list_user_bookings(
    State(pool): State<sqlx::PgPool>,
    Extension(claims): Extension<AccessClaims>,
) -> Result<Json<Vec<BookingResponse>>> {
    let rows = sqlx::query(
        r#"
        SELECT b.user_id, b.form_id, b.created_at, sf.title, sf.start_time
        FROM records.bookings b
        JOIN records.session_forms sf ON b.form_id = sf.id
        WHERE b.user_id = $1
        ORDER BY sf.start_time ASC
        "#
    )
    .bind(claims.user_id)
    .fetch_all(&pool)
    .await?;

    let bookings: Vec<BookingResponse> = rows.into_iter().map(|row| {
        BookingResponse {
            user_id: row.get("user_id"),
            session_id: row.get("form_id"),
            created_at: row.get("created_at"),
            session_title: row.get("title"),
            session_start_time: row.get("start_time"),
        }
    }).collect();

    Ok(Json(bookings))
}

async fn list_session_bookings(
    State(pool): State<sqlx::PgPool>,
    Extension(claims): Extension<AccessClaims>,
    Path(session_id): Path<Uuid>,
) -> Result<Json<Vec<BookingUserResponse>>> {
    // Check if user has permission to view bookings (session author or admin)
    let session_author = sqlx::query_scalar::<_, Uuid>(
        "SELECT author_id FROM records.session_forms WHERE id = $1"
    )
    .bind(session_id)
    .fetch_optional(&pool)
    .await?;

    match session_author {
        Some(author_id) if author_id != claims.user_id && !claims.admin => {
            return Err(Error::UnprocessableEntity("Access denied".to_string()));
        }
        None => {
            return Err(Error::UnprocessableEntity("Session not found".to_string()));
        }
        _ => {} // User is authorized
    }

    let rows = sqlx::query(
        r#"
        SELECT b.user_id, b.created_at, 
               u.first_name || ' ' || u.surname as user_name,
               u.shortcode, u.tier
        FROM records.bookings b
        JOIN auth.users u ON b.user_id = u.id
        WHERE b.form_id = $1
        ORDER BY b.created_at ASC
        "#
    )
    .bind(session_id)
    .fetch_all(&pool)
    .await?;

    let bookings: Vec<BookingUserResponse> = rows.into_iter().map(|row| {
        BookingUserResponse {
            user_id: row.get("user_id"),
            user_name: row.get("user_name"),
            shortcode: row.get("shortcode"),
            tier: row.get("tier"),
            created_at: row.get("created_at"),
        }
    }).collect();

    Ok(Json(bookings))
}