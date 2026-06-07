use thiserror::Error;

/// Domain-specific errors with structured context.
///
/// All recoverable and non-recoverable errors the TUI can surface.
/// Each variant includes structured context fields for precise error reporting.
#[derive(Debug, Error)]
pub enum TuiError {
    #[error("catalog file not found: {path}")]
    CatalogNotFound { path: String },

    #[error("catalog parse error in {path} at byte {offset}: {detail}")]
    CatalogParse {
        path: String,
        offset: usize,
        detail: String,
    },

    #[error("catalog entry skipped in {path}: control byte at offset {offset} in field '{field}'")]
    TaintedEntry {
        path: String,
        offset: usize,
        field: String,
    },

    #[error("workspace not found: traversed to filesystem root from {start}")]
    WorkspaceNotFound { start: String },

    #[error("invalid workspace: {path} missing {missing}")]
    InvalidWorkspace { path: String, missing: String },

    #[error("subprocess failed: {command} exited with code {code}")]
    SubprocessFailed { command: String, code: i32 },

    #[error("subprocess timed out after {timeout_secs}s: {command}")]
    SubprocessTimeout { command: String, timeout_secs: u64 },

    #[error("validation rejected: {value} violates {rule}")]
    ValidationRejected { value: String, rule: String },

    #[error("path traversal rejected: {path}")]
    PathTraversal { path: String },

    #[error("terminal capability missing: {capability}")]
    TerminalCapability { capability: String },

    #[error("log destination unavailable: {path}: {reason}")]
    LogDestination { path: String, reason: String },
}

impl From<std::io::Error> for TuiError {
    fn from(err: std::io::Error) -> Self {
        // Map I/O errors to the most appropriate variant based on error kind.
        match err.kind() {
            std::io::ErrorKind::NotFound => TuiError::CatalogNotFound {
                path: err
                    .get_ref()
                    .map(|e| e.to_string())
                    .unwrap_or_else(|| "unknown".to_string()),
            },
            _ => TuiError::LogDestination {
                path: "unknown".to_string(),
                reason: err.to_string(),
            },
        }
    }
}

impl From<serde_json::Error> for TuiError {
    fn from(err: serde_json::Error) -> Self {
        TuiError::CatalogParse {
            path: "unknown".to_string(),
            offset: err.column(),
            detail: err.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_catalog_not_found_display() {
        let err = TuiError::CatalogNotFound {
            path: "/some/path/agents.json".to_string(),
        };
        assert_eq!(
            err.to_string(),
            "catalog file not found: /some/path/agents.json"
        );
    }

    #[test]
    fn test_catalog_parse_display() {
        let err = TuiError::CatalogParse {
            path: "catalog/agents.json".to_string(),
            offset: 42,
            detail: "unexpected token".to_string(),
        };
        assert_eq!(
            err.to_string(),
            "catalog parse error in catalog/agents.json at byte 42: unexpected token"
        );
    }

    #[test]
    fn test_tainted_entry_display() {
        let err = TuiError::TaintedEntry {
            path: "catalog/agents.json".to_string(),
            offset: 10,
            field: "summary".to_string(),
        };
        assert_eq!(
            err.to_string(),
            "catalog entry skipped in catalog/agents.json: control byte at offset 10 in field 'summary'"
        );
    }

    #[test]
    fn test_workspace_not_found_display() {
        let err = TuiError::WorkspaceNotFound {
            start: "/home/user/projects".to_string(),
        };
        assert_eq!(
            err.to_string(),
            "workspace not found: traversed to filesystem root from /home/user/projects"
        );
    }

    #[test]
    fn test_invalid_workspace_display() {
        let err = TuiError::InvalidWorkspace {
            path: "/home/user/repo".to_string(),
            missing: "catalog/agents.json".to_string(),
        };
        assert_eq!(
            err.to_string(),
            "invalid workspace: /home/user/repo missing catalog/agents.json"
        );
    }

    #[test]
    fn test_subprocess_failed_display() {
        let err = TuiError::SubprocessFailed {
            command: "npm run validate".to_string(),
            code: 1,
        };
        assert_eq!(
            err.to_string(),
            "subprocess failed: npm run validate exited with code 1"
        );
    }

    #[test]
    fn test_subprocess_timeout_display() {
        let err = TuiError::SubprocessTimeout {
            command: "npm run validate".to_string(),
            timeout_secs: 300,
        };
        assert_eq!(
            err.to_string(),
            "subprocess timed out after 300s: npm run validate"
        );
    }

    #[test]
    fn test_validation_rejected_display() {
        let err = TuiError::ValidationRejected {
            value: "../../etc/passwd".to_string(),
            rule: "path_traversal".to_string(),
        };
        assert_eq!(
            err.to_string(),
            "validation rejected: ../../etc/passwd violates path_traversal"
        );
    }

    #[test]
    fn test_path_traversal_display() {
        let err = TuiError::PathTraversal {
            path: "../../etc/passwd".to_string(),
        };
        assert_eq!(err.to_string(), "path traversal rejected: ../../etc/passwd");
    }

    #[test]
    fn test_terminal_capability_display() {
        let err = TuiError::TerminalCapability {
            capability: "256-color".to_string(),
        };
        assert_eq!(err.to_string(), "terminal capability missing: 256-color");
    }

    #[test]
    fn test_log_destination_display() {
        let err = TuiError::LogDestination {
            path: "/var/log/tui.log".to_string(),
            reason: "permission denied".to_string(),
        };
        assert_eq!(
            err.to_string(),
            "log destination unavailable: /var/log/tui.log: permission denied"
        );
    }

    #[test]
    fn test_from_io_error_not_found() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file missing");
        let tui_err: TuiError = io_err.into();
        match tui_err {
            TuiError::CatalogNotFound { path } => {
                assert_eq!(path, "file missing");
            }
            _ => panic!("expected CatalogNotFound variant"),
        }
    }

    #[test]
    fn test_from_io_error_other() {
        let io_err = std::io::Error::new(std::io::ErrorKind::PermissionDenied, "access denied");
        let tui_err: TuiError = io_err.into();
        match tui_err {
            TuiError::LogDestination { reason, .. } => {
                assert!(reason.contains("access denied"));
            }
            _ => panic!("expected LogDestination variant"),
        }
    }

    #[test]
    fn test_from_serde_json_error() {
        let json_err = serde_json::from_str::<serde_json::Value>("invalid json").unwrap_err();
        let tui_err: TuiError = json_err.into();
        match tui_err {
            TuiError::CatalogParse { detail, .. } => {
                assert!(!detail.is_empty());
            }
            _ => panic!("expected CatalogParse variant"),
        }
    }

    #[test]
    fn test_error_is_send_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<TuiError>();
    }
}
