use crate::errors::LoggingError;
use opentelemetry_sdk::logs::SdkLoggerProvider;

#[cfg(any(feature = "otlp", feature = "stdout"))]
use crate::exporters;

pub fn install() -> Result<SdkLoggerProvider, LoggingError> {
    todo!()
}
