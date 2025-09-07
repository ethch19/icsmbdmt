use axum::{
    extract::{State, Form},
    http::StatusCode,
    routing::{get, post},
    Router,
    Extension
};
use serde::{Deserialize, Serialize};
use sqlx::postgres::types::PgInterval;
use uuid::Uuid;
use validator::Validate;
use chrono::{DateTime, Utc};

use crate::http::defaults::{default_time, default_uuid};
use crate::http::token::AccessClaims;
use crate::{Error, Result};

#[derive(sqlx::FromRow, Debug, Deserialize, Serialize, Validate)]
pub struct SessionForm {
    #[serde(default = "default_uuid")]
    pub id: Uuid,
    pub author_id: Uuid,
    #[validate(length(min = 1, max = 50))]
    pub title: String,
    #[validate(length(min = 1, max = 500))]
    pub description: String,
    #[validate(length(min = 1, max = 100))]
    pub location: String,
    pub tier: i16,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    #[serde(default, with = "crate::http::pg_interval")]
    pub recurrence: Option<PgInterval>,
    pub recurrence_end: Option<DateTime<Utc>>,
    pub user_limit: Option<i16>,
    #[serde(default = "default_time")]
    pub created_at: DateTime<Utc>,
}

#[derive(sqlx::FromRow, Debug, Deserialize, Serialize)]
pub struct SessionResponse {
    pub id: Uuid,
    pub author_id: Uuid,
    pub author_name: String,
    pub title: String,
    pub description: String,
    pub location: String,
    pub tier: i16,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub recurrence: Option<PgInterval>,
    pub recurrence_end: Option<DateTime<Utc>>,
    pub user_limit: Option<i16>,
    pub current_bookings: i64,
    pub created_at: DateTime<Utc>,
    pub is_booked: bool,
}

#[derive(Deserialize, Serialize)]
pub struct CreateSessionRequest {
    pub title: String,
    pub description: String,
    pub location: String,
    pub tier: i16,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub recurrence: Option<PgInterval>,
    pub recurrence_end: Option<DateTime<Utc>>,
    pub user_limit: Option<i16>,
}

#[derive(Deserialize)]
pub struct SessionQuery {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub upcoming_only: Option<bool>,
}

#[derive(sqlx::FromRow, Serialize)]
pub struct BookingResponse {
    pub user_id: Uuid,
    pub form_id: Uuid,
    pub user_name: String,
    pub created_at: DateTime<Utc>,
}

pub fn router() -> Router<sqlx::PgPool> {
    Router::new()
        .route("/", get(get_sessions).post(create_session))
        .route("/:id", get(get_session).patch(update_session).delete(delete_session))
        .route("/:id/book", post(book_session))
        .route("/:id/unbook", delete(unbook_session))
        .route("/:id/bookings", get(get_session_bookings))
}

async fn create_session(
    State(pool): State<sqlx::PgPool>,
    Extension(claims): Extension<AccessClaims>,
    Json(payload): Json<CreateSessionRequest>,
) -> Result<Json<SessionResponse>> {
    // Check if user has permission to create sessions (admin or tier >= 2)
    if !claims.admin && claims.tier < 2 {
        return Err(Error::Auth(crate::http::AuthError::InvalidToken));
    }

    // Validate request
    if payload.start_time >= payload.end_time {
        return Err(Error::UnprocessableEntity("Start time must be before end time".to_string()));
    }
    //
    if let (Some(_), None) = (&payload.recurrence, &payload.recurrence_end) {
        return Err(Error::UnprocessableEntity("Recurrence end time required when recurrence is set".to_string()));
    }

    let session = sqlx::query_as!(
        SessionForm,
        r#"
        INSERT INTO records.session_forms 
        (author_id, title, description, location, tier, start_time, end_time, recurrence, recurrence_end, user_limit)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
        RETURNING *
        "#,
        claims.user_id,
        payload.title,
        payload.description,
        payload.location,
        payload.tier,
        payload.start_time,
        payload.end_time,
        payload.recurrence,
        payload.recurrence_end,
        payload.user_limit
    )
    .fetch_one(&pool)
    .await?;

    let response = SessionResponse {
        id: session.id,
        author_id: session.author_id,
        author_name: claims.name,
        title: session.title,
        description: session.description,
        location: session.location,
        tier: session.tier,
        start_time: session.start_time,
        end_time: session.end_time,
        recurrence: session.recurrence,
        recurrence_end: session.recurrence_end,
        user_limit: session.user_limit,
        current_bookings: 0,
        created_at: session.created_at,
        is_booked: false,
    };

    Ok(Json(response))
}



