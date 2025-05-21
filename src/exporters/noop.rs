// Copyright (c) 2025, The Ruskit Authors
// MIT License
// All rights reserved.

//! # No-Operation (Noop) Log Exporter
//!
//! This module provides a logging exporter that doesn't send logs to any external system,
//! but still maintains proper console/terminal output. It's useful for development,
//! testing, or situations where you don't need or want to export logs to an external
//! observability system, but still want formatted logs in your terminal.
//!
//! The noop exporter still configures the tracing subscriber with appropriate formatting
//! based on the environment (pretty for local development, JSON/Bunyan for production),
//! but it doesn't set up any OpenTelemetry export bridges.

use crate::{errors::LoggingError, exporters::filters::target_filters};
use configs::app::AppConfigs;
use opentelemetry_sdk::logs::{LoggerProviderBuilder, SdkLoggerProvider};
use tracing::error;
use tracing_bunyan_formatter::BunyanFormattingLayer;
use tracing_subscriber::{
    fmt::{
        Layer,
        format::{Format, Pretty},
    },
    layer::SubscriberExt,
};

/// Installs and configures the no-operation (noop) log exporter.
///
/// This function sets up a logging system that formats and outputs logs to the console
/// but doesn't export them to any external system. It configures the formatting based on
/// the environment:
/// - In local environments, logs are formatted in a pretty, human-readable format
/// - In non-local environments, logs are formatted as JSON in Bunyan format
///
/// Unlike other exporters, the noop exporter doesn't set up any OpenTelemetry bridges,
/// making it lightweight and suitable for development or scenarios where external
/// observability systems aren't needed.
///
/// # Returns
///
/// * `Result<SdkLoggerProvider, LoggingError>` - On success, returns a default
///   OpenTelemetry logger provider with no exporters configured. On failure,
///   returns a `LoggingError`.
///
/// # Errors
///
/// Returns `LoggingError::InternalError` if there's a problem setting up the
/// tracing subscriber.
///
/// # Examples
///
/// ```
/// use logging::exporters::noop;
///
/// fn main() {
///     let provider = noop::install().expect("Failed to set up logging");
///     // Now logs will be written to the console but not exported
///     tracing::info!("Application started");
/// }
/// ```
pub fn install() -> Result<SdkLoggerProvider, LoggingError> {
    let app_cfgs = AppConfigs::new();

    let base_fmt_layer = tracing_subscriber::fmt::layer()
        .with_writer(std::io::stderr)
        .event_format(
            tracing_subscriber::fmt::format()
                .with_thread_ids(true)
                .with_thread_names(true)
                .with_ansi(app_cfgs.env.is_local())
                .with_level(true)
                .with_target(true)
                .with_file(true)
                .with_line_number(true)
                .with_source_location(true)
                .compact(),
        );

    let mut fmt_pretty: Option<Layer<_, Pretty, Format<Pretty>>> = None;
    let mut fmt_json = None;
    if app_cfgs.env.is_local() {
        fmt_pretty = Some(Layer::new().pretty());
    } else {
        fmt_json = Some(BunyanFormattingLayer::new(
            app_cfgs.name.clone(),
            std::io::stdout,
        ));
    }

    let filters = target_filters(&app_cfgs.log_level);

    match tracing::subscriber::set_global_default(
        tracing_subscriber::registry()
            .with(base_fmt_layer)
            .with(fmt_json)
            .with(fmt_pretty)
            .with(filters),
    ) {
        Err(err) => {
            error!(error = ?err, "failure to set tracing subscribe");
            return Err(LoggingError::InternalError {});
        }
        _ => Ok(LoggerProviderBuilder::default().build()),
    }
}
