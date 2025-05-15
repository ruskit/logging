// Copyright (c) 2025, The Ruskit Authors
// MIT License
// All rights reserved.

use super::envs::log_level;
use tracing::level_filters::LevelFilter;
use tracing_subscriber::filter::Targets;

#[allow(dead_code)]
pub fn target_filters(level: &str) -> Targets {
    let level_filter = log_level(level);

    Targets::new()
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
        .with_target("log", LevelFilter::WARN)
}
