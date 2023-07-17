use aws_sdk_dynamodb::{Client, types::AttributeValue};
use aws_types::region::Region;
use axum::{Json, extract::Path};

pub async fn paragraph_handler(Path(id): Path<u64>) -> impl axum::response::IntoResponse {

    let config = aws_config::from_env().region(Region::new("us-east-1")).load().await;
    let client = Client::new(&config);

    let res = client.get_item()
        .table_name("paragraphs")
        .key("id", AttributeValue::N(id.to_string()))
        .send()
        .await;

    let item = res.unwrap().item.unwrap();
    let text = item.get("NewValue");
    let paragraph = text.unwrap().as_s().unwrap();
   
    let json_response = serde_json::json!({
        "status": "ok",
        "message": paragraph
    });
    Json(json_response)
}