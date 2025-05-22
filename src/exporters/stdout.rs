// Copyright (c) 2025, The Ruskit Authors
// MIT License
// All rights reserved.

//! # Stdout Log Exporter
//!
//! This module provides functionality for exporting logs to standard output.
//! It configures a logging system that writes logs either in a pretty format
//! (for local development) or JSON/Bunyan format (for production environments).

use crate::{errors::LoggingError, exporters::filters::target_filters};
use configs::app::AppConfigs;
use opentelemetry::KeyValue;
use opentelemetry_appender_tracing::layer;
use opentelemetry_sdk::{Resource, logs::SdkLoggerProvider};
use opentelemetry_stdout::LogExporter;
use tracing::error;
use tracing_bunyan_formatter::BunyanFormattingLayer;
use tracing_log::LogTracer;
use tracing_subscriber::{
    fmt::{
        Layer,
        format::{Format, Pretty},
    },
    layer::SubscriberExt,
    prelude::*,
};

/// Installs and configures the stdout log exporter.
///
/// This function sets up a logging system that exports logs to standard output.
/// The formatting of logs depends on the environment:
/// - In local environments, logs are formatted in a pretty, human-readable format
/// - In non-local environments, logs are formatted as JSON in Bunyan format
///
/// It also configures OpenTelemetry integration and appropriate filtering
/// based on the application configuration.
///
/// # Returns
///
/// * `Result<SdkLoggerProvider, LoggingError>` - On success, returns the configured
///   OpenTelemetry logger provider. On failure, returns a `LoggingError`.
///
/// # Errors
///
/// Returns `LoggingError::InternalError` if there's a problem setting up the
/// tracing subscriber.
///
/// # Examples
///
/// ```
/// use logging::exporters::stdout;
///
/// fn main() {
///     let provider = stdout::install().expect("Failed to set up logging");
///     // Now logs will be written to stdout
///     tracing::info!("Application started");
/// }
/// ```
pub fn install() -> Result<SdkLoggerProvider, LoggingError> {
    let app_cfgs = AppConfigs::new();

    match LogTracer::init() {
        Err(err) => {
            error!(
                error = ?err,
                "failure to initialize logger, probably the log was already initialized"
            );
            Ok(())
        }
        _ => Ok(()),
    }?;

    let exporter = LogExporter::default();
    let provider: SdkLoggerProvider = SdkLoggerProvider::builder()
        .with_resource(
            Resource::builder()
                .with_service_name(app_cfgs.name.clone())
                .with_attribute(KeyValue::new(
                    "service.namespace",
                    format!("{}", app_cfgs.namespace),
                ))
                .with_attribute(KeyValue::new("environment", format!("{}", app_cfgs.env)))
                .with_attribute(KeyValue::new("library.language", "rust"))
                .build(),
        )
        .with_simple_exporter(exporter)
        .build();

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
    let otel_layer = layer::OpenTelemetryTracingBridge::new(&provider).with_filter(filters.clone());

    match tracing::subscriber::set_global_default(
        tracing_subscriber::registry()
            .with(otel_layer)
            .with(base_fmt_layer)
            .with(fmt_json)
            .with(fmt_pretty)
            .with(filters),
    ) {
        Err(err) => {
            error!(error = ?err, "failure to set tracing subscribe");
            return Err(LoggingError::InternalError {});
        }
        _ => Ok(provider),
    }
}
