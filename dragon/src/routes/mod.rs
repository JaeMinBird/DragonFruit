use crate::db::DbPool;
use crate::handlers;
use crate::middleware::{require_auth, AuthUser};
use axum::{
    middleware,
    routing::{get, post, put, delete},
    Router,
};
use std::sync::Arc;

// Create all routes for the API
pub fn create_router(db_pool: DbPool) -> Router {
    // Public routes (no authentication required)
    let public_routes = Router::new()
        .route("/register", post(handlers::register))
        .route("/login", post(handlers::login));
    
    // Protected routes (authentication required)
    let protected_routes = Router::new()
        // User routes
        .route("/me", get(handlers::me))
        .route("/totp/setup", post(handlers::setup_totp))
        .route("/totp/verify", post(handlers::verify_and_enable_totp))
        .route("/totp/disable", post(handlers::disable_totp))
        
        // Category routes
        .route("/categories", get(handlers::get_categories))
        .route("/categories", post(handlers::create_category))
        .route("/categories/:id", put(handlers::update_category))
        .route("/categories/:id", delete(handlers::delete_category))
        
        // Credential routes
        .route("/credentials", get(handlers::get_credentials))
        .route("/credentials", post(handlers::create_credential))
        .route("/credentials/:id", get(handlers::get_credential))
        .route("/credentials/:id", put(handlers::update_credential))
        .route("/credentials/:id", delete(handlers::delete_credential))
        .route("/categories/:id/credentials", get(handlers::get_credentials_by_category))
        
        // Apply auth middleware to all routes
        .route_layer(middleware::from_fn_with_state(
            db_pool.clone(),
            require_auth,
        ));
    
    // Combine routes and add state
    Router::new()
        .nest("/api", public_routes.merge(protected_routes))
        .with_state(db_pool)
} 