// Copyright (c) 2025, The Ruskit Authors
// MIT License
// All rights reserved.

//! # OpenTelemetry Protocol (OTLP) gRPC Log Exporter
//!
//! This module provides functionality for exporting logs to an OpenTelemetry collector
//! using the OTLP protocol over gRPC. This enables integration with observability platforms
//! that support the OpenTelemetry standard, such as Jaeger, Prometheus, or commercial
//! observability solutions.
//!
//! The OTLP exporter sends logs in a standardized format, allowing for distributed tracing,
//! metrics collection, and log correlation across different services and applications.
//! It also configures local console/terminal output with formatting based on the environment.

use crate::{errors::LoggingError, exporters::filters::target_filters};
use configs::{app::AppConfigs, otlp::OTLPConfigs};
use opentelemetry::KeyValue;
use opentelemetry_appender_tracing::layer;
use opentelemetry_otlp::{Compression, LogExporter, Protocol, WithExportConfig, WithTonicConfig};
use opentelemetry_sdk::{Resource, logs::SdkLoggerProvider};
use tracing::error;
use tracing_bunyan_formatter::BunyanFormattingLayer;
use tracing_subscriber::{
    fmt::{
        Layer,
        format::{Format, Pretty},
    },
    layer::SubscriberExt,
    prelude::*,
};

/// Installs and configures the OpenTelemetry OTLP gRPC log exporter.
///
/// This function sets up a logging system that exports logs to an OpenTelemetry collector
/// using the OTLP protocol over gRPC, while also maintaining console output. The console
/// formatting depends on the environment:
/// - In local environments, logs are formatted in a pretty, human-readable format
/// - In non-local environments, logs are formatted as JSON in Bunyan format
///
/// It configures the OpenTelemetry exporter with gRPC protocol, Gzip compression,
/// and timeout settings from the OTLPConfigs.
///
/// # Returns
///
/// * `Result<SdkLoggerProvider, LoggingError>` - On success, returns the configured
///   OpenTelemetry logger provider. On failure, returns a `LoggingError`.
///
/// # Errors
///
/// Returns `LoggingError::InternalError` if there's a problem setting up the
/// log exporter or the tracing subscriber.
///
/// # Examples
///
/// ```
/// use logging::exporters::otlp_grpc;
///
/// fn main() {
///     let provider = otlp_grpc::install().expect("Failed to set up OTLP logging");
///     // Now logs will be written both to the console and sent to the OpenTelemetry collector
///     tracing::info!("Application started");
/// }
/// ```
pub fn install() -> Result<SdkLoggerProvider, LoggingError> {
    let app_cfgs = AppConfigs::new();
    let otlp_cfgs = OTLPConfigs::new();

    // Create the OTLP log exporter with gRPC configuration
    let exporter = match LogExporter::builder()
        .with_tonic()
        .with_protocol(Protocol::Grpc)
        .with_timeout(otlp_cfgs.exporter_timeout)
        .with_endpoint(otlp_cfgs.endpoint.clone())
        .with_compression(Compression::Gzip)
        .build()
    {
        Ok(exporter) => Ok(exporter),
        Err(err) => {
            error!(error = ?err, "failure to create log exporter");
            Err(LoggingError::InternalError {})
        }
    }?;

    // Configure the logger provider with service information
    let provider: SdkLoggerProvider = SdkLoggerProvider::builder()
        .with_resource(
            Resource::builder()
                .with_service_name(app_cfgs.name.clone())
                .with_attribute(KeyValue::new("environment", format!("{}", app_cfgs.name)))
                .with_attribute(KeyValue::new("library.language", "rust"))
                .build(),
        )
        .with_simple_exporter(exporter)
        .build();

    // Configure the base formatting layer with detailed metadata
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

    // Select the appropriate formatter based on environment
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

    // Configure filters and OpenTelemetry bridge
    let filters = target_filters(&app_cfgs.log_level);
    let otel_layer = layer::OpenTelemetryTracingBridge::new(&provider).with_filter(filters.clone());

    // Set up the global subscriber with all configured layers
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
        _ => {}
    }

    Ok(provider)
}
