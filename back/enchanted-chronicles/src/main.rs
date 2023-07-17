use axum::{
    routing::get,
    Router,
    Json,
    extract::Path,
};

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use aws_sdk_dynamodb::{Client, types::AttributeValue};
use aws_types::region::Region;

async fn paragraph_handler(Path(id): Path<u64>) -> impl axum::response::IntoResponse {

    let config = aws_config::from_env().region(Region::new("us-east-1")).load().await;
    let client = Client::new(&config);

    let res = client.get_item()
        .table_name("paragraphs")
        .key("id", AttributeValue::N(id.to_string()))
        .send()
        .await;

    let item = res.unwrap().item.unwrap();
    let text = item.get("NewValue");
    let n = text.unwrap().as_s().unwrap();
   
    let json_response = serde_json::json!({
        "status": "ok",
        "message": n
    });
    Json(json_response)
}

#[tokio::main]
async fn main() {
    // initizlize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "axum_api=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();


        let app = Router::new()
            .route("/paragraph/:id", get(paragraph_handler)); 

        let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3000));
        tracing::debug!("listening on {}", addr);
        axum::Server::bind(&addr)
            .serve(app.into_make_service())
            .await
            .expect("server failed to start");

}