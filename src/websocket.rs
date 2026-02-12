use tokio::net::TcpListener;
use tokio_tungstenite::{accept_async, tungstenite::Message};
use tokio::sync::broadcast;
use serde_json::Value;
use futures_util::{SinkExt, StreamExt};

pub struct WebSocketHandler {
    log_receiver: broadcast::Receiver<Value>,
}

impl WebSocketHandler {
    pub fn new(log_receiver: broadcast::Receiver<Value>) -> Self {
        Self { log_receiver }
    }
    
    pub async fn start_server(mut self) {
        let listener = match TcpListener::bind("0.0.0.0:9001").await {
            Ok(listener) => listener,
            Err(e) => {
                eprintln!("Failed to bind WebSocket server: {}", e);
                return;
            }
        };
        
        println!("WebSocket dashboard server listening on port 9001");
        
        while let Ok((stream, addr)) = listener.accept().await {
            println!("WebSocket connection from: {}", addr);
            
            let mut receiver = self.log_receiver.resubscribe();
            
            tokio::spawn(async move {
                let ws_stream = match accept_async(stream).await {
                    Ok(ws) => ws,
                    Err(e) => {
                        eprintln!("WebSocket handshake failed: {}", e);
                        return;
                    }
                };
                
                let (mut ws_sender, mut ws_receiver) = ws_stream.split();
                
                // Handle incoming WebSocket messages (for filter updates)
                let ws_recv_task = tokio::spawn(async move {
                    while let Some(msg) = ws_receiver.next().await {
                        match msg {
                            Ok(Message::Text(text)) => {
                                println!("Received filter update: {}", text);
                                // TODO: Parse and apply filter updates
                            }
                            Ok(Message::Close(_)) => break,
                            Err(e) => {
                                eprintln!("WebSocket receive error: {}", e);
                                break;
                            }
                            _ => {}
                        }
                    }
                });
                
                // Forward log entries to WebSocket client
                let ws_send_task = tokio::spawn(async move {
                    while let Ok(log_entry) = receiver.recv().await {
                        let message = Message::Text(log_entry.to_string());
                        if ws_sender.send(message).await.is_err() {
                            break;
                        }
                    }
                });
                
                // Wait for either task to complete
                tokio::select! {
                    _ = ws_recv_task => {},
                    _ = ws_send_task => {},
                }
                
                println!("WebSocket connection closed");
            });
        }
    }
}