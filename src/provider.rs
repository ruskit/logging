// Copyright (c) 2025, The Ruskit Authors
// MIT License
// All rights reserved.

//! # Logging Provider
//!
//! This module provides functionality for installing and configuring
//! the logging system. It acts as the main entry point for initializing
//! logging in applications using this crate.

use crate::errors::LoggingError;
use opentelemetry_sdk::logs::SdkLoggerProvider;

#[cfg(any(feature = "otlp", feature = "stdout"))]
use crate::exporters;

/// Installs and configures the logging system based on enabled features.
///
/// This function is the main entry point for initializing the logging system
/// in applications. It configures the appropriate logging exporter based on
/// the feature flags that were enabled when compiling the crate.
///
/// # Returns
///
/// * `Result<SdkLoggerProvider, LoggingError>` - On success, returns the configured
///   OpenTelemetry logger provider. On failure, returns a `LoggingError`.
///
/// # Errors
///
/// Returns `LoggingError::InvalidFeaturesError` if no supported exporter features
/// (like "stdout" or "otlp") are enabled.
///
/// # Examples
///
/// ```
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
    #[cfg(feature = "stdout")]
    {
        return exporters::stdout::install();
    }

    #[cfg(feature = "otlp")]
    {
        return exporters::stdout::install();
    }

    Err(LoggingError::InvalidFeaturesError)
}
