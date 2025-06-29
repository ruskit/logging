// Copyright (c) 2025, The Ruskit Authors
// MIT License
// All rights reserved.

//! # Logging Provider
//!
//! This module provides functionality for installing and configuring
//! the logging system. It acts as the main entry point for initializing
//! logging in applications using this crate.

use crate::errors::LoggingError;
use crate::exporters;
use opentelemetry_sdk::logs::SdkLoggerProvider;

/// Installs and configures the logging system based on enabled features.
///
/// This function is the main entry point for initializing the logging system
/// in applications. It configures the appropriate logging exporter based on
/// the feature flags that were enabled when compiling the crate.
///
/// ## Feature Priority
///
/// When multiple features are enabled, the priority order is:
/// 1. **otlp**: Uses the OpenTelemetry OTLP gRPC exporter (highest priority)
/// 2. **stdout**: Uses the standard output exporter
/// 3. **none**: Falls back to the noop exporter (no external export, console only)
///
/// # Returns
///
/// * `Result<SdkLoggerProvider, LoggingError>` - On success, returns the configured
///   OpenTelemetry logger provider. On failure, returns a `LoggingError`.
///
/// # Errors
///
/// Returns `LoggingError::InternalError` if there's a problem setting up the
/// chosen exporter.
///
/// # Examples
///
/// ```no_run
/// use logging::provider;
///
/// fn main() {
///     // Initialize the logging system
///     let provider = provider::install().expect("Failed to initialize logging");
///     
///     // Now you can use tracing macros for logging
///     tracing::info!("Application started");
/// }
/// ```
pub fn install() -> Result<SdkLoggerProvider, LoggingError> {
    // Prioritize OTLP over stdout if both are enabled
    #[cfg(feature = "otlp")]
    {
        println!("Using OTLP exporter for logging");
        return exporters::otlp_grpc::install();
    }

    #[cfg(all(feature = "stdout", not(feature = "otlp")))]
    {
        println!("Using stdout exporter for logging");
        return exporters::stdout::install();
    }

    #[cfg(not(any(feature = "stdout", feature = "otlp")))]
    {
        println!("No supported logging exporter features enabled. Using noop exporter.");
        return exporters::noop::install();
    }
}
