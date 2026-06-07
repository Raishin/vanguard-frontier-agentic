use std::path::PathBuf;

use clap::{Parser, ValueEnum};

/// Enterprise-grade TUI for the Vanguard Frontier Agentic marketplace catalog.
#[derive(Debug, Parser)]
#[command(name = "vfa-tui", version, about)]
pub struct Cli {
    /// Path to the workspace root (auto-detected if omitted).
    #[arg(long)]
    pub workspace: Option<PathBuf>,

    /// Path to the log file for audit output.
    #[arg(long)]
    pub log_file: Option<PathBuf>,

    /// Logging verbosity level.
    #[arg(long, value_enum, default_value_t = LogLevel::Info)]
    pub log_level: LogLevel,

    /// Disable colored output.
    #[arg(long)]
    pub no_color: bool,
}

/// Supported log levels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}
