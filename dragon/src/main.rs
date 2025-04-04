mod auth;
mod crypto;
mod db;
mod errors;
mod handlers;
mod middleware;
mod models;
mod routes;

use dotenv::dotenv;
use tower_http::trace::TraceLayer;
use crate::middleware::cors_layer;
use std::net::SocketAddr;
use axum::{
    routing::{get, post, put, delete},
    Router,
    middleware,
};
use tower::ServiceBuilder;
use tower_http::cors::{CorsLayer, Any};

use dragonfruit::{
    db::create_pool,
    handlers::{
        // Auth handlers
        register, login, generate_totp_for_user, enable_totp, get_profile, update_profile,
        
        // Category handlers (need to implement these)
        
        // Credential handlers (need to implement these)
    },
    middleware::auth::{require_auth, AuthUser},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables from .env file
    dotenv::dotenv().ok();
    
    // Set up tracing for logging
    tracing_subscriber::fmt::init();
    
    // Create database connection pool
    let pool = create_pool().await?;
    
    // Set up CORS middleware
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);
    
    // Define public routes (no auth required)
    let public_routes = Router::new()
        .route("/auth/register", post(register))
        .route("/auth/login", post(login))
        .with_state(pool.clone());
    
    // Define protected routes (auth required)
    let protected_routes = Router::new()
        .route("/auth/profile", get(get_profile))
        .route("/auth/profile", put(update_profile))
        .route("/auth/totp/generate", post(generate_totp_for_user))
        .route("/auth/totp/enable", post(enable_totp))
        .with_state(pool.clone())
        .route_layer(middleware::from_fn_with_state(pool.clone(), require_auth));
        
    // Build the application with middleware
    let app = Router::new()
        .nest("/api", 
            Router::new()
                .merge(public_routes)
                .merge(protected_routes)
        )
        .layer(cors)
        .fallback(|| async { "Dragon Fruit Password Manager API" });
    
    // Start the server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Starting server on {}", addr);
    
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;
    
    Ok(())
}
