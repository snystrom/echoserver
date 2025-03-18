use axum::{
    extract::{Request, Json},
    http::StatusCode,
    response::IntoResponse,
    routing::any,
    Router,
};
use serde_json::{json, Value};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // Create a router that handles all HTTP methods on all paths
    let app = Router::new().fallback(any(echo_handler));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn echo_handler(request: Request) -> impl IntoResponse {
    // Extract method and path
    let method = request.method().clone();
    let uri = request.uri().clone();

    // Extract headers
    let headers = request.headers().clone();
    let headers_json: Value = headers
        .iter()
        .map(|(name, value)| {
            (
                name.to_string(),
                // Explicitly convert to String to avoid ambiguity
                value.to_str().unwrap_or("invalid utf-8").to_string(),
            )
        })
        .collect();

    // Extract body
    let (parts, body) = request.into_parts();

    let bytes = match axum::body::to_bytes(body, usize::MAX).await {
        Ok(bytes) => bytes,
        Err(_) => return (StatusCode::INTERNAL_SERVER_ERROR, "Error reading body").into_response(),
    };

    let body_str = if bytes.is_empty() {
        None
    } else {
        match String::from_utf8(bytes.to_vec()) {
            Ok(s) => Some(s),
            Err(_) => Some("Body contains invalid UTF-8".to_string()),
        }
    };

    let response = json!({
        "method": method.as_str(),
        "endpoint": uri.to_string(),
        "body": body_str,
        "headers": headers_json,
    });


    // Convert the JSON to a string for logging
    let response_str = serde_json::to_string_pretty(&response).unwrap_or_else(|e| {
        format!("Error serializing response: {}", e)
    });

    println!("Response: \n{}", response_str);

    Json(response).into_response()
}
