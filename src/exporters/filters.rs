// Copyright (c) 2025, The Ruskit Authors
// MIT License
// All rights reserved.

//! # Log Filtering
//!
//! This module provides functionality for filtering log messages based on their
//! target and level, allowing for fine-grained control over what gets logged.

use super::envs::log_level;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::filter::Targets;

/// Creates a configured target filter for controlling log output verbosity.
///
/// This function creates a `Targets` filter that controls which log messages are
/// displayed based on their target (module path) and level. It sets a default
/// log level based on the provided `level` string, and then configures specific
/// filters for common external dependencies to reduce their verbosity.
///
/// # Arguments
///
/// * `level` - A string representing the desired log level (e.g., "info", "debug").
///   This will be used as the default level for all targets not explicitly configured.
///
/// # Returns
///
/// A `Targets` filter configured with appropriate log levels for various targets.
///
/// # Examples
///
/// ```
/// use logging::exporters::filters;
///
/// let filter = filters::target_filters("info");
/// // The filter is now configured with INFO level as default
/// // and WARNING level for external dependencies
/// ```
#[allow(dead_code)]
pub fn target_filters(level: &str) -> Targets {
    let level_filter = log_level(level);

    Targets::new()
        .with_default(level_filter)
        .with_target("lapin", LevelFilter::WARN)
        .with_target("tower", LevelFilter::WARN)
        .with_target("h2", LevelFilter::WARN)
        .with_target("hyper", LevelFilter::WARN)
        .with_target("rustls", LevelFilter::WARN)
        .with_target("paho_mqtt", LevelFilter::WARN)
        .with_target("c_trace", LevelFilter::WARN)
        .with_target("aws_smithy_runtime", LevelFilter::WARN)
        .with_target("aws_config", LevelFilter::WARN)
        .with_target("aws_sdk_secretsmanager", LevelFilter::WARN)
        .with_target("aws_runtime", LevelFilter::WARN)
        .with_target("opentelemetry_sdk", LevelFilter::WARN)
}
