//! Integration test 13.3 — workspace scanning with mock harness directories.
//! Validates: Requirements 7.2, 7.3, 7.6, 7.7, 7.8, 27.2
//!
//! Builds throwaway workspaces on disk (`.claude/agents/*.md`) and verifies the
//! scanner's multi-strategy detection: filename + VFA-EXPORT metadata confirm
//! an install (≥2 strategies); a filename-only hit stays unconfirmed; unrelated
//! files are ignored.

use std::path::{Path, PathBuf};

use vfa_tui::federation::scanner::{CatalogIndex, DetectionMethod, WorkspaceScanner};
use vfa_tui::models::workspace::{ResolvedWorkspace, WorkspaceStatus};

const ASSET_ID: &str = "agents/aws/cdk-agent";

/// Write `content` to `<ws>/.claude/agents/<file_name>`.
fn place_claude_agent(ws: &Path, file_name: &str, content: &str) {
    let agents = ws.join(".claude").join("agents");
    std::fs::create_dir_all(&agents).unwrap();
    std::fs::write(agents.join(file_name), content).unwrap();
}

fn index() -> CatalogIndex {
    // Register the canonical asset with no template content so the content
    // signature strategy stays out of it (keeps detection deterministic).
    CatalogIndex::new(vec![(ASSET_ID.to_string(), ASSET_ID.to_string(), None)])
}

fn resolved(ws: &Path) -> ResolvedWorkspace {
    ResolvedWorkspace {
        canonical_path: ws.to_path_buf(),
        name: "team-a".to_string(),
        team: None,
        tags: vec![],
        status: WorkspaceStatus::Available,
    }
}

#[test]
fn filename_plus_metadata_confirms_install() {
    let tmp = tempfile::TempDir::new().unwrap();
    let content = format!(
        "# VFA-EXPORT: {{\"id\":\"{ASSET_ID}\",\"version\":\"1.2.0\"}}\n# CDK Agent\nbody line\n"
    );
    place_claude_agent(tmp.path(), "cdk-agent.md", &content);

    let scanner = WorkspaceScanner::new(4);
    let assets = scanner.scan_workspace(&resolved(tmp.path()), &index());

    assert_eq!(assets.len(), 1, "exactly one asset detected");
    let a = &assets[0];
    assert_eq!(a.asset_id, ASSET_ID);
    assert!(a.confirmed, "two strategies must confirm the install");
    assert_eq!(a.installed_version.as_deref(), Some("1.2.0"));
    assert!(a.detection_methods.contains(&DetectionMethod::Filename));
    assert!(a
        .detection_methods
        .contains(&DetectionMethod::MetadataComment));
    assert_eq!(a.harness, ".claude");
    assert!(!a.content_hash.is_empty(), "content hash recorded");
}

#[test]
fn filename_only_is_unconfirmed() {
    let tmp = tempfile::TempDir::new().unwrap();
    // Matching filename, but no VFA-EXPORT header and no template to match.
    place_claude_agent(tmp.path(), "cdk-agent.md", "# CDK Agent\njust some text\n");

    let scanner = WorkspaceScanner::new(4);
    let assets = scanner.scan_workspace(&resolved(tmp.path()), &index());

    assert_eq!(assets.len(), 1);
    assert_eq!(assets[0].asset_id, ASSET_ID);
    assert!(!assets[0].confirmed, "a single strategy must NOT confirm");
    assert_eq!(assets[0].detection_methods, vec![DetectionMethod::Filename]);
}

#[test]
fn unrelated_file_is_not_detected() {
    let tmp = tempfile::TempDir::new().unwrap();
    place_claude_agent(
        tmp.path(),
        "totally-unknown.md",
        "# Unknown\nnot in catalog\n",
    );

    let scanner = WorkspaceScanner::new(4);
    let assets = scanner.scan_workspace(&resolved(tmp.path()), &index());
    assert!(assets.is_empty(), "no strategy fires for an unknown file");
}

#[test]
fn metadata_only_detects_via_export_header() {
    // No filename match (different name), but a VFA-EXPORT header names the id.
    let tmp = tempfile::TempDir::new().unwrap();
    let content = format!("# VFA-EXPORT: {{\"id\":\"{ASSET_ID}\"}}\n# Renamed\n");
    place_claude_agent(tmp.path(), "renamed-by-user.md", &content);

    let scanner = WorkspaceScanner::new(4);
    let assets = scanner.scan_workspace(&resolved(tmp.path()), &index());

    assert_eq!(assets.len(), 1);
    assert_eq!(assets[0].asset_id, ASSET_ID);
    // Only the metadata strategy fired → not yet confirmed.
    assert!(!assets[0].confirmed);
    assert_eq!(
        assets[0].detection_methods,
        vec![DetectionMethod::MetadataComment]
    );
}

#[tokio::test]
async fn scan_all_aggregates_per_workspace() {
    let tmp_a = tempfile::TempDir::new().unwrap();
    let tmp_b = tempfile::TempDir::new().unwrap();
    let content =
        format!("# VFA-EXPORT: {{\"id\":\"{ASSET_ID}\",\"version\":\"2.0.0\"}}\n# A\nx\n");
    place_claude_agent(tmp_a.path(), "cdk-agent.md", &content);
    // Workspace B has nothing installed.

    let scanner = WorkspaceScanner::new(4);
    let workspaces = vec![resolved(tmp_a.path()), resolved(tmp_b.path())];
    let result = scanner.scan_all(&workspaces, &index()).await;

    let a: PathBuf = tmp_a.path().to_path_buf();
    let b: PathBuf = tmp_b.path().to_path_buf();
    assert_eq!(
        result.get(&a).map(|v| v.len()).unwrap_or(0),
        1,
        "workspace A has one install"
    );
    assert!(
        result.get(&b).map(|v| v.is_empty()).unwrap_or(true),
        "workspace B has none"
    );
}
