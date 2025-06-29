// Copyright (c) 2025, The Ruskit Authors
// MIT License
// All rights reserved.

//! # Ruskit Logging
//!
//! This crate provides a structured logging system for Rust applications within the Ruskit
//! framework. It's built on top of the `tracing` and `opentelemetry` ecosystems to provide
//! powerful logging capabilities with minimal configuration.
//!
//! The crate offers environment-aware formatting (pretty-printed logs for local development,
//! JSON/Bunyan format for production), configurable log levels, and filtering for external
//! dependencies.
//!
//! ## Features
//!
//! - **Configurable exporters**: Supports multiple logging backends (stdout, OTLP/gRPC)
//! - **Environment-aware formatting**: Format logs appropriately for different environments
//! - **Targeted filtering**: Control verbosity of external dependencies
//! - **OpenTelemetry integration**: Seamless integration with OpenTelemetry tracing
//!
//! ## Usage
//!
//! ```no_run
//! use logging::provider;
//!
//! fn main() {
//!     // Initialize the logging system
//!     let provider = provider::install().expect("Failed to initialize logging");
//!     
//!     // Use tracing macros for logging
//!     tracing::info!("Application started");
//!     
//!     // Structured logging
//!     tracing::info!(user_id = "123", "User logged in");
//! }
//! ```

pub mod errors;
pub mod exporters;
pub mod provider;
