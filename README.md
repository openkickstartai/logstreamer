# LogStreamer

High-performance real-time log streaming service built in Rust with advanced filtering, alerting, and dashboard visualization capabilities.

## Features

- Real-time log ingestion via TCP (port 8080)
- WebSocket-based live dashboard (port 9001)
- Customizable regex and field-based filters
- High-throughput log processing with async I/O
- Built-in metrics collection and analytics
- Configurable alerting system

## Quick Start

```bash
# Build the project
cargo build --release

# Run the log streamer
cargo run
```

## Usage

### Sending Logs
Connect to port 8080 and send newline-delimited log messages:

```bash
echo "ERROR: Database connection failed" | nc localhost 8080
```

### Dashboard
Connect to the WebSocket dashboard at `ws://localhost:9001` to receive real-time log streams.

### Architecture

- `main.rs`: Entry point and server orchestration
- `stream.rs`: Core log streaming and parsing logic
- `filter.rs`: Advanced filtering engine with regex support
- `websocket.rs`: WebSocket server for dashboard connectivity
- `metrics.rs`: Performance metrics and analytics collection

## Performance

Built with Tokio async runtime for maximum throughput and minimal resource usage. Supports thousands of concurrent connections with efficient memory management.