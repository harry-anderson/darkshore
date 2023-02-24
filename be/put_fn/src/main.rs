use http::HeaderMap;
use aws_lambda_events::encodings::Body;
use aws_sdk_dynamodb::{model::AttributeValue, Client};
use lambda_runtime::{service_fn, Error, LambdaEvent};

use aws_lambda_events::apigw::{
    ApiGatewayV2httpRequest as Request, ApiGatewayV2httpResponse as Response,
};
use std::env;

/// Main function
#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_ansi(false)
        .without_time()
        .with_max_level(tracing_subscriber::filter::LevelFilter::INFO)
        .init();

    // Initialize the AWS SDK for Rust
    let config = aws_config::load_from_env().await;
    let table_name = env::var("TABLE_NAME").expect("TABLE_NAME must be set");
    let dynamodb_client = Client::new(&config);

    // Register the Lambda handler
    //
    // We use a closure to pass the `dynamodb_client` and `table_name` as arguments
    // to the handler function.
    lambda_runtime::run(service_fn(|request: LambdaEvent<Request>| {
        handler(&dynamodb_client, &table_name, request)
    }))
    .await?;

    Ok(())
}

/// This function will run for every invoke of the Lambda function.
async fn handler(
    client: &Client,
    table_name: &str,
    event: LambdaEvent<Request>,
) -> Result<Response, Error> {
    tracing::info!("event: {:?}", event);
    let req = event.payload;
    let path_parameters = req.path_parameters;
    let id = match path_parameters.get("id") {
        Some(id) => id,
        None => {
            return Ok(Response {
                status_code: 400,
                body: Some(Body::Text(String::from("id is required"))),
                headers: HeaderMap::new(),
                multi_value_headers: HeaderMap::new(),
                is_base64_encoded: None,
                cookies: vec![],
            })
        }
    };

    // Put the item in the DynamoDB table
    let res = client
        .put_item()
        .table_name(table_name)
        .item("PK", AttributeValue::S(id.to_string()))
        .item("SK", AttributeValue::S(id.to_string()))
        .item("data", AttributeValue::S(req.body.unwrap_or_default()))
        .send()
        .await;

    // Return a response to the end-user
    let (status_code, body) = match res {
        Ok(_) => (200, Some(Body::Text(String::from("itemsaved")))),
        Err(_) => (500, Some(Body::Text(String::from("internal server error")))),
    };

    Ok(Response {
        status_code,
        body,
        headers: HeaderMap::new(),
        multi_value_headers: HeaderMap::new(),
        is_base64_encoded: None,
        cookies: vec![],
    })
}

