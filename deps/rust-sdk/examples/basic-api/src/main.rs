use std::error::Error;

use by_axum::axum::{routing::get, Router};
use by_axum::{ApiError, Result};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn Error>> {
    let app = Router::new().route("/hello", get(hello_handler));

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    by_axum::serve(listener, app).await.unwrap();

    Ok(())
}

#[derive(Serialize, Deserialize)]
struct HelloErrorResponse {
    error: String,
}

#[derive(Serialize, Deserialize, JsonSchema)]
struct HelloResponse {
    message: String,
}

async fn hello_handler() -> Result<HelloResponse, HelloErrorResponse> {
    Err(ApiError::InternalServerError(HelloErrorResponse {
        error: "Hello, world!".to_string(),
    }))
}
