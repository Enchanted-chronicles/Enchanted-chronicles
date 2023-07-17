use axum::Router;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
mod routes;

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
            .merge(routes::api_router_routes()); 

        let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3000));
        tracing::debug!("listening on {}", addr);
        axum::Server::bind(&addr)
            .serve(app.into_make_service())
            .await
            .expect("server failed to start");

}