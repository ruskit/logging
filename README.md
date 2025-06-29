# Ruskit Logging

![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)
![Crate Status](https://img.shield## Dependencies

- `opentelemetry` (v0.30.0): Core OpenTelemetry API
- `opentelemetry_sdk` (v0.30.0): Implementation of the OpenTelemetry API
- `opentelemetry_stdout` (v0.30.0): OpenTelemetry exporter for standard output
- `opentelemetry_otlp` (v0.30.0): OpenTelemetry exporter for OTLP protocol
- `opentelemetry-appender-tracing` (v0.30.1): OpenTelemetry tracing bridge
- `tracing` (v0.1.41): Core tracing infrastructure
- `tracing-subscriber` (v0.3.19): Subscriber management and filtering
- `tracing-opentelemetry` (v0.31.0): OpenTelemetry integration for tracing
- `tracing-bunyan-formatter` (v0.3.10): JSON/Bunyan output format
- `configs`: Ruskit configuration management
- `thiserror` (v2.0.12): Error handlingstatus-stable-green.svg)

A structured logging library for Rust applications in the Ruskit framework, built on top of the `tracing` and `opentelemetry` ecosystems.

## Features

- **Multiple exporters**: Support for different logging backends
  - Standard output (stdout) for local development and simple deployments
  - OpenTelemetry Protocol (OTLP) over gRPC for distributed tracing and observability
- **Environment-aware formatting**: 
  - Pretty-printed logs for local development
  - JSON/Bunyan format for production environments
- **Intelligent filtering**: Automatically controls verbosity of common external dependencies
- **OpenTelemetry integration**: Seamless integration with the OpenTelemetry ecosystem
- **Feature-gated components**: Only include the exporters you need via Cargo features

## Installation

Add the logging crate to your `Cargo.toml`:

```toml
[dependencies]
configs = { git = "https://github.com/ruskit/configs.git", tag = "v0.0.1" }
logging = { git = "https://github.com/ruskit/logging.git", tag = "v0.0.1" }
```

### Available Features

- `stdout` - Enable the standard output exporter (default)
- `otlp` - Enable the OpenTelemetry Protocol (OTLP) over gRPC exporter
- `noop` - Enable the no-operation exporter (console only, no external export)

### Feature Priority

When multiple features are enabled, the priority order is:
1. **otlp**: Uses the OpenTelemetry OTLP gRPC exporter (highest priority)
2. **stdout**: Uses the standard output exporter
3. **noop**: Falls back to console-only logging (no external export)

## Usage

```rust
use logging::provider;
use tracing::{info, warn, error, debug, trace};

fn main() {
    // Initialize the logging system
    let _provider = provider::install().expect("Failed to initialize logging");
    
    // Now you can use tracing macros for logging
    info!("Application started");
    warn!("This is a warning message");
    error!("This is an error message");
    debug!("This is a debug message");
    trace!("This is a trace message");
    
    // You can also include structured data in your logs
    info!(
        user_id = "abc123", 
        request_path = "/api/users",
        "User request processed successfully"
    );
}
```

## Configuration

The logging library reads configuration from the Ruskit `configs` crate:

### For Stdout Exporter

The behavior is controlled through `AppConfigs`:

```rust
use configs::app::{AppConfigs, Environment};

let app_configs = AppConfigs {
    name: "my-service".to_string(),
    env: Environment::Local, // Affects output format (Local = pretty, others = JSON)
    // ... other fields
};
```

### For OTLP Exporter

When using the OTLP exporter, additional configuration is read from `OTLPConfigs`:

```rust
use configs::otlp::OTLPConfigs;

let otlp_configs = OTLPConfigs {
    endpoint: "http://localhost:4317".to_string(),
    exporter_timeout: std::time::Duration::from_secs(10),
    // ... other fields
};
```

## Log Filtering

The library automatically applies targeted filtering for common external libraries:

```
lapin, tower, h2, hyper, rustls, paho_mqtt, aws_* ...
```

These libraries are set to only show WARNING and higher severity logs, reducing noise in your application logs while maintaining your configured log level for your own code.

## Under the Hood

The Ruskit logging library creates an OpenTelemetry-compatible logging provider with the following components:

1. An `SdkLoggerProvider` configured with appropriate resources (service name, environment)
2. Environment-aware formatting (pretty or JSON/Bunyan)
3. Targeted filters for common external dependencies
4. OpenTelemetry bridge for tracing integration

## Dependencies

- `opentelemetry`: Core OpenTelemetry API
- `opentelemetry_sdk`: Implementation of the OpenTelemetry API
- `opentelemetry_stdout`: OpenTelemetry exporter for standard output
- `opentelemetry_otlp`: OpenTelemetry exporter for OTLP protocol
- `tracing`: Core tracing infrastructure
- `tracing-subscriber`: Subscriber management and filtering
- `tracing-bunyan-formatter`: JSON/Bunyan output format
- `configs`: Ruskit configuration management
- `thiserror`: Error handling

## License

MIT License - See [LICENSE](LICENSE) for details

## Ruskit Ecosystem

This crate is part of the [Ruskit](https://github.com/ruskit) ecosystem, which provides a modular toolkit for building robust Rust applications with built-in support for configuration management, structured logging, observability, and more.