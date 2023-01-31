use aws_lambda_events::apigw::{
    ApiGatewayV2CustomAuthorizerSimpleResponse, ApiGatewayV2CustomAuthorizerV2Request,
};
use lambda_runtime::{run, service_fn, Error, LambdaEvent};
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt()
        .with_ansi(false)
        .without_time()
        .with_max_level(tracing_subscriber::filter::LevelFilter::INFO)
        .init();

    run(service_fn(handler)).await
}

pub async fn handler(
    event: LambdaEvent<ApiGatewayV2CustomAuthorizerV2Request>,
) -> Result<ApiGatewayV2CustomAuthorizerSimpleResponse, Error> {

    if let Some(token) = event.payload.headers.get("authorization") {
        // do something
        let allow = token.as_bytes() == b"harry_is_cool";

        Ok(ApiGatewayV2CustomAuthorizerSimpleResponse {
            is_authorized: allow,
            context: json!({}),
        })
    } else {

    Ok(ApiGatewayV2CustomAuthorizerSimpleResponse {
        is_authorized: false,
        context: json!({}),
    })
    }
}
