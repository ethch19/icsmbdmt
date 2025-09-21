// backend/src/http/public.rs - Public API endpoints that don't require authentication
use axum::{
    extract::{State, Query},
    http::StatusCode,
    routing::get,
    Json, Router,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use sqlx::Row;

use crate::{Error, Result};

#[derive(Debug, Serialize)]
pub struct PublicSessionResponse {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub location: String,
    pub tier: i16,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub user_limit: Option<i16>,
    pub current_bookings: i64,
    pub author_name: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct PublicSessionQuery {
    pub limit: Option<i64>,
    pub upcoming_only: Option<bool>,
    pub tier_filter: Option<i16>,
}

pub fn router() -> Router<sqlx::PgPool> {
    Router::new()
        .route("/sessions", get(list_public_sessions))
        .route("/sessions/featured", get(list_featured_sessions))
        .route("/stats", get(get_public_stats))
}

async fn list_public_sessions(
    State(pool): State<sqlx::PgPool>,
    Query(query): Query<PublicSessionQuery>,
) -> Result<Json<Vec<PublicSessionResponse>>> {
    let limit = query.limit.unwrap_or(50).min(100); // Max 100 sessions
    let upcoming_only = query.upcoming_only.unwrap_or(true);
    
    let mut sql = r#"
        SELECT sf.id, sf.title, sf.description, sf.location, sf.tier, 
               sf.start_time, sf.end_time, sf.user_limit, sf.created_at,
               u.first_name || ' ' || u.surname as author_name,
               COUNT(b.user_id) as current_bookings
        FROM records.session_forms sf
        LEFT JOIN auth.users u ON sf.author_id = u.id
        LEFT JOIN records.bookings b ON sf.id = b.form_id
        WHERE 1=1
    "#.to_string();
    
    if upcoming_only {
        sql.push_str(" AND sf.start_time > NOW()");
    }
    
    if let Some(tier) = query.tier_filter {
        sql.push_str(&format!(" AND sf.tier = {}", tier));
    }
    
    sql.push_str(r#"
        GROUP BY sf.id, u.first_name, u.surname
        ORDER BY sf.start_time ASC
        LIMIT $1
    "#);

    let rows = sqlx::query(&sql)
        .bind(limit)
        .fetch_all(&pool)
        .await?;

    let sessions: Vec<PublicSessionResponse> = rows.into_iter().map(|row| {
        PublicSessionResponse {
            id: row.get("id"),
            title: row.get("title"),
            description: row.get("description"),
            location: row.get("location"),
            tier: row.get("tier"),
            start_time: row.get("start_time"),
            end_time: row.get("end_time"),
            user_limit: row.get("user_limit"),
            current_bookings: row.get("current_bookings"),
            author_name: row.get("author_name"),
            created_at: row.get("created_at"),
        }
    }).collect();

    Ok(Json(sessions))
}

async fn list_featured_sessions(
    State(pool): State<sqlx::PgPool>,
) -> Result<Json<Vec<PublicSessionResponse>>> {
    // Featured sessions: upcoming sessions with highest booking rates or team-level sessions
    let rows = sqlx::query(
        r#"
        SELECT sf.id, sf.title, sf.description, sf.location, sf.tier, 
               sf.start_time, sf.end_time, sf.user_limit, sf.created_at,
               u.first_name || ' ' || u.surname as author_name,
               COUNT(b.user_id) as current_bookings,
               CASE 
                   WHEN sf.user_limit IS NOT NULL THEN (COUNT(b.user_id)::float / sf.user_limit::float)
                   ELSE 0
               END as booking_rate
        FROM records.session_forms sf
        LEFT JOIN auth.users u ON sf.author_id = u.id
        LEFT JOIN records.bookings b ON sf.id = b.form_id
        WHERE sf.start_time > NOW()
        GROUP BY sf.id, u.first_name, u.surname
        ORDER BY sf.tier DESC, booking_rate DESC, current_bookings DESC
        LIMIT 6
        "#
    )
    .fetch_all(&pool)
    .await?;

    let sessions: Vec<PublicSessionResponse> = rows.into_iter().map(|row| {
        PublicSessionResponse {
            id: row.get("id"),
            title: row.get("title"),
            description: row.get("description"),
            location: row.get("location"),
            tier: row.get("tier"),
            start_time: row.get("start_time"),
            end_time: row.get("end_time"),
            user_limit: row.get("user_limit"),
            current_bookings: row.get("current_bookings"),
            author_name: row.get("author_name"),
            created_at: row.get("created_at"),
        }
    }).collect();

    Ok(Json(sessions))
}

#[derive(Debug, Serialize)]
pub struct PublicStats {
    pub total_sessions: i64,
    pub upcoming_sessions: i64,
    pub total_members: i64,
    pub sessions_this_month: i64,
}

async fn get_public_stats(
    State(pool): State<sqlx::PgPool>,
) -> Result<Json<PublicStats>> {
    let total_sessions: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM records.session_forms"
    )
    .fetch_one(&pool)
    .await?;

    let upcoming_sessions: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM records.session_forms WHERE start_time > NOW()"
    )
    .fetch_one(&pool)
    .await?;

    let total_members: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM auth.users WHERE tier >= 1"
    )
    .fetch_one(&pool)
    .await?;

    let sessions_this_month: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM records.session_forms WHERE created_at >= date_trunc('month', NOW())"
    )
    .fetch_one(&pool)
    .await?;

    Ok(Json(PublicStats {
        total_sessions,
        upcoming_sessions,
        total_members,
        sessions_this_month,
    }))
}