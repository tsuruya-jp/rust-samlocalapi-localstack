use aws_config::BehaviorVersion;
use aws_lambda_events::{apigw::{ApiGatewayProxyRequest, ApiGatewayProxyResponse}, encodings::Body, http::HeaderMap};
use aws_sdk_dynamodb::Client;
use lambda_runtime::{Error, LambdaEvent, run, service_fn};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Deserialize, Serialize)]
struct User {
    user_id: String,
    name: String,
    email: String,
}

pub async fn handler(
    _event: LambdaEvent<ApiGatewayProxyRequest>,
) -> Result<ApiGatewayProxyResponse, Error> {

    let config = aws_config::defaults(BehaviorVersion::latest())
        .endpoint_url("http://localstack:4566")
        .load()
        .await;

    let client = Client::new(&config);

    let res = client
        .scan()
        .table_name("users")
        .send()
        .await;

    let mut headers = HeaderMap::new();
    headers.insert("content-type", "application/json".parse().unwrap());

    let resp = match res {
        Ok(resp) => {
            tracing::info!("Found {} items", resp.items().len());
            let mut users: Vec<User> = Vec::new();

            for item in resp.items() {
                if let (Some(user_id), Some(name), Some(email)) = (
                    item.get("user_id").and_then(|v| v.as_s().ok()),
                    item.get("name").and_then(|v| v.as_s().ok()),
                    item.get("email").and_then(|v| v.as_s().ok()),
                ) {
                    users.push(User {
                        user_id: user_id.clone(),
                        name: name.clone(),
                        email: email.clone(),
                    });
                }
            }

            let body = json!({
                "users": users,
                "count": users.len()
            }).to_string();

            ApiGatewayProxyResponse {
                status_code: 200,
                multi_value_headers: headers.clone(),
                is_base64_encoded: false,
                body: Some(Body::Text(body)),
                headers,
            }
        }
        Err(err) => {
            tracing::error!("Failed to scan users table: {err:?}");

            let error_body = json!({
                "error": "Failed to get users",
                "message": err.to_string()
            }).to_string();

            ApiGatewayProxyResponse {
                status_code: 500,
                multi_value_headers: headers.clone(),
                is_base64_encoded: false,
                body: Some(Body::Text(error_body)),
                headers,
            }
        }
    };

    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt().json()
        .with_max_level(tracing::Level::INFO)
        .with_current_span(false)
        .with_ansi(false)
        .without_time()
        .with_target(false)
        .init();

    run(service_fn(handler)).await
}
