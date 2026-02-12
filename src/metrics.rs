use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use tokio::time::{interval, Duration};
use serde_json::json;

pub struct MetricsCollector {
    processed_count: AtomicU64,
    error_count: AtomicU64,
    connections_count: AtomicU64,
    start_time: std::time::Instant,
}

impl MetricsCollector {
    pub fn new() -> Self {
        Self {
            processed_count: AtomicU64::new(0),
            error_count: AtomicU64::new(0),
            connections_count: AtomicU64::new(0),
            start_time: std::time::Instant::now(),
        }
    }
    
    pub async fn increment_processed(&self) {
        self.processed_count.fetch_add(1, Ordering::Relaxed);
    }
    
    pub async fn increment_errors(&self) {
        self.error_count.fetch_add(1, Ordering::Relaxed);
    }
    
    pub async fn increment_connections(&self) {
        self.connections_count.fetch_add(1, Ordering::Relaxed);
    }
    
    pub fn get_metrics(&self) -> serde_json::Value {
        let uptime = self.start_time.elapsed().as_secs();
        let processed = self.processed_count.load(Ordering::Relaxed);
        let errors = self.error_count.load(Ordering::Relaxed);
        let connections = self.connections_count.load(Ordering::Relaxed);
        
        json!({
            "uptime_seconds": uptime,
            "logs_processed": processed,
            "errors": errors,
            "active_connections": connections,
            "throughput_per_second": if uptime > 0 { processed / uptime } else { 0 },
            "error_rate": if processed > 0 { (errors as f64 / processed as f64) * 100.0 } else { 0.0 }
        })
    }
    
    pub async fn start_collection(self: Arc<Self>) {
        let mut interval = interval(Duration::from_secs(10));
        
        loop {
            interval.tick().await;
            let metrics = self.get_metrics();
            println!("Metrics: {}", metrics);
            
            // Here you could send metrics to external systems
            // like Prometheus, InfluxDB, etc.
        }
    }
}