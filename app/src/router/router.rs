use axum::{
    routing::get,
    Router,
    http::StatusCode,
    response::Html,
};

pub fn setup_routes() -> Router {
    Router::new()
        .route("/", get(root))
}

async fn root() -> (StatusCode, Html<&'static str>) {
    (
        StatusCode::OK,
        Html("<h1>Hello, World!</h1>"),
    )
}