use axum::{routing::get, Router};

mod api;

pub fn api_router_routes() -> Router {
    Router::new()
        .route("/api/paragraph/:id", get(api::paragraph_handler))
}