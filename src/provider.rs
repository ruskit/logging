// Copyright (c) 2025, The Ruskit Authors
// MIT License
// All rights reserved.

use crate::errors::LoggingError;
use opentelemetry_sdk::logs::SdkLoggerProvider;

#[cfg(any(feature = "otlp", feature = "stdout"))]
use crate::exporters;

pub fn install() -> Result<SdkLoggerProvider, LoggingError> {
    #[cfg(feature = "stdout")]
    {
        return exporters::stdout::install();
    }

    #[cfg(feature = "otlp")]
    {
        return exporters::stdout::install();
    }

    Err(LoggingError::InvalidFeaturesError)
}
