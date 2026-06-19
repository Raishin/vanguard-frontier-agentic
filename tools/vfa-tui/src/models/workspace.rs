//! Workspace registry models — entries, resolved state, and status.

use serde::{Deserialize, Serialize};

/// A single entry from the workspace registry TOML (`[[workspace]]` table).
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WorkspaceEntry {
    /// Path to the workspace root — supports `$HOME`/`$VAR` expansion.
    pub path: String,
    /// Human-readable name; defaults to the directory basename when absent.
    pub name: Option<String>,
    /// Owning team identifier.
    pub team: Option<String>,
    /// Free-form classification tags (e.g. `["production", "pci"]`).
    pub tags: Option<Vec<String>>,
    /// Per-workspace policy overrides stored as a raw TOML value.
    pub policy_overrides: Option<toml::Value>,
}

/// A `WorkspaceEntry` after path expansion and filesystem validation.
#[derive(Debug, Clone)]
pub struct ResolvedWorkspace {
    /// Canonicalized absolute path to the workspace root.
    pub canonical_path: std::path::PathBuf,
    /// Effective name (from entry or derived from basename).
    pub name: String,
    /// Owning team, if specified.
    pub team: Option<String>,
    /// Classification tags (empty if none were specified).
    pub tags: Vec<String>,
    /// Current reachability / scan status of this workspace.
    pub status: WorkspaceStatus,
}

/// Reachability and scan status of a resolved workspace.
#[derive(Debug, Clone, PartialEq)]
pub enum WorkspaceStatus {
    /// Workspace path exists and is accessible.
    Available,
    /// Workspace path is inaccessible; carries the reason.
    Unavailable(String),
    /// A background scan is currently in progress.
    Scanning,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn workspace_entry_round_trip() {
        let entry = WorkspaceEntry {
            path: "$HOME/repos/payment-service".to_string(),
            name: Some("payment-service".to_string()),
            team: Some("payments".to_string()),
            tags: Some(vec!["production".to_string(), "pci".to_string()]),
            policy_overrides: None,
        };

        let json = serde_json::to_string(&entry).expect("serialize");
        let decoded: WorkspaceEntry = serde_json::from_str(&json).expect("deserialize");

        assert_eq!(decoded.path, entry.path);
        assert_eq!(decoded.name, entry.name);
        assert_eq!(decoded.team, entry.team);
        assert_eq!(decoded.tags, entry.tags);
    }

    #[test]
    fn workspace_entry_minimal_fields() {
        let json = r#"{"path": "/home/user/foo"}"#;
        let entry: WorkspaceEntry = serde_json::from_str(json).expect("deserialize");
        assert_eq!(entry.path, "/home/user/foo");
        assert!(entry.name.is_none());
        assert!(entry.team.is_none());
        assert!(entry.tags.is_none());
        assert!(entry.policy_overrides.is_none());
    }

    #[test]
    fn workspace_status_equality() {
        assert_eq!(WorkspaceStatus::Available, WorkspaceStatus::Available);
        assert_eq!(
            WorkspaceStatus::Unavailable("no such path".to_string()),
            WorkspaceStatus::Unavailable("no such path".to_string()),
        );
        assert_ne!(WorkspaceStatus::Available, WorkspaceStatus::Scanning);
    }

    #[test]
    fn resolved_workspace_debug_clone() {
        let rw = ResolvedWorkspace {
            canonical_path: std::path::PathBuf::from("/home/user/foo"),
            name: "foo".to_string(),
            team: None,
            tags: vec![],
            status: WorkspaceStatus::Available,
        };
        let cloned = rw.clone();
        assert_eq!(cloned.name, "foo");
        // Ensure Debug doesn't panic.
        let _ = format!("{:?}", rw);
    }
}
