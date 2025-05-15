// Copyright (c) 2025, The Ruskit Authors
// MIT License
// All rights reserved.

//! # Logging Exporters
//!
//! This module contains various exporters for the logging system. Each exporter
//! implements a different way to output logs, such as to standard output (stdout)
//! or to an OpenTelemetry collector via gRPC.
//!
//! The available exporters depend on which features are enabled when compiling
//! the crate:
//!
//! - **stdout**: Exports logs to the standard output
//! - **otlp**: Exports logs to an OpenTelemetry collector using gRPC
//!
//! This module also contains utilities for logging configuration, such as
//! environment variable handling and target filtering.

mod envs;
mod filters;

#[cfg(feature = "otlp")]
pub mod otlp_grpc;

#[cfg(feature = "stdout")]
pub mod stdout;
