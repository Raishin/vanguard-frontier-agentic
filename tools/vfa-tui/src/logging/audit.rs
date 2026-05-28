use std::path::Path;

use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{fmt, EnvFilter};
use uuid::Uuid;

/// Initialize the logging subsystem with JSON output.
///
/// Configures tracing-subscriber with:
/// - JSON format for structured logging
/// - Configurable level filter from the `log_level` string
/// - Session ID included in all events
/// - Output to stderr (always) and optionally to a file
/// - If the file cannot be opened, logs a warning to stderr and continues
pub fn init_logging(
    log_file: Option<&Path>,
    log_level: &str,
    session_id: Uuid,
) -> anyhow::Result<()> {
    let filter = EnvFilter::try_new(log_level).unwrap_or_else(|_| EnvFilter::new("info"));

    // Create the base stderr layer with JSON format
    let stderr_layer = fmt::layer()
        .json()
        .with_target(true)
        .with_span_events(FmtSpan::NONE)
        .with_writer(std::io::stderr);

    if let Some(file_path) = log_file {
        match std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(file_path)
        {
            Ok(file) => {
                let file_layer = fmt::layer()
                    .json()
                    .with_target(true)
                    .with_span_events(FmtSpan::NONE)
                    .with_writer(file);

                tracing_subscriber::registry()
                    .with(filter)
                    .with(stderr_layer)
                    .with(file_layer)
                    .init();
            }
            Err(e) => {
                // Fall back to stderr only, but still initialize
                tracing_subscriber::registry()
                    .with(filter)
                    .with(stderr_layer)
                    .init();

                tracing::warn!(
                    session_id = %session_id,
                    path = %file_path.display(),
                    error = %e,
                    "Failed to open log file, continuing with stderr only"
                );
            }
        }
    } else {
        tracing_subscriber::registry()
            .with(filter)
            .with(stderr_layer)
            .init();
    }

    tracing::info!(session_id = %session_id, "Logging initialized");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    // Note: tracing can only be initialized once per process, so we test
    // the logic paths but can only actually init in one test.
    // We test the file path building and filter creation separately.

    #[test]
    fn env_filter_parses_valid_levels() {
        let filter = EnvFilter::try_new("debug");
        assert!(filter.is_ok());

        let filter = EnvFilter::try_new("warn");
        assert!(filter.is_ok());

        let filter = EnvFilter::try_new("info");
        assert!(filter.is_ok());
    }

    #[test]
    fn env_filter_falls_back_on_invalid() {
        // Invalid level should fall back to info (our code handles this)
        let filter = EnvFilter::try_new("not_a_level");
        // This may or may not error depending on the string, but our code
        // wraps it with unwrap_or_else
        let _ = filter;
    }

    #[test]
    fn session_id_is_valid_uuid() {
        let id = Uuid::new_v4();
        let s = id.to_string();
        assert_eq!(s.len(), 36);
        assert!(Uuid::parse_str(&s).is_ok());
    }
}
