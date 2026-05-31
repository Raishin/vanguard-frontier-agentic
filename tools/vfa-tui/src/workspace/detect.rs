use std::path::{Path, PathBuf};

use crate::error::TuiError;

/// The expected package name that identifies a valid workspace.
const EXPECTED_PACKAGE_NAME: &str = "@raishin/vanguard-frontier-agentic";

/// Result of checking workspace markers at a given directory.
#[derive(Debug, PartialEq)]
enum MarkerCheck {
    /// Both markers present and valid — this is the workspace root.
    Valid,
    /// Neither marker is present — continue traversal.
    None,
    /// Only some markers are present — invalid workspace (partial markers).
    Partial { missing: String },
}

/// Detect the workspace root by traversing upward from `start` (or CWD if None).
///
/// A valid workspace directory must contain:
/// - `catalog/agents.json` (file must exist)
/// - `package.json` (must contain `"name": "@raishin/vanguard-frontier-agentic"`)
///
/// Returns:
/// - `Ok(path)` if a valid workspace root is found
/// - `Err(TuiError::InvalidWorkspace)` if markers are partially present
/// - `Err(TuiError::WorkspaceNotFound)` if traversal reaches filesystem root
pub fn detect_workspace(start: Option<&Path>) -> Result<PathBuf, TuiError> {
    let start_dir = match start {
        Some(p) => p.to_path_buf(),
        None => std::env::current_dir().map_err(|_| TuiError::WorkspaceNotFound {
            start: ".".to_string(),
        })?,
    };

    let mut current = start_dir.clone();

    loop {
        match check_markers(&current) {
            MarkerCheck::Valid => return Ok(current),
            MarkerCheck::Partial { missing } => {
                return Err(TuiError::InvalidWorkspace {
                    path: current.display().to_string(),
                    missing,
                });
            }
            MarkerCheck::None => {
                if !current.pop() {
                    break;
                }
            }
        }
    }

    Err(TuiError::WorkspaceNotFound {
        start: start_dir.display().to_string(),
    })
}

/// Check workspace markers at a given directory.
///
/// Logic:
/// 1. Check if `catalog/agents.json` exists
/// 2. Check if `package.json` exists
/// 3. If both exist, parse `package.json` and verify the `name` field
/// 4. Return the appropriate `MarkerCheck` variant
fn check_markers(dir: &Path) -> MarkerCheck {
    let has_catalog = dir.join("catalog").join("agents.json").is_file();
    let package_path = dir.join("package.json");
    let has_package = package_path.is_file();

    match (has_catalog, has_package) {
        (false, false) => MarkerCheck::None,
        (true, false) => MarkerCheck::Partial {
            missing: "package.json".to_string(),
        },
        (false, true) => {
            // Only consider this partial if the package.json actually has our name.
            // A random package.json in a subdirectory shouldn't trigger InvalidWorkspace.
            if package_has_expected_name(&package_path) {
                MarkerCheck::Partial {
                    missing: "catalog/agents.json".to_string(),
                }
            } else {
                MarkerCheck::None
            }
        }
        (true, true) => {
            if package_has_expected_name(&package_path) {
                MarkerCheck::Valid
            } else {
                // catalog/agents.json exists but package.json has wrong name.
                // This is a partial match — the package name doesn't match.
                MarkerCheck::Partial {
                    missing: format!(
                        "package.json with name \"{}\"",
                        EXPECTED_PACKAGE_NAME
                    ),
                }
            }
        }
    }
}

