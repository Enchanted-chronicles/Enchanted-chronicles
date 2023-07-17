use crate::repositories::paragraphs_repository::dynamodb_paragraphs;

pub async fn paragraph_service(id: u64) -> String {
    let res = dynamodb_paragraphs(id).await;
    let item = res.unwrap().item.unwrap();
    let text = item.get("NewValue");
    let paragraph = text.unwrap().as_s().unwrap().to_string();
    paragraph
 }