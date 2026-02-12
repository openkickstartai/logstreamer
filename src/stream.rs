use tokio::net::TcpStream;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::sync::broadcast;
use std::sync::Arc;
use serde_json::{json, Value};
use chrono::Utc;
use crate::filter::LogFilter;
use crate::metrics::MetricsCollector;

#[derive(Clone)]
pub struct LogStreamer {
    sender: Arc<broadcast::Sender<Value>>,
    filter: LogFilter,
    metrics: Arc<MetricsCollector>,
}

impl LogStreamer {
    pub fn new(sender: Arc<broadcast::Sender<Value>>, metrics: Arc<MetricsCollector>) -> Self {
        Self {
            sender,
            filter: LogFilter::new(),
            metrics,
        }
    }
    
    pub async fn handle_connection(&self, stream: TcpStream) -> Result<(), Box<dyn std::error::Error>> {
        let mut reader = BufReader::new(stream);
        let mut line = String::new();
        
        loop {
            line.clear();
            match reader.read_line(&mut line).await {
                Ok(0) => break, // EOF
                Ok(_) => {
                    let log_entry = self.parse_log_line(&line).await;
                    
                    if self.filter.should_process(&log_entry) {
                        self.metrics.increment_processed().await;
                        
                        if let Err(_) = self.sender.send(log_entry) {
                            // No active receivers
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Read error: {}", e);
                    break;
                }
            }
        }
        
        Ok(())
    }
    
    async fn parse_log_line(&self, line: &str) -> Value {
        json!({
            "timestamp": Utc::now().to_rfc3339(),
            "message": line.trim(),
            "level": self.extract_log_level(line),
            "source": "tcp_stream"
        })
    }
    
    fn extract_log_level(&self, line: &str) -> String {
        let line_upper = line.to_uppercase();
        if line_upper.contains("ERROR") {
            "ERROR".to_string()
        } else if line_upper.contains("WARN") {
            "WARN".to_string()
        } else if line_upper.contains("INFO") {
            "INFO".to_string()
        } else {
            "DEBUG".to_string()
        }
    }
}