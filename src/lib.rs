// Copyright (c) 2025, The Ruskit Authors
// MIT License
// All rights reserved.

//! # Ruskit Logging
//!
//! A structured logging library for Rust applications in the Ruskit framework.
//!
//! This crate provides a simple yet powerful interface for configuring application logging,
//! using the `tracing` ecosystem internally. It supports different log formats based on the
//! environment (pretty formatting for local development and JSON/Bunyan for production).
//!
//! ## Example
//!
//! ```rust
//! use logging;
//! use configs::AppConfigs;
//!
//! fn main() {
//!     let app_configs = AppConfigs::default();
//!     logging::setup(&app_configs).expect("Failed to set up logging");
//!     
//!     // Now you can use tracing macros for logging
//!     tracing::info!("Application started");
//! }
//! ```

pub mod errors;
mod logger;

pub use logger::setup;
