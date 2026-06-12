use thiserror::Error;

/// Domain-specific errors with structured context.
///
/// All recoverable and non-recoverable errors the TUI can surface.
/// Each variant includes structured context fields for precise error reporting.
#[derive(Debug, Error)]
pub enum TuiError {
    // === Catalog Errors ===
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

    // === Workspace Errors ===
    #[error("workspace not found: traversed to filesystem root from {start}")]
    WorkspaceNotFound { start: String },

    #[error("invalid workspace: {path} missing {missing}")]
    InvalidWorkspace { path: String, missing: String },

    // === Registry Errors ===
    /// Workspace registry TOML failed to parse. `path` is the registry file;
    /// `line` is the 1-based line number when available.
    #[error("registry parse error in {path}: {detail}")]
    RegistryParse { path: String, detail: String },

    /// Two registry entries resolve to the same canonical workspace path.
    /// `entries` is a human-readable description of the conflicting entries.
    #[error("duplicate workspace path in {path}: {entries}")]
    RegistryDuplicate { path: String, entries: String },

    /// A required field is absent from a workspace registry entry.
    #[error("registry entry '{entry}' missing required field '{field}'")]
    RegistryFieldMissing { entry: String, field: String },

    // === Policy Errors ===
    /// Policy TOML failed to parse. `line` is the 1-based source line when available.
    #[error("policy parse error in {path}{}: {detail}", .line.map(|l| format!(" at line {l}")).unwrap_or_default())]
    PolicyParse {
        path: String,
        line: Option<usize>,
        detail: String,
    },

    /// A policy rule references a nonexistent asset, role, or uses an
    /// unsupported rule type.
    #[error("invalid policy rule '{rule}': {reason}")]
    PolicyInvalidRule { rule: String, reason: String },

    // === Gate Errors ===
    /// Cycle detected while building the gate execution DAG.
    /// `gates` is the comma-separated list of gate names forming the cycle.
    #[error("cycle detected in gate DAG involving: {gates}")]
    GateCycle { gates: String },

    /// The gates.toml (or equivalent) could not be parsed.
    #[error("gate config parse error in {path}: {detail}")]
    GateConfigParse { path: String, detail: String },

    // === Persistence Errors ===
    /// SQLite database file could not be opened or created.
    #[error("persistence open failed for {path}: {detail}")]
    PersistenceOpen { path: String, detail: String },

    /// Schema migration between two versions failed.
    #[error("persistence migration from v{from} to v{to} failed: {detail}")]
    PersistenceMigration { from: u32, to: u32, detail: String },

    /// A SQLite query or statement failed at runtime.
    /// Mapped from `rusqlite::Error` via `From`.
    #[error("persistence query error: {detail}")]
    PersistenceQuery { detail: String },

    /// Audit log hash chain integrity check failed at the given entry ID.
    #[error("audit log hash chain broken at entry {entry_id}")]
    AuditChainBroken { entry_id: i64 },

    // === Configuration Errors ===
    /// Two or more CLI flags or config values are mutually exclusive.
    #[error("configuration conflict: {detail}")]
    ConfigConflict { detail: String },

    /// A specific CLI flag or config value is invalid.
    #[error("invalid configuration for '{flag}': {detail}")]
    ConfigInvalid { flag: String, detail: String },

    // === Subprocess Errors ===
    #[error("subprocess failed: {command} exited with code {code}")]
    SubprocessFailed { command: String, code: i32 },

    #[error("subprocess timed out after {timeout_secs}s: {command}")]
    SubprocessTimeout { command: String, timeout_secs: u64 },

    // === Security Errors ===
    #[error("validation rejected: {value} violates {rule}")]
    ValidationRejected { value: String, rule: String },

    #[error("path traversal rejected: {path}")]
    PathTraversal { path: String },

    // === Terminal / Logging Errors ===
    #[error("terminal capability missing: {capability}")]
    TerminalCapability { capability: String },

    #[error("log destination unavailable: {path}: {reason}")]
    LogDestination { path: String, reason: String },
}

// ---------------------------------------------------------------------------
// From conversions
// ---------------------------------------------------------------------------

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

/// Maps a rusqlite runtime error to `PersistenceQuery`.
impl From<rusqlite::Error> for TuiError {
    fn from(err: rusqlite::Error) -> Self {
        TuiError::PersistenceQuery {
            detail: err.to_string(),
        }
    }
}

