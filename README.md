# Logging Crate

![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)
![Crate Status](https://img.shields.io/badge/status-stable-green.svg)

A structured logging library for Rust applications in the Ruskit framework, built on top of the `tracing` ecosystem.

## Features

- **Environment-aware formatting**: Pretty-printed logs for local development, JSON/Bunyan format for production
- **Configurable log levels**: Easily set log levels based on application configuration
- **External crate filtering**: Control the verbosity of logs from external dependencies
- **Seamless integration**: Works with Ruskit's configuration system out of the box

## Installation

Add the logging crate to your `Cargo.toml`:

```toml
[dependencies]
configs = { git = "https://github.com/ruskit/configs.git", tag = "v0.0.1" }
logging = { git = "https://github.com/ruskit/logging.git", tag = "v0.0.1" }
```

## Usage

The logging crate is designed to work with the Ruskit configuration system. Here's a basic example:

```rust
use configs::AppConfigs;
use logging;
use tracing::{info, warn, error, debug, trace};

fn main() {
    // Initialize application configs
    let app_configs = AppConfigs::default();
    
    // Set up structured logging
    logging::setup(&app_configs).expect("Failed to set up logging");
    
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
        "User request processed"
    );
}
```

### Configuration Options

The logging behavior can be controlled through the `AppConfigs` structure:

```rust
// Set log level (debug, info, warn, error, trace)
app_configs.log_level = "info".to_owned();

// Control external crate logs
app_configs.enable_external_creates_logging = false;

// Environment affects output format (Local = pretty, others = JSON)
app_configs.env = Environment::Local;
```

## How It Works

When you call `logging::setup(&app_configs)`, the library configures a global `tracing` subscriber based on your application configuration:

1. It initializes the log tracer to capture logs from the standard Rust `log` crate
2. Configures the log level filter based on your application configuration
3. Sets up target-specific filters for external crates when `enable_external_creates_logging` is false
4. Selects the appropriate output format (pretty-printed for local development, JSON/Bunyan for production)
5. Registers the global subscriber

## Log Levels

The crate supports the following log levels:

- **ERROR**: Only critical errors
- **WARN**: Warnings and errors 
- **INFO**: General information, warnings, and errors (default)
- **DEBUG**: Detailed debugging information plus info, warnings, and errors
- **TRACE**: The most verbose logging level

## Dependencies

- `configs`: Ruskit configuration management
- `tracing`: Core tracing infrastructure
- `tracing-subscriber`: Subscriber management and filtering
- `tracing-bunyan-formatter`: JSON/Bunyan output format
- `tracing-log`: Bridge between log and tracing crates
- `thiserror`: Error handling

## License

MIT License - See [LICENSE](LICENSE) for details

## Ruskit Ecosystem

This crate is part of the [Ruskit](https://github.com/ruskit) ecosystem, which provides a modular toolkit for building robust Rust applications with built-in support for configuration management, structured logging, secrets management, and more.