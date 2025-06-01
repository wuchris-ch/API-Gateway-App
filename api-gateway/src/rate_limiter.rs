use dashmap::DashMap;
use governor::{Quota, RateLimiter as GovernorRateLimiter};
use nonzero_ext::*;
use redis::AsyncCommands;
use std::{
    num::NonZeroU32,
    sync::Arc,
    time::{Duration, SystemTime, UNIX_EPOCH},
};
use tokio::sync::RwLock;
use tracing::{debug, error};

use crate::config::Config;

#[derive(Clone)]
pub struct RateLimiter {
    config: Arc<Config>,
    memory_limiters: Arc<DashMap<String, GovernorRateLimiter<String, dashmap::DashMap<String, governor::state::InMemoryState>, governor::clock::DefaultClock>>>,
    redis_client: Option<redis::Client>,
}

#[derive(Debug)]
pub enum RateLimitError {
    Exceeded,
    InternalError(String),
}

impl std::fmt::Display for RateLimitError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RateLimitError::Exceeded => write!(f, "Rate limit exceeded"),
            RateLimitError::InternalError(msg) => write!(f, "Rate limiter error: {}", msg),
        }
    }
}

impl std::error::Error for RateLimitError {}

impl RateLimiter {
    pub async fn new(config: Arc<Config>) -> anyhow::Result<Self> {
        let redis_client = if config.rate_limiting.storage == "redis" {
            Some(redis::Client::open(config.redis.url.as_str())?)
        } else {
            None
        };

        Ok(Self {
            config,
            memory_limiters: Arc::new(DashMap::new()),
            redis_client,
        })
    }

    pub async fn check_rate_limit(&self, client_id: &str) -> Result<(), RateLimitError> {
        if self.config.rate_limiting.storage == "redis" {
            self.check_rate_limit_redis(client_id).await
        } else {
            self.check_rate_limit_memory(client_id).await
        }
    }

    async fn check_rate_limit_memory(&self, client_id: &str) -> Result<(), RateLimitError> {
        let limiter = self.memory_limiters.entry(client_id.to_string()).or_insert_with(|| {
            let quota = Quota::per_minute(
                NonZeroU32::new(self.config.rate_limiting.default_requests_per_minute)
                    .unwrap_or(nonzero!(60u32))
            ).allow_burst(
                NonZeroU32::new(self.config.rate_limiting.burst_size)
                    .unwrap_or(nonzero!(10u32))
            );
            
            GovernorRateLimiter::dashmap(quota)
        });

        match limiter.check_key(client_id) {
            Ok(_) => {
                debug!("Rate limit check passed for client: {}", client_id);
                Ok(())
            }
            Err(_) => {
                debug!("Rate limit exceeded for client: {}", client_id);
                Err(RateLimitError::Exceeded)
            }
        }
    }

    async fn check_rate_limit_redis(&self, client_id: &str) -> Result<(), RateLimitError> {
        let redis_client = self.redis_client.as_ref()
            .ok_or_else(|| RateLimitError::InternalError("Redis client not configured".to_string()))?;

        let mut conn = redis_client.get_async_connection().await
            .map_err(|e| RateLimitError::InternalError(format!("Redis connection error: {}", e)))?;

        let key = format!("rate_limit:{}", client_id);
        let window_start = self.get_current_window_start();
        let window_key = format!("{}:{}", key, window_start);

        // Use Redis pipeline for atomic operations
        let (current_count,): (i32,) = redis::pipe()
            .incr(&window_key, 1)
            .expire(&window_key, 60) // 1 minute window
            .ignore()
            .get(&window_key)
            .query_async(&mut conn)
            .await
            .map_err(|e| RateLimitError::InternalError(format!("Redis query error: {}", e)))?;

        if current_count > self.config.rate_limiting.default_requests_per_minute as i32 {
            debug!("Rate limit exceeded for client: {} (count: {})", client_id, current_count);
            Err(RateLimitError::Exceeded)
        } else {
            debug!("Rate limit check passed for client: {} (count: {})", client_id, current_count);
            Ok(())
        }
    }

    fn get_current_window_start(&self) -> u64 {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        // Round down to the nearest minute
        now - (now % 60)
    }

    pub async fn get_rate_limit_status(&self, client_id: &str) -> Option<RateLimitStatus> {
        if self.config.rate_limiting.storage == "redis" {
            self.get_rate_limit_status_redis(client_id).await
        } else {
            self.get_rate_limit_status_memory(client_id).await
        }
    }

    async fn get_rate_limit_status_memory(&self, client_id: &str) -> Option<RateLimitStatus> {
        // For in-memory rate limiting, we can't easily get the current count
        // This is a limitation of the governor crate
        Some(RateLimitStatus {
            limit: self.config.rate_limiting.default_requests_per_minute,
            remaining: 0, // Unknown for memory-based limiting
            reset_time: 0, // Unknown for memory-based limiting
        })
    }

    async fn get_rate_limit_status_redis(&self, client_id: &str) -> Option<RateLimitStatus> {
        let redis_client = self.redis_client.as_ref()?;
        let mut conn = redis_client.get_async_connection().await.ok()?;

        let key = format!("rate_limit:{}", client_id);
        let window_start = self.get_current_window_start();
        let window_key = format!("{}:{}", key, window_start);

        let current_count: i32 = conn.get(&window_key).await.unwrap_or(0);
        let limit = self.config.rate_limiting.default_requests_per_minute;
        let remaining = if current_count < limit as i32 {
            limit - current_count as u32
        } else {
            0
        };

        Some(RateLimitStatus {
            limit,
            remaining,
            reset_time: window_start + 60, // Next minute
        })
    }
}

#[derive(Debug, Clone)]
pub struct RateLimitStatus {
    pub limit: u32,
    pub remaining: u32,
    pub reset_time: u64,
} 