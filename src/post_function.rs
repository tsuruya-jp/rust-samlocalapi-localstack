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
    event: LambdaEvent<ApiGatewayProxyRequest>,
) -> Result<ApiGatewayProxyResponse, Error> {

    let config = aws_config::defaults(BehaviorVersion::latest())
        .endpoint_url("http://localstack:4566")
        .load()
        .await;

    let client = Client::new(&config);

    let mut headers = HeaderMap::new();
    headers.insert("content-type", "application/json".parse().unwrap());

    let body = event.payload.body.as_ref();

    let resp = match body {
        Some(body_str) => {
            match serde_json::from_str::<User>(body_str) {
                Ok(user) => {
                    let result = client
                        .put_item()
                        .table_name("users")
                        .item("user_id", aws_sdk_dynamodb::types::AttributeValue::S(user.user_id.clone()))
                        .item("name", aws_sdk_dynamodb::types::AttributeValue::S(user.name.clone()))
                        .item("email", aws_sdk_dynamodb::types::AttributeValue::S(user.email.clone()))
                        .send()
                        .await;

                    match result {
                        Ok(_) => {
                            let response_body = json!({
                                "message": "User created successfully",
                                "user": user
                            }).to_string();

                            ApiGatewayProxyResponse {
                                status_code: 201,
                                multi_value_headers: headers.clone(),
                                is_base64_encoded: false,
                                body: Some(Body::Text(response_body)),
                                headers,
                            }
                        }
                        Err(err) => {
                            eprintln!("Failed to put item: {err:?}");

                            let error_body = json!({
                                "error": "Failed to create user",
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
                    }
                }
                Err(err) => {
                    eprintln!("Failed to parse JSON: {err:?}");

                    let error_body = json!({
                        "error": "Invalid request body",
                        "message": err.to_string()
                    }).to_string();

                    ApiGatewayProxyResponse {
                        status_code: 400,
                        multi_value_headers: headers.clone(),
                        is_base64_encoded: false,
                        body: Some(Body::Text(error_body)),
                        headers,
                    }
                }
            }
        }
        None => {
            let error_body = json!({
                "error": "Request body is required"
            }).to_string();

            ApiGatewayProxyResponse {
                status_code: 400,
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
    run(service_fn(handler)).await
}

