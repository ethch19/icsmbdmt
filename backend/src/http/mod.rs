// backend/src/http/mod.rs - Updated with public routes
use axum::{middleware::from_fn, Router};
use tower_http::cors::CorsLayer;
use axum::http::{Method, HeaderValue};

mod defaults;
mod pg_interval;
mod sessions;
mod bookings;
mod token;
mod users;
mod public; // Add public module

pub use self::token::AuthError;
pub use self::users::{get_members, User};
use crate::Result;

pub fn router_app(db: sqlx::PgPool) -> Router {
    let cors = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE, Method::OPTIONS])
        .allow_headers(tower_http::cors::Any);

    let user_router = Router::new().merge(token::router()).merge(users::router());
    
    // Admin routes (protected with admin middleware)
    let admin_router = Router::new()
        .merge(users::admin_router())
        .layer(from_fn(admin_auth_middleware));
    
    // Protected routes (require authentication)
    let protected_routes = Router::new()
        .nest("/sessions", sessions::router())
        .nest("/bookings", bookings::router())
        .layer(from_fn(token::mid_jwt_auth));
    
    // Public routes (no authentication required)
    let public_router = Router::new()
        .nest("/public", public::router());
    
    let v1_routes = Router::new()
        .merge(protected_routes) // Protected routes
        .nest("/users", user_router) // User routes (mixed auth)
        .nest("/admin", admin_router) // Admin routes
        .merge(public_router); // Public routes
    
    Router::new()
        .nest("/api/v1", v1_routes)
        .layer(cors)
        .with_state(db)
}

// Admin authentication middleware
async fn admin_auth_middleware(
    req: axum::extract::Request,
    next: axum::middleware::Next,
) -> Result<axum::response::Response> {
    // Get the claims from the JWT middleware that should have run first
    let claims = req.extensions().get::<token::AccessClaims>();
    
    match claims {
        Some(claims) if claims.admin || claims.tier >= 2 => {
            // User is admin or team member, allow access
            Ok(next.run(req).await)
        }
        _ => {
            // User is not admin/team member, deny access
            Err(crate::Error::UnprocessableEntity("Admin access required".to_string()))
        }
    }
}

pub async fn serve(db: sqlx::PgPool) -> Result<()> {
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, router_app(db)).await;
    Ok(())
}