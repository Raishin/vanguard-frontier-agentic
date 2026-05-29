use std::path::{Path, PathBuf};

use crate::error::TuiError;

/// Detect the workspace root by traversing upward from `start` (or CWD if None).
///
/// A valid workspace directory must contain:
/// - `catalog/agents.json` (file must exist)
/// - `package.json` (must contain `"name": "@raishin/vanguard-frontier-agentic"`)
pub fn detect_workspace(start: Option<&Path>) -> Result<PathBuf, TuiError> {
    let start_dir = match start {
        Some(p) => p.to_path_buf(),
        None => std::env::current_dir().map_err(|_| TuiError::WorkspaceNotFound {
            start: PathBuf::from("."),
        })?,
    };

    let mut current = start_dir.clone();

    loop {
        if is_workspace_root(&current) {
            return Ok(current);
        }

        if !current.pop() {
            break;
        }
    }

    Err(TuiError::WorkspaceNotFound { start: start_dir })
}

fn is_workspace_root(dir: &Path) -> bool {
    // Check catalog/agents.json exists
    let agents_path = dir.join("catalog").join("agents.json");
    if !agents_path.is_file() {
        return false;
    }

    // Check package.json exists and contains the expected name
    let package_path = dir.join("package.json");
    if !package_path.is_file() {
        return false;
    }

    match std::fs::read_to_string(&package_path) {
        Ok(content) => match serde_json::from_str::<serde_json::Value>(&content) {
            Ok(v) => {
                v.get("name").and_then(|n| n.as_str()) == Some("@raishin/vanguard-frontier-agentic")
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
        // No space after colon - the old string matching would fail here
        fs::write(
            tmp.path().join("package.json"),
            r#"{"name":"@raishin/vanguard-frontier-agentic","version":"1.0.0"}"#,
        )
        .unwrap();
        let result = detect_workspace(Some(tmp.path()));
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), tmp.path());
    }
}
