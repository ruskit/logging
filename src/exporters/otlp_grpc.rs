use super::level::get_log_level_filter;
use crate::errors::LoggingError;
use configs::{Configs, DynamicConfigs};
use opentelemetry::KeyValue;
use opentelemetry_appender_tracing::layer;
use opentelemetry_sdk::{Resource, logs::SdkLoggerProvider};
use tracing::level_filters::LevelFilter;
use tracing_subscriber::filter::Targets;

pub fn install<T>(cfg: &Configs<T>) -> Result<SdkLoggerProvider, LoggingError>
where
    T: DynamicConfigs,
{
    let exporter = opentelemetry_stdout::LogExporter::default();
    let provider: SdkLoggerProvider = SdkLoggerProvider::builder()
        .with_resource(
            Resource::builder()
                .with_service_name("log-appender-tracing-example")
                .build(),
        )
        .with_simple_exporter(exporter)
        .build();

    let level_filter = get_log_level_filter(&cfg.app);
    let target_filters = Targets::new()
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
        .with_target("log", LevelFilter::WARN);

    todo!()
}
