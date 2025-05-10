// Copyright (c) 2025, The Ruskit Authors
// MIT License
// All rights reserved.

//! # Logger Implementation
//!
//! This module provides the core logging functionality for the Ruskit framework.
//!
//! It implements a configurable logging system using the `tracing` ecosystem, which
//! supports structured logging with different output formats based on the environment.
//! In local environments, it uses pretty-printed output, while in other environments
//! it uses JSON (Bunyan) format.
//!
//! The module also provides configuration for log filtering, allowing applications to
//! control the verbosity of logs from external crates.

use crate::errors::LoggingError;
use configs::{AppConfigs, Environment};
use tracing::warn;
use tracing_bunyan_formatter::BunyanFormattingLayer;
use tracing_log::LogTracer;
use tracing_subscriber::{
    filter::{LevelFilter, Targets},
    fmt::{
        Layer,
        format::{Format, Pretty},
    },
    layer::SubscriberExt,
};

/// Sets up the logging system based on the provided configuration.
///
/// This function initializes the global logging system for the application using
/// the settings specified in the application configuration. It configures:
///
/// - Log level filtering based on the `log_level` configuration
/// - Output format (pretty-printed for local environment, JSON/Bunyan for others)
/// - Target-specific log level filters for external crates
///
/// # Arguments
///
/// * `cfg` - The application configuration containing logging preferences
///
/// # Returns
///
/// A `Result` indicating success or failure in setting up the logging system
///
/// # Example
///
/// ```
/// use configs::AppConfigs;
/// use logging::setup;
///
/// let app_configs = AppConfigs::default();
/// setup(&app_configs).expect("Failed to set up logging");
/// ```
pub fn setup(cfg: &AppConfigs) -> Result<(), LoggingError> {
    match LogTracer::init() {
        Err(err) => {
            warn!(
                error = err.to_string(),
                "failure to initialize logger, probably the log was already initialized"
            );
            Ok(())
        }
        _ => Ok(()),
    }?;

    let level_filter = get_log_level_filter(cfg);

    let mut target_filters = Targets::new().with_default(level_filter);
    if !cfg.enable_external_creates_logging {
        target_filters = Targets::new()
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
            .with_target("log", LevelFilter::WARN);
    }

    let mut fmt_pretty: Option<Layer<_, Pretty, Format<Pretty>>> = None;
    let mut fmt_json = None;

    if cfg.env == Environment::Local {
        fmt_pretty = Some(Layer::new().pretty());
    } else {
        fmt_json = Some(BunyanFormattingLayer::new(
            cfg.name.to_owned(),
            std::io::stdout,
        ));
    }

    match tracing::subscriber::set_global_default(
        tracing_subscriber::registry()
            .with(fmt_json)
            .with(fmt_pretty)
            .with(target_filters),
    ) {
        Err(err) => {
            warn!(error = err.to_string(), "failure to set tracing subscribe");
            Err(LoggingError::InternalError {})
        }
        _ => Ok(()),
    }
}

/// Converts a string log level into a `LevelFilter` for tracing.
///
/// This function parses the log level string from the application configuration
/// and returns the corresponding `LevelFilter` enum value. It supports various
/// case formats (lowercase, uppercase, title case) for the log level names.
///
/// # Arguments
///
/// * `cfg` - The application configuration containing the `log_level` setting
///
/// # Returns
///
/// A `LevelFilter` corresponding to the configured log level. If the level
/// is not recognized, returns `LevelFilter::OFF`.
///
/// # Supported Log Levels
///
/// - "debug", "Debug", "DEBUG" -> `LevelFilter::DEBUG`
/// - "info", "Info", "INFO" -> `LevelFilter::INFO`
/// - "warn", "Warn", "WARN" -> `LevelFilter::WARN`
/// - "error", "Error", "ERROR" -> `LevelFilter::ERROR`
/// - "trace", "Trace", "TRACE" -> `LevelFilter::TRACE`
/// - Any other value -> `LevelFilter::OFF`
fn get_log_level_filter(cfg: &AppConfigs) -> LevelFilter {
    match cfg.log_level.as_str() {
        "debug" | "Debug" | "DEBUG" => LevelFilter::DEBUG,
        "info" | "Info" | "INFO" => LevelFilter::INFO,
        "warn" | "Warn" | "WARN" => LevelFilter::WARN,
        "error" | "Error" | "ERROR" => LevelFilter::ERROR,
        "trace" | "Trace" | "TRACE" => LevelFilter::TRACE,
        _ => LevelFilter::OFF,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn setup_successfully() {
        let res = setup(&AppConfigs::default());
        assert!(res.is_ok());
    }

    #[test]
    fn get_log_level_successfully() {
        let mut cfg = AppConfigs::default();

        cfg.log_level = "debug".to_owned();
        assert_eq!(get_log_level_filter(&cfg), LevelFilter::DEBUG);
        cfg.log_level = "Debug".to_owned();
        assert_eq!(get_log_level_filter(&cfg), LevelFilter::DEBUG);
        cfg.log_level = "DEBUG".to_owned();
        assert_eq!(get_log_level_filter(&cfg), LevelFilter::DEBUG);

        cfg.log_level = "info".to_owned();
        assert_eq!(get_log_level_filter(&cfg), LevelFilter::INFO);
        cfg.log_level = "Info".to_owned();
        assert_eq!(get_log_level_filter(&cfg), LevelFilter::INFO);
        cfg.log_level = "INFO".to_owned();
        assert_eq!(get_log_level_filter(&cfg), LevelFilter::INFO);

        cfg.log_level = "warn".to_owned();
        assert_eq!(get_log_level_filter(&cfg), LevelFilter::WARN);
        cfg.log_level = "Warn".to_owned();
        assert_eq!(get_log_level_filter(&cfg), LevelFilter::WARN);
        cfg.log_level = "WARN".to_owned();
        assert_eq!(get_log_level_filter(&cfg), LevelFilter::WARN);

        cfg.log_level = "error".to_owned();
        assert_eq!(get_log_level_filter(&cfg), LevelFilter::ERROR);
        cfg.log_level = "Error".to_owned();
        assert_eq!(get_log_level_filter(&cfg), LevelFilter::ERROR);
        cfg.log_level = "ERROR".to_owned();
        assert_eq!(get_log_level_filter(&cfg), LevelFilter::ERROR);

        cfg.log_level = "trace".to_owned();
        assert_eq!(get_log_level_filter(&cfg), LevelFilter::TRACE);
        cfg.log_level = "Trace".to_owned();
        assert_eq!(get_log_level_filter(&cfg), LevelFilter::TRACE);
        cfg.log_level = "TRACE".to_owned();
        assert_eq!(get_log_level_filter(&cfg), LevelFilter::TRACE);

        cfg.log_level = "UNKNOWN".to_owned();
        assert_eq!(get_log_level_filter(&cfg), LevelFilter::OFF);
    }
}
