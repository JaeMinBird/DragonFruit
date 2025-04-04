use std::env;
use tower_http::cors::{CorsLayer, Any};

pub fn cors_layer() -> CorsLayer {
    let frontend_origin = env::var("FRONTEND_ORIGIN")
        .unwrap_or_else(|_| "http://localhost:5173".to_string());
    
    CorsLayer::new()
        .allow_origin(frontend_origin.parse::<axum::http::HeaderValue>().unwrap())
        .allow_methods([
            axum::http::Method::GET,
            axum::http::Method::POST,
            axum::http::Method::PUT,
            axum::http::Method::DELETE,
            axum::http::Method::OPTIONS,
        ])
        .allow_headers(Any)
        .allow_credentials(true)
} 