use axum::{
    body::Body,
    http::{HeaderMap, Method, StatusCode, Uri},
    response::Response,
};
use reqwest::Client;
use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
    time::Duration,
};
use tokio::sync::RwLock;
use tracing::{debug, error, info, warn};

use crate::config::{BackendConfig, Config, LoadBalancingStrategy, RouteConfig};

#[derive(Clone)]
pub struct ProxyService {
    config: Arc<Config>,
    client: Client,
    backend_states: Arc<RwLock<HashMap<String, BackendState>>>,
}

#[derive(Debug, Clone)]
struct BackendState {
    servers: Vec<ServerState>,
    current_index: Arc<AtomicUsize>,
}

#[derive(Debug, Clone)]
struct ServerState {
    url: String,
    healthy: bool,
    connections: Arc<AtomicUsize>,
}

impl ProxyService {
    pub async fn new(config: Arc<Config>) -> anyhow::Result<Self> {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()?;

        let mut backend_states = HashMap::new();
        
        for (name, backend) in &config.backends {
            let servers = backend
                .servers
                .iter()
                .map(|url| ServerState {
                    url: url.clone(),
                    healthy: true,
                    connections: Arc::new(AtomicUsize::new(0)),
                })
                .collect();

            backend_states.insert(
                name.clone(),
                BackendState {
                    servers,
                    current_index: Arc::new(AtomicUsize::new(0)),
                },
            );
        }

        Ok(Self {
            config,
            client,
            backend_states: Arc::new(RwLock::new(backend_states)),
        })
    }

    pub async fn proxy_request(
        &self,
        method: Method,
        uri: Uri,
        headers: HeaderMap,
        body: Body,
        request_id: &str,
    ) -> anyhow::Result<Response> {
        // Find matching route
        let route = self.find_matching_route(&uri.path())?;
        
        // Get backend configuration
        let backend = self.config.backends.get(&route.backend)
            .ok_or_else(|| anyhow::anyhow!("Backend '{}' not found", route.backend))?;

        // Select server based on load balancing strategy
        let server_url = self.select_server(backend, &route.load_balancing).await?;
        
        debug!(
            "Proxying request to {} (backend: {}, server: {}, request_id: {})",
            uri.path(),
            route.backend,
            server_url,
            request_id
        );

        // Build target URL
        let target_url = format!("{}{}", server_url, uri.path_and_query().map(|pq| pq.as_str()).unwrap_or(""));

        // Convert axum body to reqwest body
        let body_bytes = axum::body::to_bytes(body, usize::MAX).await?;

        // Build request
        let mut request_builder = self.client.request(method.clone(), &target_url);

        // Copy headers (excluding host and connection headers)
        for (name, value) in headers.iter() {
            let name_str = name.as_str().to_lowercase();
            if !["host", "connection", "content-length"].contains(&name_str.as_str()) {
                request_builder = request_builder.header(name, value);
            }
        }

        // Add request ID header
        request_builder = request_builder.header("X-Request-ID", request_id);

        // Add body if present
        if !body_bytes.is_empty() {
            request_builder = request_builder.body(body_bytes);
        }

        // Set timeout
        if let Some(timeout_ms) = route.timeout_ms {
            request_builder = request_builder.timeout(Duration::from_millis(timeout_ms));
        }

        // Execute request
        let response = request_builder.send().await?;

        // Convert reqwest response to axum response
        let status = StatusCode::from_u16(response.status().as_u16())?;
        let mut response_headers = HeaderMap::new();

        // Copy response headers
        for (name, value) in response.headers().iter() {
            if let Ok(header_name) = axum::http::HeaderName::from_bytes(name.as_str().as_bytes()) {
                if let Ok(header_value) = axum::http::HeaderValue::from_bytes(value.as_bytes()) {
                    response_headers.insert(header_name, header_value);
                }
            }
        }

        let body_bytes = response.bytes().await?;
        let body = Body::from(body_bytes);

        let mut response_builder = Response::builder().status(status);
        
        // Add headers to response
        for (name, value) in response_headers.iter() {
            response_builder = response_builder.header(name, value);
        }

        let response = response_builder.body(body)?;

        info!(
            "Request proxied successfully (status: {}, request_id: {})",
            status,
            request_id
        );

        Ok(response)
    }

    fn find_matching_route(&self, path: &str) -> anyhow::Result<&RouteConfig> {
        for route in &self.config.routes {
            if self.path_matches(&route.path, path) {
                return Ok(route);
            }
        }
        
        Err(anyhow::anyhow!("No matching route found for path: {}", path))
    }

    fn path_matches(&self, pattern: &str, path: &str) -> bool {
        if pattern.ends_with("*") {
            let prefix = &pattern[..pattern.len() - 1];
            path.starts_with(prefix)
        } else {
            pattern == path
        }
    }

    async fn select_server(
        &self,
        backend: &BackendConfig,
        strategy: &LoadBalancingStrategy,
    ) -> anyhow::Result<String> {
        let backend_states = self.backend_states.read().await;
        let backend_state = backend_states.get(&backend.name)
            .ok_or_else(|| anyhow::anyhow!("Backend state not found: {}", backend.name))?;

        let healthy_servers: Vec<_> = backend_state
            .servers
            .iter()
            .filter(|server| server.healthy)
            .collect();

        if healthy_servers.is_empty() {
            return Err(anyhow::anyhow!("No healthy servers available for backend: {}", backend.name));
        }

        let selected_server = match strategy {
            LoadBalancingStrategy::RoundRobin => {
                let index = backend_state.current_index.fetch_add(1, Ordering::Relaxed);
                &healthy_servers[index % healthy_servers.len()]
            }
            LoadBalancingStrategy::LeastConnections => {
                healthy_servers
                    .iter()
                    .min_by_key(|server| server.connections.load(Ordering::Relaxed))
                    .unwrap()
            }
            LoadBalancingStrategy::Random => {
                use rand::Rng;
                let index = rand::thread_rng().gen_range(0..healthy_servers.len());
                &healthy_servers[index]
            }
            LoadBalancingStrategy::WeightedRoundRobin => {
                // For simplicity, fall back to round robin
                let index = backend_state.current_index.fetch_add(1, Ordering::Relaxed);
                &healthy_servers[index % healthy_servers.len()]
            }
        };

        // Increment connection count
        selected_server.connections.fetch_add(1, Ordering::Relaxed);

        Ok(selected_server.url.clone())
    }

    pub async fn update_server_health(&self, backend_name: &str, server_url: &str, healthy: bool) {
        let mut backend_states = self.backend_states.write().await;
        if let Some(backend_state) = backend_states.get_mut(backend_name) {
            for server in &mut backend_state.servers {
                if server.url == server_url {
                    server.healthy = healthy;
                    if healthy {
                        info!("Server {} marked as healthy", server_url);
                    } else {
                        warn!("Server {} marked as unhealthy", server_url);
                    }
                    break;
                }
            }
        }
    }

    pub async fn get_backend_status(&self) -> HashMap<String, Vec<(String, bool, usize)>> {
        let backend_states = self.backend_states.read().await;
        let mut status = HashMap::new();

        for (name, state) in backend_states.iter() {
            let servers: Vec<_> = state
                .servers
                .iter()
                .map(|server| {
                    (
                        server.url.clone(),
                        server.healthy,
                        server.connections.load(Ordering::Relaxed),
                    )
                })
                .collect();
            status.insert(name.clone(), servers);
        }

        status
    }
} 