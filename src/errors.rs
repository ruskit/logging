// Copyright (c) 2025, The Ruskit Authors
// MIT License
// All rights reserved.

//! # Logging Errors
//!
//! This module defines the error types that can occur within the logging crate.
//!
//! The main error type is `LoggingError`, which represents different error conditions
//! that may occur during logging operations.

use thiserror::Error;

/// Errors that can occur during logging operations.
///
/// This enum uses the `thiserror` crate to provide detailed error messages
/// and simplify error handling throughout the application.
#[derive(Error, Debug, PartialEq, Eq)]
pub enum LoggingError {
    /// Represents an internal error in the logging system.
    ///
    /// This error is typically returned when there's a problem setting up the
    /// global tracing subscriber or other internal logging components.
    #[error("logging internal error")]
    InternalError,

    #[error("this exporter requires specific features")]
    InvalidFeaturesError,
}
