use std::path::PathBuf;

/// All recoverable and non-recoverable errors the TUI can surface.
#[derive(Debug, thiserror::Error)]
pub enum TuiError {
    #[error("catalog not found at {path}")]
    CatalogNotFound { path: PathBuf },

    #[error("catalog parse error in {path} at offset {offset}: {detail}")]
    CatalogParse {
        path: PathBuf,
        offset: usize,
        detail: String,
    },

    #[error("tainted entry in {path} at offset {offset}, field: {field}")]
    TaintedEntry {
        path: PathBuf,
        offset: usize,
        field: String,
    },

    #[error("workspace not found starting from {start}")]
    WorkspaceNotFound { start: PathBuf },

    #[error("invalid workspace at {path}: missing {missing}")]
    InvalidWorkspace { path: PathBuf, missing: String },

    #[error("subprocess failed: {command} exited with code {code}")]
    SubprocessFailed { command: String, code: i32 },

    #[error("subprocess timeout: {command} exceeded {timeout_secs}s")]
    SubprocessTimeout { command: String, timeout_secs: u64 },

    #[error("validation rejected value {value:?}: rule {rule}")]
    ValidationRejected { value: String, rule: String },

    #[error("path traversal detected: {path}")]
    PathTraversal { path: PathBuf },

    #[error("terminal capability unavailable: {capability}")]
    TerminalCapability { capability: String },

    #[error("log destination error at {path}: {reason}")]
    LogDestination { path: PathBuf, reason: String },
}
