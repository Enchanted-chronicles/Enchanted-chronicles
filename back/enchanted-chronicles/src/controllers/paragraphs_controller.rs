use axum::{Json, extract::Path};

use crate::services::paragraphs_service::paragraph_service;

pub async fn get_paragraph(Path(id): Path<u64>) -> impl axum::response::IntoResponse {
    let paragraph = paragraph_service(id).await;

    let json_response = serde_json::json!({
        "status": "ok",
        "message": paragraph
    });
    Json(json_response)
}