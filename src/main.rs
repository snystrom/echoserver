use axum::{
    extract::{Request, Json},
    http::StatusCode,
    response::IntoResponse,
    routing::any,
    Router,
};
use serde_json::{json, Value};
use std::net::{IpAddr, SocketAddr};
use std::str::FromStr;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "echoserver", about = "HTTP server that echoes request details")]
struct Opt {
    /// IP address to bind the server to
    #[structopt(short, long, default_value = "127.0.0.1")]
    ip: String,

    /// Port to bind the server to
    #[structopt(short, long, default_value = "3000")]
    port: u16,

    /// Return a simple 200 Success instead of the full JSON response
    #[structopt(short, long)]
    quiet: bool,

    /// Mask the Authorization header value with "***" in the response
    #[structopt(short = "m", long)]
    mask_auth: bool,
}

#[tokio::main]
async fn main() {
    // Parse command-line arguments
    let opt = Opt::from_args();

    // Parse the IP address
    let ip = IpAddr::from_str(&opt.ip).unwrap_or_else(|_| {
        eprintln!("Invalid IP address: {}, using 127.0.0.1 instead", opt.ip);
        IpAddr::from_str("127.0.0.1").unwrap()
    });

    let addr = SocketAddr::from((ip, opt.port));

    // Create a router that handles all HTTP methods on all paths
    let app = Router::new().fallback(any(move |req| echo_handler(req, opt.quiet, opt.mask_auth)));


    println!("Server running on http://{}", addr);

    // Start the server
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap_or_else(|e| {
        eprintln!("Failed to bind to {}: {}", addr, e);
        std::process::exit(1);
    });

    axum::serve(listener, app).await.unwrap_or_else(|e| {
        eprintln!("Server error: {}", e);
        std::process::exit(1);
    });
}

async fn echo_handler(request: Request, quiet: bool, mask_auth: bool) -> impl IntoResponse {
    // Extract method and path
    let method = request.method().clone();
    let uri = request.uri().clone();

    // Extract headers
    let headers = request.headers().clone();
    let headers_json: Value = headers
        .iter()
        .map(|(name, value)| {
            let header_name = name.to_string();
            let header_value = if mask_auth &&
                               header_name.to_lowercase() == "authorization" {
                // Mask the authorization header value
                "***".to_string()
            } else {
                // Use the original header value
                value.to_str().unwrap_or("invalid utf-8").to_string()
            };
            (
                header_name, header_value
            )

         })
        .collect();

    // Extract body
    let (_parts, body) = request.into_parts();

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

    if quiet {
        StatusCode::OK.into_response()
    } else {
        Json(response).into_response()
    }
}
