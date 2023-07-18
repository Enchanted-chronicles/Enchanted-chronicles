use axum::response::{IntoResponse, Html};

pub async fn handler_hello() -> impl IntoResponse {
    println!("->> {:<12} - handler_hello", "HANDLER");

    Html("Hello, <strong>World!<strong>")
}