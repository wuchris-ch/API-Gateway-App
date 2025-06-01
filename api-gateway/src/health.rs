use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    sync::Arc,
    time::{Duration, Instant},
};
use tokio::{sync::RwLock, time::interval};
use tracing::{debug, error, info, warn};

use crate::config::Config;

#[derive(Clone)]
pub struct HealthChecker {
    config: Arc<Config>,
    client: Client,
    health_status: Arc<RwLock<HashMap<String, ServiceHealth>>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceHealth {
    pub service_name: String,
    pub servers: Vec<ServerHealth>,
    pub overall_status: HealthStatus,
    pub last_check: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerHealth {
    pub url: String,
    pub status: HealthStatus,
    pub response_time_ms: Option<u64>,
    pub last_check: u64,
    pub consecutive_failures: u32,
    pub consecutive_successes: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum HealthStatus {
    Healthy,
    Unhealthy,
    Unknown,
}

impl HealthChecker {
    pub fn new(config: Arc<Config>) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(10))
            .build()
            .expect("Failed to create HTTP client");

        let mut health_status = HashMap::new();
        
        for (name, backend) in &config.backends {
            let servers = backend
                .servers
                .iter()
                .map(|url| ServerHealth {
                    url: url.clone(),
                    status: HealthStatus::Unknown,
                    response_time_ms: None,
                    last_check: 0,
                    consecutive_failures: 0,
                    consecutive_successes: 0,
                })
                .collect();

            health_status.insert(
                name.clone(),
                ServiceHealth {
                    service_name: name.clone(),
                    servers,
                    overall_status: HealthStatus::Unknown,
                    last_check: 0,
                },
            );
        }

        Self {
            config,
            client,
            health_status: Arc::new(RwLock::new(health_status)),
        }
    }

    pub async fn start_health_checks(&self) {
        info!("Starting health check background task");
        
        let mut interval = interval(Duration::from_secs(30)); // Default interval
        
        loop {
            interval.tick().await;
            self.perform_health_checks().await;
        }
    }

    async fn perform_health_checks(&self) {
        debug!("Performing health checks for all backends");
        
        let mut futures = Vec::new();
        
        for (backend_name, backend_config) in &self.config.backends {
            if !backend_config.health_check.enabled {
                continue;
            }
            
            for server_url in &backend_config.servers {
                let future = self.check_server_health(
                    backend_name.clone(),
                    server_url.clone(),
                    backend_config.health_check.path.clone(),
                    backend_config.health_check.timeout_seconds,
                );
                futures.push(future);
            }
        }
        
        // Execute all health checks concurrently
        let results = futures::future::join_all(futures).await;
        
        // Update overall service health status
        self.update_service_health_status().await;
        
        debug!("Health checks completed for {} servers", results.len());
    }

    async fn check_server_health(
        &self,
        backend_name: String,
        server_url: String,
        health_path: String,
        timeout_seconds: u64,
    ) -> (String, String, bool, Option<u64>) {
        let health_url = format!("{}{}", server_url, health_path);
        let start_time = Instant::now();
        
        debug!("Checking health for server: {}", health_url);
        
        let client = self.client.clone();
        let request = client
            .get(&health_url)
            .timeout(Duration::from_secs(timeout_seconds));
        
        match request.send().await {
            Ok(response) => {
                let response_time = start_time.elapsed().as_millis() as u64;
                let is_healthy = response.status().is_success();
                
                if is_healthy {
                    debug!("Health check passed for {}: {} ({}ms)", server_url, response.status(), response_time);
                } else {
                    warn!("Health check failed for {}: {} ({}ms)", server_url, response.status(), response_time);
                }
                
                self.update_server_health(
                    &backend_name,
                    &server_url,
                    is_healthy,
                    Some(response_time),
                ).await;
                
                (backend_name, server_url, is_healthy, Some(response_time))
            }
            Err(e) => {
                let response_time = start_time.elapsed().as_millis() as u64;
                error!("Health check error for {}: {} ({}ms)", server_url, e, response_time);
                
                self.update_server_health(
                    &backend_name,
                    &server_url,
                    false,
                    Some(response_time),
                ).await;
                
                (backend_name, server_url, false, Some(response_time))
            }
        }
    }

    async fn update_server_health(
        &self,
        backend_name: &str,
        server_url: &str,
        is_healthy: bool,
        response_time_ms: Option<u64>,
    ) {
        let mut health_status = self.health_status.write().await;
        
        if let Some(service_health) = health_status.get_mut(backend_name) {
            for server_health in &mut service_health.servers {
                if server_health.url == server_url {
                    let now = std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_secs();
                    
                    server_health.last_check = now;
                    server_health.response_time_ms = response_time_ms;
                    
                    if is_healthy {
                        server_health.status = HealthStatus::Healthy;
                        server_health.consecutive_successes += 1;
                        server_health.consecutive_failures = 0;
                    } else {
                        server_health.status = HealthStatus::Unhealthy;
                        server_health.consecutive_failures += 1;
                        server_health.consecutive_successes = 0;
                    }
                    
                    break;
                }
            }
            
            service_health.last_check = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs();
        }
    }

    async fn update_service_health_status(&self) {
        let mut health_status = self.health_status.write().await;
        
        for (backend_name, backend_config) in &self.config.backends {
            if let Some(service_health) = health_status.get_mut(backend_name) {
                let healthy_servers = service_health
                    .servers
                    .iter()
                    .filter(|server| {
                        server.status == HealthStatus::Healthy &&
                        server.consecutive_successes >= backend_config.health_check.healthy_threshold
                    })
                    .count();
                
                let total_servers = service_health.servers.len();
                
                service_health.overall_status = if healthy_servers == 0 {
                    HealthStatus::Unhealthy
                } else if healthy_servers == total_servers {
                    HealthStatus::Healthy
                } else {
                    // Partially healthy - still consider it healthy if at least one server is up
                    HealthStatus::Healthy
                };
            }
        }
    }

    pub async fn get_health_status(&self) -> HashMap<String, ServiceHealth> {
        self.health_status.read().await.clone()
    }

    pub async fn is_server_healthy(&self, backend_name: &str, server_url: &str) -> bool {
        let health_status = self.health_status.read().await;
        
        if let Some(service_health) = health_status.get(backend_name) {
            for server_health in &service_health.servers {
                if server_health.url == server_url {
                    return server_health.status == HealthStatus::Healthy;
                }
            }
        }
        
        false
    }

    pub async fn get_healthy_servers(&self, backend_name: &str) -> Vec<String> {
        let health_status = self.health_status.read().await;
        
        if let Some(service_health) = health_status.get(backend_name) {
            return service_health
                .servers
                .iter()
                .filter(|server| server.status == HealthStatus::Healthy)
                .map(|server| server.url.clone())
                .collect();
        }
        
        Vec::new()
    }
} 