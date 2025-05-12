use tracing::level_filters::LevelFilter;

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
pub fn log_level() -> LevelFilter {
    let level = std::env::var("LOG_LEVEL")
        .unwrap_or_else(|_| "info".to_string())
        .to_lowercase();

    match level.as_str() {
        "debug" | "Debug" | "DEBUG" => LevelFilter::DEBUG,
        "info" | "Info" | "INFO" => LevelFilter::INFO,
        "warn" | "Warn" | "WARN" => LevelFilter::WARN,
        "error" | "Error" | "ERROR" => LevelFilter::ERROR,
        "trace" | "Trace" | "TRACE" => LevelFilter::TRACE,
        _ => LevelFilter::OFF,
    }
}

pub fn app_name() -> String {
    std::env::var("APP_NAME").unwrap_or_else(|_| "default-app-name".to_string())
}

pub fn otlp_exporter_host() -> String {
    std::env::var("OTLP_EXPORTER_HOST").unwrap_or_else(|_| "localhost:4317".to_string())
}
