//! Tracing setup shared by the services: one call at start, naming the
//! service. The level comes from `RUST_LOG`, `info` when unset.

use tracing_subscriber::{fmt, EnvFilter};

/// How log lines look.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Format {
    /// For a person reading a terminal.
    Pretty,
    /// One JSON object per line, for a log pipeline.
    Json,
}

/// Install the global subscriber for `service`.
pub fn init(service: &str, format: Format) {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    match format {
        Format::Pretty => fmt().with_env_filter(filter).pretty().init(),
        Format::Json => fmt().with_env_filter(filter).json().init(),
    }
    tracing::info!(service, "logging started");
}
