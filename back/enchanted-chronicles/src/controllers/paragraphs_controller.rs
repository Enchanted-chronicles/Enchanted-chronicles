use axum::{Json, extract::Query, response::IntoResponse};

use crate::services::paragraphs_service::paragraph_service;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ParagraphParams {
    id: Option<u64>,
}

pub async fn get_paragraph(Query(params): Query<ParagraphParams>) -> impl IntoResponse {
    let paragraph = paragraph_service(params.id.unwrap()).await;

    let json_response = serde_json::json!({
        "status": "ok",
        "message": paragraph 
    });
    Json(json_response)
}