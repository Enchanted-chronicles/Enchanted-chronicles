use aws_sdk_dynamodb::{Client, types::AttributeValue};
use aws_types::region::Region;

pub async fn dynamodb_paragraphs(id: u64) -> 
    Result<
        aws_sdk_dynamodb::operation::get_item::GetItemOutput, 
        aws_sdk_dynamodb::error::SdkError<aws_sdk_dynamodb::operation::get_item::GetItemError>
    > {

    let config = aws_config::from_env().region(Region::new("us-east-1")).load().await;
    let client = Client::new(&config);

    let res = client.get_item()
    .table_name("paragraphs")
    .key("id", AttributeValue::N(id.to_string()))
    .send()
    .await;
    res
}