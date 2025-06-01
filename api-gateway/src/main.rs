use axum::{
    extract::{Path, Query, State},
    http::{HeaderMap, Method, StatusCode, Uri},
    middleware,
    response::{IntoResponse, Response},
    routing::{any, get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::Arc,
    time::{Duration, Instant},
};
use tokio::sync::RwLock;
use tower::ServiceBuilder;
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
    compression::CompressionLayer,
};
use tracing::{info, warn, error};
use uuid::Uuid;

mod config;
mod middleware;
mod proxy;
mod rate_limiter;
mod health;
mod metrics;
mod auth;

use config::Config;
use middleware::{auth_middleware, logging_middleware, rate_limit_middleware};
use proxy::ProxyService;
use rate_limiter::RateLimiter;
use health::HealthChecker;
use metrics::MetricsCollector;

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<Config>,
    pub proxy_service: Arc<ProxyService>,
    pub rate_limiter: Arc<RateLimiter>,
    pub health_checker: Arc<HealthChecker>,
    pub metrics: Arc<MetricsCollector>,
}

#[derive(Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
    pub request_id: String,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T, request_id: String) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
            request_id,
        }
    }

    pub fn error(error: String, request_id: String) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(error),
            request_id,
        }
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter("api_gateway=debug,tower_http=debug")
        .init();

    info!("Starting API Gateway...");

    // Load configuration
    let config = Arc::new(Config::load()?);
    info!("Configuration loaded successfully");

    // Initialize services
    let proxy_service = Arc::new(ProxyService::new(config.clone()).await?);
    let rate_limiter = Arc::new(RateLimiter::new(config.clone()).await?);
    let health_checker = Arc::new(HealthChecker::new(config.clone()));
    let metrics = Arc::new(MetricsCollector::new());

    // Create application state
    let state = AppState {
        config: config.clone(),
        proxy_service,
        rate_limiter,
        health_checker,
        metrics,
    };

    // Start health checking background task
    let health_checker_clone = state.health_checker.clone();
    tokio::spawn(async move {
        health_checker_clone.start_health_checks().await;
    });

    // Build the router
    let app = Router::new()
        // Health and metrics endpoints
        .route("/health", get(health_endpoint))
        .route("/metrics", get(metrics_endpoint))
        .route("/admin/config", get(config_endpoint))
        .route("/admin/routes", get(routes_endpoint))
        
        // Proxy all other requests
        .route("/*path", any(proxy_handler))
        .fallback(proxy_handler)
        
        // Add middleware layers
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(CompressionLayer::new())
                .layer(CorsLayer::new()
                    .allow_origin(Any)
                    .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
                    .allow_headers(Any))
                .layer(middleware::from_fn_with_state(state.clone(), logging_middleware))
                .layer(middleware::from_fn_with_state(state.clone(), rate_limit_middleware))
                .layer(middleware::from_fn_with_state(state.clone(), auth_middleware))
        )
        .with_state(state);

    // Start the server
    let addr = SocketAddr::from(([0, 0, 0, 0], config.server.port));
    info!("API Gateway listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn health_endpoint(State(state): State<AppState>) -> impl IntoResponse {
    let request_id = Uuid::new_v4().to_string();
    let health_status = state.health_checker.get_health_status().await;
    
    Json(ApiResponse::success(health_status, request_id))
}

async fn metrics_endpoint(State(state): State<AppState>) -> impl IntoResponse {
    let request_id = Uuid::new_v4().to_string();
    let metrics = state.metrics.get_metrics().await;
    
    Json(ApiResponse::success(metrics, request_id))
}

async fn config_endpoint(State(state): State<AppState>) -> impl IntoResponse {
    let request_id = Uuid::new_v4().to_string();
    
    // Return sanitized config (without sensitive data)
    let config_info = serde_json::json!({
        "version": "1.0.0",
        "server": {
            "port": state.config.server.port,
            "host": state.config.server.host
        },
        "routes": state.config.routes.len(),
        "rate_limiting": {
            "enabled": state.config.rate_limiting.enabled,
            "default_limit": state.config.rate_limiting.default_requests_per_minute
        }
    });
    
    Json(ApiResponse::success(config_info, request_id))
}

async fn routes_endpoint(State(state): State<AppState>) -> impl IntoResponse {
    let request_id = Uuid::new_v4().to_string();
    let routes: Vec<_> = state.config.routes.iter()
        .map(|route| serde_json::json!({
            "path": route.path,
            "method": route.method,
            "backend": route.backend,
            "load_balancing": route.load_balancing,
            "rate_limit": route.rate_limit
        }))
        .collect();
    
    Json(ApiResponse::success(routes, request_id))
}

async fn proxy_handler(
    State(state): State<AppState>,
    method: Method,
    uri: Uri,
    headers: HeaderMap,
    body: axum::body::Body,
) -> Result<Response, StatusCode> {
    let request_id = Uuid::new_v4().to_string();
    
    // Record request metrics
    state.metrics.record_request(&method.to_string(), uri.path()).await;
    
    let start_time = Instant::now();
    
    // Proxy the request
    match state.proxy_service.proxy_request(method, uri, headers, body, &request_id).await {
        Ok(response) => {
            let duration = start_time.elapsed();
            state.metrics.record_response_time(duration).await;
            Ok(response)
        }
        Err(e) => {
            let duration = start_time.elapsed();
            state.metrics.record_response_time(duration).await;
            state.metrics.record_error(&e.to_string()).await;
            
            error!("Proxy error: {} (request_id: {})", e, request_id);
            Err(StatusCode::BAD_GATEWAY)
        }
    }
} 