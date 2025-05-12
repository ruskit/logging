use configs::AppConfigs;
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
pub fn get_log_level_filter(cfg: &AppConfigs) -> LevelFilter {
    match cfg.log_level.as_str() {
        "debug" | "Debug" | "DEBUG" => LevelFilter::DEBUG,
        "info" | "Info" | "INFO" => LevelFilter::INFO,
        "warn" | "Warn" | "WARN" => LevelFilter::WARN,
        "error" | "Error" | "ERROR" => LevelFilter::ERROR,
        "trace" | "Trace" | "TRACE" => LevelFilter::TRACE,
        _ => LevelFilter::OFF,
    }
}
