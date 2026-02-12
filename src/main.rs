use tokio::net::TcpListener;
use std::sync::Arc;
use tokio::sync::broadcast;
use serde_json::Value;

mod stream;
mod filter;
mod websocket;
mod metrics;

use stream::LogStreamer;
use websocket::WebSocketHandler;
use metrics::MetricsCollector;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (tx, _rx) = broadcast::channel(1000);
    let tx = Arc::new(tx);
    
    let metrics = Arc::new(MetricsCollector::new());
    let streamer = LogStreamer::new(tx.clone(), metrics.clone());
    
    // Start log ingestion server
    let listener = TcpListener::bind("0.0.0.0:8080").await?;
    println!("LogStreamer listening on port 8080");
    
    // Start WebSocket server for dashboard
    let ws_handler = WebSocketHandler::new(tx.subscribe());
    tokio::spawn(async move {
        ws_handler.start_server().await;
    });
    
    // Start metrics collection
    tokio::spawn(async move {
        metrics.start_collection().await;
    });
    
    // Accept incoming log connections
    loop {
        let (socket, addr) = listener.accept().await?;
        println!("New connection from: {}", addr);
        
        let streamer_clone = streamer.clone();
        tokio::spawn(async move {
            if let Err(e) = streamer_clone.handle_connection(socket).await {
                eprintln!("Connection error: {}", e);
            }
        });
    }
}