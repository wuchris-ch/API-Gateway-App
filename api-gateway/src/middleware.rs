use axum::{
    extract::{Request, State},
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::Response,
};
use tracing::{info, warn};
use uuid::Uuid;

use crate::{auth::AuthService, AppState};

pub async fn logging_middleware(
    State(state): State<AppState>,
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let method = request.method().clone();
    let uri = request.uri().clone();
    let request_id = Uuid::new_v4().to_string();
    
    // Add request ID to headers
    let (mut parts, body) = request.into_parts();
    parts.headers.insert("X-Request-ID", request_id.parse().unwrap());
    let request = Request::from_parts(parts, body);

    info!(
        "Request started: {} {} (request_id: {})",
        method,
        uri,
        request_id
    );

    let start_time = std::time::Instant::now();
    let response = next.run(request).await;
    let duration = start_time.elapsed();

    info!(
        "Request completed: {} {} {} (duration: {:?}, request_id: {})",
        method,
        uri,
        response.status(),
        duration,
        request_id
    );

    Ok(response)
}

pub async fn rate_limit_middleware(
    State(state): State<AppState>,
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    if !state.config.rate_limiting.enabled {
        return Ok(next.run(request).await);
    }

    // Extract client identifier (IP address or API key)
    let client_id = extract_client_id(&request);
    
    // Check rate limit
    if let Err(_) = state.rate_limiter.check_rate_limit(&client_id).await {
        warn!("Rate limit exceeded for client: {}", client_id);
        return Err(StatusCode::TOO_MANY_REQUESTS);
    }

    Ok(next.run(request).await)
}

pub async fn auth_middleware(
    State(state): State<AppState>,
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    if !state.config.auth.enabled {
        return Ok(next.run(request).await);
    }

    let path = request.uri().path();
    
    // Check if path is in bypass list
    for bypass_path in &state.config.auth.bypass_paths {
        if path_matches(bypass_path, path) {
            return Ok(next.run(request).await);
        }
    }

    // Extract and validate authentication
    let headers = request.headers();
    
    if let Some(auth_header) = headers.get("Authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            if auth_str.starts_with("Bearer ") {
                let token = &auth_str[7..];
                if AuthService::validate_jwt_token(token, &state.config.auth.jwt_secret).is_ok() {
                    return Ok(next.run(request).await);
                }
            }
        }
    }

    // Check for API key
    if let Some(api_key_header) = headers.get(&state.config.auth.api_key_header) {
        if let Ok(api_key) = api_key_header.to_str() {
            if AuthService::validate_api_key(api_key).await.is_ok() {
                return Ok(next.run(request).await);
            }
        }
    }

    warn!("Authentication failed for path: {}", path);
    Err(StatusCode::UNAUTHORIZED)
}

fn extract_client_id(request: &Request) -> String {
    // Try to get API key first
    if let Some(api_key) = request.headers().get("X-API-Key") {
        if let Ok(key_str) = api_key.to_str() {
            return format!("api_key:{}", key_str);
        }
    }

    // Fall back to IP address
    if let Some(forwarded) = request.headers().get("X-Forwarded-For") {
        if let Ok(forwarded_str) = forwarded.to_str() {
            if let Some(ip) = forwarded_str.split(',').next() {
                return format!("ip:{}", ip.trim());
            }
        }
    }

    // Default to connection info
    "unknown".to_string()
}

fn path_matches(pattern: &str, path: &str) -> bool {
    if pattern.ends_with("*") {
        let prefix = &pattern[..pattern.len() - 1];
        path.starts_with(prefix)
    } else {
        pattern == path
    }
} 