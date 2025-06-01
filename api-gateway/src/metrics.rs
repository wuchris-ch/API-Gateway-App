use prometheus::{Counter, Histogram, Registry, Encoder, TextEncoder};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    sync::Arc,
    time::Duration,
};
use tokio::sync::RwLock;
use lazy_static::lazy_static;

lazy_static! {
    static ref REGISTRY: Registry = Registry::new();
    static ref REQUEST_COUNTER: Counter = Counter::new("gateway_requests_total", "Total number of requests").unwrap();
    static ref REQUEST_DURATION: Histogram = Histogram::new("gateway_request_duration_seconds", "Request duration in seconds").unwrap();
    static ref ERROR_COUNTER: Counter = Counter::new("gateway_errors_total", "Total number of errors").unwrap();
    static ref BACKEND_REQUEST_COUNTER: Counter = Counter::new("gateway_backend_requests_total", "Total number of backend requests").unwrap();
}

#[derive(Clone)]
pub struct MetricsCollector {
    custom_metrics: Arc<RwLock<HashMap<String, CustomMetric>>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomMetric {
    pub name: String,
    pub value: f64,
    pub labels: HashMap<String, String>,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricsSummary {
    pub total_requests: u64,
    pub total_errors: u64,
    pub average_response_time_ms: f64,
    pub requests_per_second: f64,
    pub error_rate: f64,
    pub backend_status: HashMap<String, BackendMetrics>,
    pub custom_metrics: Vec<CustomMetric>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackendMetrics {
    pub total_requests: u64,
    pub healthy_servers: u32,
    pub total_servers: u32,
    pub average_response_time_ms: f64,
}

impl MetricsCollector {
    pub fn new() -> Self {
        // Register metrics with Prometheus
        REGISTRY.register(Box::new(REQUEST_COUNTER.clone())).unwrap();
        REGISTRY.register(Box::new(REQUEST_DURATION.clone())).unwrap();
        REGISTRY.register(Box::new(ERROR_COUNTER.clone())).unwrap();
        REGISTRY.register(Box::new(BACKEND_REQUEST_COUNTER.clone())).unwrap();

        Self {
            custom_metrics: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn record_request(&self, method: &str, path: &str) {
        REQUEST_COUNTER.inc();
        
        // Record custom metric for method/path combination
        let metric_name = format!("requests_{}_{}", method.to_lowercase(), sanitize_path(path));
        self.increment_custom_metric(&metric_name, 1.0, HashMap::new()).await;
    }

    pub async fn record_response_time(&self, duration: Duration) {
        REQUEST_DURATION.observe(duration.as_secs_f64());
        
        // Record custom metric for response time
        let mut labels = HashMap::new();
        labels.insert("unit".to_string(), "milliseconds".to_string());
        
        self.set_custom_metric(
            "response_time_ms",
            duration.as_millis() as f64,
            labels,
        ).await;
    }

    pub async fn record_error(&self, error_type: &str) {
        ERROR_COUNTER.inc();
        
        // Record custom metric for error type
        let mut labels = HashMap::new();
        labels.insert("error_type".to_string(), error_type.to_string());
        
        self.increment_custom_metric("errors", 1.0, labels).await;
    }

    pub async fn record_backend_request(&self, backend_name: &str, success: bool, response_time: Duration) {
        BACKEND_REQUEST_COUNTER.inc();
        
        let mut labels = HashMap::new();
        labels.insert("backend".to_string(), backend_name.to_string());
        labels.insert("success".to_string(), success.to_string());
        
        self.increment_custom_metric("backend_requests", 1.0, labels.clone()).await;
        
        // Record backend response time
        labels.insert("unit".to_string(), "milliseconds".to_string());
        self.set_custom_metric(
            &format!("backend_response_time_{}", backend_name),
            response_time.as_millis() as f64,
            labels,
        ).await;
    }

    pub async fn set_custom_metric(&self, name: &str, value: f64, labels: HashMap<String, String>) {
        let mut metrics = self.custom_metrics.write().await;
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        metrics.insert(
            name.to_string(),
            CustomMetric {
                name: name.to_string(),
                value,
                labels,
                timestamp,
            },
        );
    }

    pub async fn increment_custom_metric(&self, name: &str, increment: f64, labels: HashMap<String, String>) {
        let mut metrics = self.custom_metrics.write().await;
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let metric = metrics.entry(name.to_string()).or_insert(CustomMetric {
            name: name.to_string(),
            value: 0.0,
            labels: labels.clone(),
            timestamp,
        });

        metric.value += increment;
        metric.timestamp = timestamp;
        metric.labels = labels;
    }

    pub async fn get_metrics(&self) -> MetricsSummary {
        let custom_metrics = self.custom_metrics.read().await;
        
        // Calculate summary statistics
        let total_requests = REQUEST_COUNTER.get() as u64;
        let total_errors = ERROR_COUNTER.get() as u64;
        let error_rate = if total_requests > 0 {
            (total_errors as f64 / total_requests as f64) * 100.0
        } else {
            0.0
        };

        // Get average response time from custom metrics
        let average_response_time_ms = custom_metrics
            .get("response_time_ms")
            .map(|m| m.value)
            .unwrap_or(0.0);

        // Calculate requests per second (simplified - would need time window in production)
        let requests_per_second = total_requests as f64 / 60.0; // Rough estimate

        // Collect backend metrics
        let mut backend_status = HashMap::new();
        for (name, metric) in custom_metrics.iter() {
            if name.starts_with("backend_requests") {
                if let Some(backend_name) = metric.labels.get("backend") {
                    let backend_metrics = backend_status.entry(backend_name.clone()).or_insert(BackendMetrics {
                        total_requests: 0,
                        healthy_servers: 0,
                        total_servers: 0,
                        average_response_time_ms: 0.0,
                    });
                    
                    backend_metrics.total_requests += metric.value as u64;
                }
            }
        }

        MetricsSummary {
            total_requests,
            total_errors,
            average_response_time_ms,
            requests_per_second,
            error_rate,
            backend_status,
            custom_metrics: custom_metrics.values().cloned().collect(),
        }
    }

    pub fn get_prometheus_metrics(&self) -> String {
        let encoder = TextEncoder::new();
        let metric_families = REGISTRY.gather();
        
        match encoder.encode_to_string(&metric_families) {
            Ok(metrics) => metrics,
            Err(e) => {
                eprintln!("Failed to encode metrics: {}", e);
                String::new()
            }
        }
    }

    pub async fn reset_metrics(&self) {
        let mut custom_metrics = self.custom_metrics.write().await;
        custom_metrics.clear();
        
        // Note: Prometheus metrics cannot be reset easily
        // In production, you might want to use a different approach
    }

    pub async fn get_metric(&self, name: &str) -> Option<CustomMetric> {
        let custom_metrics = self.custom_metrics.read().await;
        custom_metrics.get(name).cloned()
    }

    pub async fn get_metrics_by_label(&self, label_key: &str, label_value: &str) -> Vec<CustomMetric> {
        let custom_metrics = self.custom_metrics.read().await;
        
        custom_metrics
            .values()
            .filter(|metric| {
                metric.labels.get(label_key).map(|v| v == label_value).unwrap_or(false)
            })
            .cloned()
            .collect()
    }
}

fn sanitize_path(path: &str) -> String {
    path.replace('/', "_")
        .replace('-', "_")
        .replace('.', "_")
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == '_')
        .collect()
} 