use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub routes: Vec<RouteConfig>,
    pub backends: HashMap<String, BackendConfig>,
    pub rate_limiting: RateLimitingConfig,
    pub auth: AuthConfig,
    pub redis: RedisConfig,
    pub database: DatabaseConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub workers: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteConfig {
    pub path: String,
    pub method: Option<String>,
    pub backend: String,
    pub load_balancing: LoadBalancingStrategy,
    pub rate_limit: Option<u32>,
    pub auth_required: bool,
    pub timeout_ms: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackendConfig {
    pub name: String,
    pub servers: Vec<String>,
    pub health_check: HealthCheckConfig,
    pub circuit_breaker: CircuitBreakerConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheckConfig {
    pub enabled: bool,
    pub path: String,
    pub interval_seconds: u64,
    pub timeout_seconds: u64,
    pub healthy_threshold: u32,
    pub unhealthy_threshold: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CircuitBreakerConfig {
    pub enabled: bool,
    pub failure_threshold: u32,
    pub recovery_timeout_seconds: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitingConfig {
    pub enabled: bool,
    pub default_requests_per_minute: u32,
    pub burst_size: u32,
    pub storage: String, // "memory" or "redis"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthConfig {
    pub enabled: bool,
    pub jwt_secret: String,
    pub api_key_header: String,
    pub bypass_paths: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedisConfig {
    pub url: String,
    pub pool_size: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LoadBalancingStrategy {
    RoundRobin,
    LeastConnections,
    Random,
    WeightedRoundRobin,
}

impl Config {
    pub fn load() -> anyhow::Result<Self> {
        // Try to load from environment variables first, then from file
        let config = if let Ok(config_str) = std::env::var("GATEWAY_CONFIG") {
            serde_json::from_str(&config_str)?
        } else {
            // Default configuration
            Self::default_config()
        };
        
        Ok(config)
    }
    
    fn default_config() -> Self {
        let mut backends = HashMap::new();
        
        backends.insert("backend_api".to_string(), BackendConfig {
            name: "Backend API".to_string(),
            servers: vec!["http://localhost:8000".to_string()],
            health_check: HealthCheckConfig {
                enabled: true,
                path: "/health".to_string(),
                interval_seconds: 30,
                timeout_seconds: 5,
                healthy_threshold: 2,
                unhealthy_threshold: 3,
            },
            circuit_breaker: CircuitBreakerConfig {
                enabled: true,
                failure_threshold: 5,
                recovery_timeout_seconds: 60,
            },
        });
        
        backends.insert("kong_gateway".to_string(), BackendConfig {
            name: "Kong Gateway".to_string(),
            servers: vec!["http://localhost:8000".to_string()],
            health_check: HealthCheckConfig {
                enabled: true,
                path: "/".to_string(),
                interval_seconds: 30,
                timeout_seconds: 5,
                healthy_threshold: 2,
                unhealthy_threshold: 3,
            },
            circuit_breaker: CircuitBreakerConfig {
                enabled: true,
                failure_threshold: 5,
                recovery_timeout_seconds: 60,
            },
        });
        
        Self {
            server: ServerConfig {
                host: "0.0.0.0".to_string(),
                port: 8080,
                workers: None,
            },
            routes: vec![
                RouteConfig {
                    path: "/api/v1/*".to_string(),
                    method: None,
                    backend: "backend_api".to_string(),
                    load_balancing: LoadBalancingStrategy::RoundRobin,
                    rate_limit: Some(100),
                    auth_required: true,
                    timeout_ms: Some(30000),
                },
                RouteConfig {
                    path: "/auth/*".to_string(),
                    method: None,
                    backend: "backend_api".to_string(),
                    load_balancing: LoadBalancingStrategy::RoundRobin,
                    rate_limit: Some(50),
                    auth_required: false,
                    timeout_ms: Some(10000),
                },
                RouteConfig {
                    path: "/public/*".to_string(),
                    method: None,
                    backend: "backend_api".to_string(),
                    load_balancing: LoadBalancingStrategy::RoundRobin,
                    rate_limit: Some(200),
                    auth_required: false,
                    timeout_ms: Some(15000),
                },
            ],
            backends,
            rate_limiting: RateLimitingConfig {
                enabled: true,
                default_requests_per_minute: 60,
                burst_size: 10,
                storage: "memory".to_string(),
            },
            auth: AuthConfig {
                enabled: true,
                jwt_secret: "your-jwt-secret-key".to_string(),
                api_key_header: "X-API-Key".to_string(),
                bypass_paths: vec![
                    "/health".to_string(),
                    "/metrics".to_string(),
                    "/auth/login".to_string(),
                    "/public/*".to_string(),
                ],
            },
            redis: RedisConfig {
                url: "redis://localhost:6379".to_string(),
                pool_size: 10,
            },
            database: DatabaseConfig {
                url: "postgresql://postgres:postgres@localhost:5432/api_gateway".to_string(),
                max_connections: 10,
            },
        }
    }
} 