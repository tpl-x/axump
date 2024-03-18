use axum::http::Method;
use tower_http::cors::{CorsLayer, Any};

pub fn with_cors() -> CorsLayer {
    CorsLayer::new()
        .allow_headers(Any)
        // allow `GET` and `POST` when accessing the resource
        .allow_methods([Method::GET, Method::POST])
        // allow requests from any origin
        .allow_origin(Any)
}