/// Check if a package.json file contains the expected package name.
fn package_has_expected_name(package_path: &Path) -> bool {
    match std::fs::read_to_string(package_path) {
        Ok(content) => match serde_json::from_str::<serde_json::Value>(&content) {
            Ok(v) => {
                v.get("name").and_then(|n| n.as_str()) == Some(EXPECTED_PACKAGE_NAME)
            }
            Err(_) => false,
        },
        Err(_) => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    fn create_workspace(dir: &Path) {
        let catalog_dir = dir.join("catalog");
        fs::create_dir_all(&catalog_dir).unwrap();
        fs::write(catalog_dir.join("agents.json"), "[]").unwrap();
        fs::write(
            dir.join("package.json"),
            r#"{"name": "@raishin/vanguard-frontier-agentic"}"#,
        )
        .unwrap();
    }

    #[test]
    fn detect_at_root() {
        let tmp = TempDir::new().unwrap();
        create_workspace(tmp.path());
        let result = detect_workspace(Some(tmp.path())).unwrap();
        assert_eq!(result, tmp.path());
    }

    #[test]
    fn detect_from_subdir() {
        let tmp = TempDir::new().unwrap();
        create_workspace(tmp.path());
        let sub = tmp.path().join("tools").join("vfa-tui");
        fs::create_dir_all(&sub).unwrap();
        let result = detect_workspace(Some(&sub)).unwrap();
        assert_eq!(result, tmp.path());
    }

    #[test]
    fn detect_fails_no_workspace() {
        let tmp = TempDir::new().unwrap();
        let result = detect_workspace(Some(tmp.path()));
        assert!(result.is_err());
        match result.unwrap_err() {
            TuiError::WorkspaceNotFound { .. } => {}
            other => panic!("unexpected error: {other:?}"),
        }
    }

    #[test]
    fn detect_fails_wrong_package_name() {
        // catalog/agents.json exists + package.json with wrong name = InvalidWorkspace
        let tmp = TempDir::new().unwrap();
        let catalog_dir = tmp.path().join("catalog");
        fs::create_dir_all(&catalog_dir).unwrap();
        fs::write(catalog_dir.join("agents.json"), "[]").unwrap();
        fs::write(
            tmp.path().join("package.json"),
            r#"{"name": "something-else"}"#,
        )
        .unwrap();
        let result = detect_workspace(Some(tmp.path()));
        assert!(result.is_err());
        match result.unwrap_err() {
            TuiError::InvalidWorkspace { path, missing } => {
                assert_eq!(path, tmp.path().display().to_string());
                assert!(missing.contains("package.json with name"));
            }
            other => panic!("expected InvalidWorkspace, got: {other:?}"),
        }
    }

    #[test]
    fn detect_fails_missing_package_json() {
        // catalog/agents.json exists but no package.json = InvalidWorkspace
        let tmp = TempDir::new().unwrap();
        let catalog_dir = tmp.path().join("catalog");
        fs::create_dir_all(&catalog_dir).unwrap();
        fs::write(catalog_dir.join("agents.json"), "[]").unwrap();
        let result = detect_workspace(Some(tmp.path()));
        assert!(result.is_err());
        match result.unwrap_err() {
            TuiError::InvalidWorkspace { path, missing } => {
                assert_eq!(path, tmp.path().display().to_string());
                assert_eq!(missing, "package.json");
            }
            other => panic!("expected InvalidWorkspace, got: {other:?}"),
        }
    }

    #[test]
    fn detect_fails_missing_catalog() {
        // package.json with correct name but no catalog/agents.json = InvalidWorkspace
        let tmp = TempDir::new().unwrap();
        fs::write(
            tmp.path().join("package.json"),
            r#"{"name": "@raishin/vanguard-frontier-agentic"}"#,
        )
        .unwrap();
        let result = detect_workspace(Some(tmp.path()));
        assert!(result.is_err());
        match result.unwrap_err() {
            TuiError::InvalidWorkspace { path, missing } => {
                assert_eq!(path, tmp.path().display().to_string());
                assert_eq!(missing, "catalog/agents.json");
            }
            other => panic!("expected InvalidWorkspace, got: {other:?}"),
        }
    }

    #[test]
    fn detect_ignores_unrelated_package_json() {
        // A package.json with a different name and no catalog should be ignored (None),
        // not treated as partial. Traversal should continue.
        let tmp = TempDir::new().unwrap();
        let sub = tmp.path().join("sub");
        fs::create_dir_all(&sub).unwrap();
        fs::write(
            sub.join("package.json"),
            r#"{"name": "unrelated-package"}"#,
        )
        .unwrap();
        // No workspace anywhere — should get WorkspaceNotFound
        let result = detect_workspace(Some(&sub));
        assert!(result.is_err());
        match result.unwrap_err() {
            TuiError::WorkspaceNotFound { .. } => {}
            other => panic!("expected WorkspaceNotFound, got: {other:?}"),
        }
    }

    #[test]
    fn detect_real_workspace() {
        // Detect from the tools/vfa-tui directory against the real workspace
        let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
        let result = detect_workspace(Some(manifest_dir));
        assert!(result.is_ok());
        let ws = result.unwrap();
        assert!(ws.join("catalog").join("agents.json").is_file());
    }

    #[test]
    fn detect_with_nonstandard_whitespace() {
        let tmp = TempDir::new().unwrap();
        let catalog_dir = tmp.path().join("catalog");
        fs::create_dir_all(&catalog_dir).unwrap();
        fs::write(catalog_dir.join("agents.json"), "[]").unwrap();
        // No space after colon - JSON parsing handles this correctly
        fs::write(
            tmp.path().join("package.json"),
            r#"{"name":"@raishin/vanguard-frontier-agentic","version":"1.0.0"}"#,
        )
        .unwrap();
        let result = detect_workspace(Some(tmp.path()));
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), tmp.path());
    }

    #[test]
    fn detect_skips_invalid_json_package() {
        // catalog/agents.json exists + package.json with invalid JSON = InvalidWorkspace
        // (because catalog marker is present but package.json can't be parsed)
        let tmp = TempDir::new().unwrap();
        let catalog_dir = tmp.path().join("catalog");
        fs::create_dir_all(&catalog_dir).unwrap();
        fs::write(catalog_dir.join("agents.json"), "[]").unwrap();
        fs::write(tmp.path().join("package.json"), "not valid json").unwrap();
        let result = detect_workspace(Some(tmp.path()));
        assert!(result.is_err());
        match result.unwrap_err() {
            TuiError::InvalidWorkspace { missing, .. } => {
                assert!(missing.contains("package.json with name"));
            }
            other => panic!("expected InvalidWorkspace, got: {other:?}"),
        }
    }

    #[test]
    fn check_markers_both_valid() {
        let tmp = TempDir::new().unwrap();
        create_workspace(tmp.path());
        assert_eq!(check_markers(tmp.path()), MarkerCheck::Valid);
    }

    #[test]
    fn check_markers_none() {
        let tmp = TempDir::new().unwrap();
        assert_eq!(check_markers(tmp.path()), MarkerCheck::None);
    }

    #[test]
    fn check_markers_partial_missing_package() {
        let tmp = TempDir::new().unwrap();
        let catalog_dir = tmp.path().join("catalog");
        fs::create_dir_all(&catalog_dir).unwrap();
        fs::write(catalog_dir.join("agents.json"), "[]").unwrap();
        assert_eq!(
            check_markers(tmp.path()),
            MarkerCheck::Partial {
                missing: "package.json".to_string()
            }
        );
    }

    #[test]
    fn check_markers_partial_missing_catalog() {
        let tmp = TempDir::new().unwrap();
        fs::write(
            tmp.path().join("package.json"),
            r#"{"name": "@raishin/vanguard-frontier-agentic"}"#,
        )
        .unwrap();
        assert_eq!(
            check_markers(tmp.path()),
            MarkerCheck::Partial {
                missing: "catalog/agents.json".to_string()
            }
        );
    }
}
