use std::path::{Path, PathBuf};

use crate::error::TuiError;

/// Characters that are rejected in subprocess arguments.
const SHELL_METACHARACTERS: &[char] = &[
    ';', '|', '&', '$', '`', '\\', '<', '>', '(', ')', '{', '}', '!', '#', '*', '?', '[', ']',
    '\n', '\r', '\0',
];

/// Validate a subprocess argument. Rejects if it contains any shell metacharacters.
pub fn validate_argument(arg: &str) -> Result<(), TuiError> {
    for c in arg.chars() {
        if SHELL_METACHARACTERS.contains(&c) {
            return Err(TuiError::ValidationRejected {
                value: arg.to_string(),
                rule: format!("contains forbidden character {:?}", c),
            });
        }
    }
    Ok(())
}

/// Validate a path is within the workspace root.
/// Canonicalizes both paths, then checks that the path starts with the workspace root.
/// Rejects null bytes and paths that escape the workspace.
pub fn validate_path(path: &Path, workspace_root: &Path) -> Result<PathBuf, TuiError> {
    // Reject null bytes in path
    if let Some(s) = path.to_str() {
        if s.contains('\0') {
            return Err(TuiError::PathTraversal {
                path: path.to_path_buf(),
            });
        }
    } else {
        // Non-UTF-8 path - reject
        return Err(TuiError::PathTraversal {
            path: path.to_path_buf(),
        });
    }

    let canonical_root = workspace_root
        .canonicalize()
        .map_err(|_| TuiError::PathTraversal {
            path: path.to_path_buf(),
        })?;

    let canonical_path = path.canonicalize().map_err(|_| TuiError::PathTraversal {
        path: path.to_path_buf(),
    })?;

    if !canonical_path.starts_with(&canonical_root) {
        return Err(TuiError::PathTraversal {
            path: path.to_path_buf(),
        });
    }

    Ok(canonical_path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn validate_argument_accepts_safe() {
        assert!(validate_argument("hello-world_1.0/path").is_ok());
    }

    #[test]
    fn validate_argument_rejects_semicolon() {
        let err = validate_argument("cmd; rm -rf /").unwrap_err();
        match err {
            TuiError::ValidationRejected { rule, .. } => {
                assert!(rule.contains(';'));
            }
            other => panic!("unexpected error: {other:?}"),
        }
    }

    #[test]
    fn validate_argument_rejects_pipe() {
        assert!(validate_argument("cmd | cat").is_err());
    }

    #[test]
    fn validate_argument_rejects_backtick() {
        assert!(validate_argument("$(whoami)").is_err());
    }

    #[test]
    fn validate_argument_rejects_null() {
        assert!(validate_argument("hello\0world").is_err());
    }

    #[test]
    fn validate_argument_rejects_newline() {
        assert!(validate_argument("line1\nline2").is_err());
    }

    #[test]
    fn validate_path_accepts_within_workspace() {
        let tmp = TempDir::new().unwrap();
        let sub = tmp.path().join("subdir");
        fs::create_dir_all(&sub).unwrap();
        let file = sub.join("test.txt");
        fs::write(&file, "content").unwrap();

        let result = validate_path(&file, tmp.path());
        assert!(result.is_ok());
    }

    #[test]
    fn validate_path_rejects_outside_workspace() {
        let tmp1 = TempDir::new().unwrap();
        let tmp2 = TempDir::new().unwrap();
        let file = tmp2.path().join("secret.txt");
        fs::write(&file, "secret").unwrap();

        let result = validate_path(&file, tmp1.path());
        assert!(result.is_err());
    }

    #[test]
    fn validate_path_rejects_traversal() {
        let tmp = TempDir::new().unwrap();
        let sub = tmp.path().join("sub");
        fs::create_dir_all(&sub).unwrap();

        // Try to traverse out using ..
        let traversal = sub.join("..").join("..").join("etc").join("passwd");
        let result = validate_path(&traversal, tmp.path());
        assert!(result.is_err());
    }
}
