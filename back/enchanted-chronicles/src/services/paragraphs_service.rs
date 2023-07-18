use axum::response::IntoResponse;

use crate::repositories::paragraphs_repository::dynamodb_paragraphs;

pub async fn paragraph_service(id: u64) -> String {
    let res = dynamodb_paragraphs(id).await;
    match res {
        // Item output has been found form DynamoDB
        Ok( item_output ) => 
        {
            let item = item_output.item;
            match item {
                Some(item) => {
                    let text = item.get("NewValue");
                    match text {
                        // Item contains a paragraph
                        Some(text) => {
                            let uncasted_paragraph = text.as_s();

                            match uncasted_paragraph {
                                // Paragraph is a string
                                Ok(uncasted_paragraph) => {
                                    let paragraph = uncasted_paragraph.to_string();
                                    println!("Paragraph is a string: {}", paragraph);
                                    paragraph                                    
                                },
                                // Paragraph is not a string
                                Err(e) => {
                                    println!("Paragraph is not a string err: {:?}", e);
                                    "".to_string()
                                }
                            }
                        },
                        None => {
                            println!("No paragraph found for id: {}", id);
                            "".to_string()
                        },
                    }
                },
                None => {
                    println!("No paragraph found for id: {}", id);
                    "".to_string()
                }
            }
        },
        Err(e) => {
            println!("Got an error retreiving paragraph from DynamoDB: {:?}", e);
            "".to_string()
        },
    }
 } 