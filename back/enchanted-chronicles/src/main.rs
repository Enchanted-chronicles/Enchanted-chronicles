use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use axum::{routing::get, Router};

// Importing all the submodules
mod controllers{ pub mod paragraphs_controller; }
mod services{ pub mod paragraphs_service; }
mod repositories{ pub mod paragraphs_repository; }

pub fn api_router_routes() -> Router {
    Router::new()
        .route("/api/paragraph/:id", get(controllers::paragraphs_controller::get_paragraph))
}

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "axum_api=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

        let app = Router::new()
            .merge(api_router_routes()); 

        let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3000));
        tracing::debug!("listening on {}", addr);
        axum::Server::bind(&addr)
            .serve(app.into_make_service())
            .await
            .expect("server failed to start");

}