async fn get_sessions(
    State(pool): State<sqlx::PgPool>,
    Extension(claims): Extension<AccessClaims>,
    Query(query): Query<SessionQuery>,
) -> Result<Json<Vec<SessionResponse>>> {
    let limit = query.limit.unwrap_or(50).min(100);
    let offset = query.offset.unwrap_or(0);
    let upcoming_only = query.upcoming_only.unwrap_or(true);

    let mut sql = r#"
        SELECT 
            sf.*,
            CONCAT(u.first_name, ' ', u.surname) as author_name,
            COALESCE(booking_counts.count, 0) as current_bookings,
            CASE WHEN user_bookings.user_id IS NOT NULL THEN true ELSE false END as is_booked
        FROM records.session_forms sf
        JOIN auth.users u ON sf.author_id = u.id
        LEFT JOIN (
            SELECT form_id, COUNT(*) as count
            FROM records.bookings
            GROUP BY form_id
        ) booking_counts ON sf.id = booking_counts.form_id
        LEFT JOIN records.bookings user_bookings ON sf.id = user_bookings.form_id AND user_bookings.user_id = $1
        WHERE sf.tier <= $2
    "#.to_string();

    if upcoming_only {
        sql.push_str(" AND sf.start_time > NOW()");
    }

    sql.push_str(" ORDER BY sf.start_time ASC LIMIT $3 OFFSET $4");

    let sessions = sqlx::query_as!(
        SessionResponse,
        &sql,
        claims.user_id,
        claims.tier,
        limit,
        offset
    )
    .fetch_all(&pool)
    .await?;

    Ok(Json(sessions))
}

async fn get_session(
    State(pool): State<sqlx::PgPool>,
    Extension(claims): Extension<AccessClaims>,
    Path(id): Path<Uuid>,
) -> Result<Json<SessionResponse>> {
    let session = sqlx::query_as!(
        SessionResponse,
        r#"
        SELECT 
            sf.*,
            CONCAT(u.first_name, ' ', u.surname) as author_name,
            COALESCE(booking_counts.count, 0) as current_bookings,
            CASE WHEN user_bookings.user_id IS NOT NULL THEN true ELSE false END as is_booked
        FROM records.session_forms sf
        JOIN auth.users u ON sf.author_id = u.id
        LEFT JOIN (
            SELECT form_id, COUNT(*) as count
            FROM records.bookings
            GROUP BY form_id
        ) booking_counts ON sf.id = booking_counts.form_id
        LEFT JOIN records.bookings user_bookings ON sf.id = user_bookings.form_id AND user_bookings.user_id = $1
        WHERE sf.id = $2 AND sf.tier <= $3
        "#,
        claims.user_id,
        id,
        claims.tier
    )
    .fetch_optional(&pool)
    .await?;

    match session {
        Some(session) => Ok(Json(session)),
        None => Err(Error::UnprocessableEntity("Session not found".to_string())),
    }
}

async fn update_session(
    State(pool): State<sqlx::PgPool>,
    Extension(claims): Extension<AccessClaims>,
    Path(id): Path<Uuid>,
    Json(payload): Json<CreateSessionRequest>,
) -> Result<Json<SessionResponse>> {
    // Check if user owns the session or is admin
    let existing_session = sqlx::query!(
        "SELECT author_id FROM records.session_forms WHERE id = $1",
        id
    )
    .fetch_optional(&pool)
    .await?;

    let session = match existing_session {
        Some(session) => session,
        None => return Err(Error::UnprocessableEntity("Session not found".to_string())),
    };

    if session.author_id != claims.user_id && !claims.admin {
        return Err(Error::Auth(crate::http::AuthError::InvalidToken));
    }

    let updated_session = sqlx::query_as!(
        SessionForm,
        r#"
        UPDATE records.session_forms 
        SET title = $2, description = $3, location = $4, tier = $5, 
            start_time = $6, end_time = $7, recurrence = $8, recurrence_end = $9, user_limit = $10
        WHERE id = $1
        RETURNING *
        "#,
        id,
        payload.title,
        payload.description,
        payload.location,
        payload.tier,
        payload.start_time,
        payload.end_time,
        payload.recurrence,
        payload.recurrence_end,
        payload.user_limit
    )
    .fetch_one(&pool)
    .await?;

    let response = SessionResponse {
        id: updated_session.id,
        author_id: updated_session.author_id,
        author_name: claims.name,
        title: updated_session.title,
        description: updated_session.description,
        location: updated_session.location,
        tier: updated_session.tier,
        start_time: updated_session.start_time,
        end_time: updated_session.end_time,
        recurrence: updated_session.recurrence,
        recurrence_end: updated_session.recurrence_end,
        user_limit: updated_session.user_limit,
        current_bookings: 0, // Would need another query to get accurate count
        created_at: updated_session.created_at,
        is_booked: false, // Would need another query to check
    };

    Ok(Json(response))
}