/// Maps a TOML deserialization error to `ConfigInvalid`.
///
/// We use `ConfigInvalid` rather than introducing a one-off `TomlParse`
/// variant because all TOML errors in this codebase surface as configuration
/// problems (registry, policy, gates), and the existing `ConfigInvalid` shape
/// (`flag` + `detail`) gives callers enough context to add the specific flag /
/// file name at the call site.  This keeps the enum focused while remaining
/// ergonomic.
impl From<toml::de::Error> for TuiError {
    fn from(err: toml::de::Error) -> Self {
        TuiError::ConfigInvalid {
            flag: "toml".to_string(),
            detail: err.to_string(),
        }
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // -----------------------------------------------------------------------
    // Display format tests — existing variants (unchanged)
    // -----------------------------------------------------------------------

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

    // -----------------------------------------------------------------------
    // Display format tests — NEW registry variants
    // -----------------------------------------------------------------------

    #[test]
    fn test_registry_parse_display() {
        let err = TuiError::RegistryParse {
            path: "workspaces.toml".to_string(),
            detail: "invalid key".to_string(),
        };
        let s = err.to_string();
        assert!(s.contains("workspaces.toml"), "missing path: {s}");
        assert!(s.contains("invalid key"), "missing detail: {s}");
    }

    #[test]
    fn test_registry_duplicate_display() {
        let err = TuiError::RegistryDuplicate {
            path: "workspaces.toml".to_string(),
            entries: "entry 1 and entry 3".to_string(),
        };
        let s = err.to_string();
        assert!(s.contains("workspaces.toml"), "missing path: {s}");
        assert!(s.contains("entry 1 and entry 3"), "missing entries: {s}");
    }

    #[test]
    fn test_registry_field_missing_display() {
        let err = TuiError::RegistryFieldMissing {
            entry: "my-workspace".to_string(),
            field: "path".to_string(),
        };
        let s = err.to_string();
        assert!(s.contains("my-workspace"), "missing entry: {s}");
        assert!(s.contains("path"), "missing field: {s}");
    }

    // -----------------------------------------------------------------------
    // Display format tests — NEW policy variants
    // -----------------------------------------------------------------------

    #[test]
    fn test_policy_parse_with_line_display() {
        let err = TuiError::PolicyParse {
            path: "policies.toml".to_string(),
            line: Some(42),
            detail: "unexpected value".to_string(),
        };
        let s = err.to_string();
        assert!(s.contains("policies.toml"), "missing path: {s}");
        assert!(s.contains("42"), "missing line number: {s}");
        assert!(s.contains("unexpected value"), "missing detail: {s}");
    }

    #[test]
    fn test_policy_parse_without_line_display() {
        let err = TuiError::PolicyParse {
            path: "policies.toml".to_string(),
            line: None,
            detail: "missing table".to_string(),
        };
        let s = err.to_string();
        assert!(s.contains("policies.toml"), "missing path: {s}");
        assert!(s.contains("missing table"), "missing detail: {s}");
        // Should not contain a spurious line number
        assert!(!s.contains("line None"), "spurious 'None' in message: {s}");
    }

    #[test]
    fn test_policy_invalid_rule_display() {
        let err = TuiError::PolicyInvalidRule {
            rule: "require-security-agent".to_string(),
            reason: "references nonexistent asset 'security-agent-v1'".to_string(),
        };
        let s = err.to_string();
        assert!(s.contains("require-security-agent"), "missing rule: {s}");
        assert!(
            s.contains("references nonexistent asset"),
            "missing reason: {s}"
        );
    }

    // -----------------------------------------------------------------------
    // Display format tests — NEW gate variants
    // -----------------------------------------------------------------------

    #[test]
    fn test_gate_cycle_display() {
        let err = TuiError::GateCycle {
            gates: "lint -> test -> lint".to_string(),
        };
        let s = err.to_string();
        assert!(s.contains("lint -> test -> lint"), "missing gates: {s}");
        assert!(s.contains("cycle"), "missing cycle keyword: {s}");
    }

    #[test]
    fn test_gate_config_parse_display() {
        let err = TuiError::GateConfigParse {
            path: "gates.toml".to_string(),
            detail: "unknown key 'prereqs'".to_string(),
        };
        let s = err.to_string();
        assert!(s.contains("gates.toml"), "missing path: {s}");
        assert!(s.contains("unknown key"), "missing detail: {s}");
    }

    // -----------------------------------------------------------------------
    // Display format tests — NEW persistence variants
    // -----------------------------------------------------------------------

    #[test]
    fn test_persistence_open_display() {
        let err = TuiError::PersistenceOpen {
            path: "/tmp/vfa.db".to_string(),
            detail: "no such file or directory".to_string(),
        };
        let s = err.to_string();
        assert!(s.contains("/tmp/vfa.db"), "missing path: {s}");
        assert!(s.contains("no such file"), "missing detail: {s}");
    }

    #[test]
    fn test_persistence_migration_display() {
        let err = TuiError::PersistenceMigration {
            from: 1,
            to: 2,
            detail: "column 'foo' already exists".to_string(),
        };
        let s = err.to_string();
        assert!(s.contains('1'), "missing from version: {s}");
        assert!(s.contains('2'), "missing to version: {s}");
        assert!(s.contains("column 'foo'"), "missing detail: {s}");
    }

    #[test]
    fn test_persistence_query_display() {
        let err = TuiError::PersistenceQuery {
            detail: "no rows returned".to_string(),
        };
        let s = err.to_string();
        assert!(s.contains("no rows returned"), "missing detail: {s}");
    }

    #[test]
    fn test_audit_chain_broken_display() {
        let err = TuiError::AuditChainBroken { entry_id: 99 };
        let s = err.to_string();
        assert!(s.contains("99"), "missing entry_id: {s}");
        assert!(s.contains("audit"), "missing 'audit': {s}");
    }

    // -----------------------------------------------------------------------
    // Display format tests — NEW configuration variants
    // -----------------------------------------------------------------------

    #[test]
    fn test_config_conflict_display() {
        let err = TuiError::ConfigConflict {
            detail: "--report and --validate-config are mutually exclusive".to_string(),
        };
        let s = err.to_string();
        assert!(s.contains("mutually exclusive"), "missing detail: {s}");
    }

    #[test]
    fn test_config_invalid_display() {
        let err = TuiError::ConfigInvalid {
            flag: "--log-level".to_string(),
            detail: "unknown level 'verbose'".to_string(),
        };
        let s = err.to_string();
        assert!(s.contains("--log-level"), "missing flag: {s}");
        assert!(s.contains("unknown level"), "missing detail: {s}");
    }

    // -----------------------------------------------------------------------
    // From<rusqlite::Error> — maps to PersistenceQuery
    // -----------------------------------------------------------------------

    #[test]
    fn test_from_rusqlite_query_returned_no_rows() {
        let rusqlite_err = rusqlite::Error::QueryReturnedNoRows;
        let tui_err: TuiError = rusqlite_err.into();
        match tui_err {
            TuiError::PersistenceQuery { detail } => {
                assert!(!detail.is_empty(), "detail should not be empty");
            }
            other => panic!("expected PersistenceQuery, got {other:?}"),
        }
    }

    #[test]
    fn test_from_rusqlite_bad_query_via_connection() {
        let conn = rusqlite::Connection::open_in_memory()
            .expect("in-memory SQLite should always open");
        // Execute a query against a table that does not exist — this produces
        // a real rusqlite error we can convert.
        let rusqlite_err = conn
            .execute("SELECT * FROM nonexistent_table", [])
            .unwrap_err();
        let tui_err: TuiError = rusqlite_err.into();
        match tui_err {
            TuiError::PersistenceQuery { detail } => {
                assert!(
                    detail.contains("nonexistent_table") || !detail.is_empty(),
                    "detail should describe the error: {detail}"
                );
            }
            other => panic!("expected PersistenceQuery, got {other:?}"),
        }
    }

    // -----------------------------------------------------------------------
    // From<toml::de::Error> — maps to ConfigInvalid
    // -----------------------------------------------------------------------

    #[test]
    fn test_from_toml_de_error() {
        // Parse intentionally invalid TOML to obtain a real toml::de::Error.
        let toml_err = toml::from_str::<toml::Value>("key = [invalid").unwrap_err();
        let tui_err: TuiError = toml_err.into();
        match tui_err {
            TuiError::ConfigInvalid { flag, detail } => {
                assert_eq!(flag, "toml", "flag should be 'toml'");
                assert!(!detail.is_empty(), "detail should describe the parse error");
            }
            other => panic!("expected ConfigInvalid, got {other:?}"),
        }
    }

    // -----------------------------------------------------------------------
    // From<std::io::Error> — existing conversions unchanged
    // -----------------------------------------------------------------------

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

    // -----------------------------------------------------------------------
    // From<serde_json::Error> — existing conversion unchanged
    // -----------------------------------------------------------------------

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

    // -----------------------------------------------------------------------
    // Trait bounds
    // -----------------------------------------------------------------------

    #[test]
    fn test_error_is_send_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<TuiError>();
    }
}
