// Copyright (c) 2025, The Ruskit Authors
// MIT License
// All rights reserved.

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

pub fn install() -> Result<SdkLoggerProvider, LoggingError> {
    let app_cfgs = AppConfigs::new();
    let otlp_cfgs = OTLPConfigs::new();

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

    let base_fmt_layer = tracing_subscriber::fmt::layer()
        .with_writer(std::io::stderr)
        .event_format(
            tracing_subscriber::fmt::format()
                .with_thread_ids(true)
                .with_thread_names(true)
                .with_ansi(app_cfgs.env.is_local())
                .with_level(true)
                .with_target(true)
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

    let filters = target_filters(&app_cfgs.env.to_string());
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
        _ => {}
    }

    Ok(provider)
}