async fn delete_session(
    State(pool): State<sqlx::PgPool>,
    Extension(claims): Extension<AccessClaims>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode> {
    // Check if user owns the session or is admin
    let existing_session = sqlx::query!(
        "SELECT author_id FROM records.session_forms WHERE id = $1",
        id
    )
    .fetch_optional(&pool)
    .await?;

    let session = match existing_session {
        Some(session) => session,
        None => return Err(Error::UnprocessableEntity("Session not found".to_string())),
    };

    if session.author_id != claims.user_id && !claims.admin {
        return Err(Error::Auth(crate::http::AuthError::InvalidToken));
    }

    sqlx::query!("DELETE FROM records.session_forms WHERE id = $1", id)
        .execute(&pool)
        .await?;

    Ok(StatusCode::NO_CONTENT)
}

async fn book_session(
    State(pool): State<sqlx::PgPool>,
    Extension(claims): Extension<AccessClaims>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode> {
    // Check if session exists and user has permission
    let session = sqlx::query!(
        r#"
        SELECT user_limit, tier,
               (SELECT COUNT(*) FROM records.bookings WHERE form_id = $1) as current_bookings
        FROM records.session_forms 
        WHERE id = $1 AND tier <= $2
        "#,
        id,
        claims.tier
    )
    .fetch_optional(&pool)
    .await?;

    let session = match session {
        Some(session) => session,
        None => return Err(Error::UnprocessableEntity("Session not found or insufficient permissions".to_string())),
    };

    // Check if session is full
    if let Some(limit) = session.user_limit {
        if session.current_bookings.unwrap_or(0) >= limit as i64 {
            return Err(Error::Conflict("Session is full".to_string()));
        }
    }

    // Try to create booking
    match sqlx::query!(
        "INSERT INTO records.bookings (user_id, form_id) VALUES ($1, $2)",
        claims.user_id,
        id
    )
    .execute(&pool)
    .await
    {
        Ok(_) => Ok(StatusCode::CREATED),
        Err(sqlx::Error::Database(err)) if err.is_unique_violation() => {
            Err(Error::Conflict("Already booked".to_string()))
        }
        Err(e) => Err(Error::Sqlx(e)),
    }
}

async fn unbook_session(
    State(pool): State<sqlx::PgPool>,
    Extension(claims): Extension<AccessClaims>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode> {
    let result = sqlx::query!(
        "DELETE FROM records.bookings WHERE user_id = $1 AND form_id = $2",
        claims.user_id,
        id
    )
    .execute(&pool)
    .await?;

    if result.rows_affected() == 0 {
        Err(Error::UnprocessableEntity("Booking not found".to_string()))
    } else {
        Ok(StatusCode::NO_CONTENT)
    }
}

async fn get_session_bookings(
    State(pool): State<sqlx::PgPool>,
    Extension(claims): Extension<AccessClaims>,
    Path(id): Path<Uuid>,
) -> Result<Json<Vec<BookingResponse>>> {
    // Check if user owns the session or is admin
    let session = sqlx::query!(
        "SELECT author_id FROM records.session_forms WHERE id = $1",
        id
    )
    .fetch_optional(&pool)
    .await?;

    let session = match session {
        Some(session) => session,
        None => return Err(Error::UnprocessableEntity("Session not found".to_string())),
    };

    if session.author_id != claims.user_id && !claims.admin {
        return Err(Error::Auth(crate::http::AuthError::InvalidToken));
    }

    let bookings = sqlx::query_as!(
        BookingResponse,
        r#"
        SELECT 
            b.user_id,
            b.form_id,
            CONCAT(u.first_name, ' ', u.surname) as user_name,
            b.created_at
        FROM records.bookings b
        JOIN auth.users u ON b.user_id = u.id
        WHERE b.form_id = $1
        ORDER BY b.created_at ASC
        "#,
        id
    )
    .fetch_all(&pool)
    .await?;

    Ok(Json(bookings))
}

// async fn create_session(
//     State(pool): State<sqlx::PgPool>,
//     Form(payload): Form<SessionForm>,
// ) -> Result<StatusCode> {
//     Ok(StatusCode::OK)
// }
