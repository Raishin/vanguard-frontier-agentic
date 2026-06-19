// Feature: rust-tui, Property 16: Workspace detection finds correct root
//
// For any directory tree where exactly one ancestor directory contains both
// `catalog/agents.json` and a `package.json` with `name` equal to
// `@raishin/vanguard-frontier-agentic`, the workspace detector SHALL return
// that directory. If no such ancestor exists, the detector SHALL return an error.
//
// **Validates: Requirements 14.2, 15.1**

use proptest::prelude::*;
use proptest::test_runner::Config;
use std::fs;
use tempfile::TempDir;
use vfa_tui::workspace::detect_workspace;

/// Create valid workspace markers at the given directory.
fn create_workspace_markers(dir: &std::path::Path) {
    let catalog_dir = dir.join("catalog");
    fs::create_dir_all(&catalog_dir).unwrap();
    fs::write(catalog_dir.join("agents.json"), "[]").unwrap();
    fs::write(
        dir.join("package.json"),
        r#"{"name": "@raishin/vanguard-frontier-agentic"}"#,
    )
    .unwrap();
}

/// Strategy to generate a vector of directory segment names (1-5 segments).
fn dir_segments_strategy() -> impl Strategy<Value = Vec<String>> {
    prop::collection::vec("[a-z][a-z0-9_]{0,7}", 1..=5)
}

/// Strategy to generate a workspace level (which ancestor gets the markers)
/// and a starting subdirectory depth below that level.
fn workspace_placement_strategy() -> impl Strategy<Value = (Vec<String>, usize)> {
    dir_segments_strategy().prop_flat_map(|segments| {
        let len = segments.len();
        // workspace_level: 0 means markers at root, len means markers at deepest dir
        (Just(segments), 0..=len)
    })
}

proptest! {
    #![proptest_config(Config::with_cases(128))]

    /// Property 16a: When workspace markers exist at an ancestor, detection from
    /// any subdirectory below that ancestor SHALL return the ancestor directory.
    #[test]
    fn workspace_detected_from_any_subdirectory(
        segments in dir_segments_strategy(),
        workspace_level in 0usize..5,
        extra_depth in 0usize..4,
    ) {
        let tmp = TempDir::new().unwrap();
        let root = tmp.path().to_path_buf();

        // Build the path up to workspace_level (clamped to segments length)
        let effective_level = workspace_level.min(segments.len());
        let mut workspace_dir = root.clone();
        for seg in segments.iter().take(effective_level) {
            workspace_dir = workspace_dir.join(seg);
        }
        fs::create_dir_all(&workspace_dir).unwrap();
        create_workspace_markers(&workspace_dir);

        // Build a starting directory deeper than the workspace
        let mut start_dir = workspace_dir.clone();
        for seg in segments.iter().skip(effective_level) {
            start_dir = start_dir.join(seg);
        }
        // Add extra depth beyond the segments
        for i in 0..extra_depth {
            start_dir = start_dir.join(format!("extra{i}"));
        }
        fs::create_dir_all(&start_dir).unwrap();

        let result = detect_workspace(Some(&start_dir));
        prop_assert!(
            result.is_ok(),
            "Expected workspace found from {:?}, got error: {:?}",
            start_dir,
            result.err()
        );
        let expected = workspace_dir;
        prop_assert_eq!(result.unwrap(), expected);
    }

    /// Property 16b: When no ancestor contains valid workspace markers,
    /// detection SHALL return an error.
    #[test]
    fn no_workspace_returns_error(
        segments in dir_segments_strategy(),
    ) {
        let tmp = TempDir::new().unwrap();
        let root = tmp.path().to_path_buf();

        // Build a directory tree with NO workspace markers anywhere
        let mut dir = root;
        for seg in &segments {
            dir = dir.join(seg);
        }
        fs::create_dir_all(&dir).unwrap();

        let result = detect_workspace(Some(&dir));
        prop_assert!(
            result.is_err(),
            "Expected error when no workspace markers exist, got: {:?}",
            result.ok()
        );
    }

    /// Property 16c: When workspace markers are placed at a random ancestor level,
    /// starting from a random subdirectory within the tree, detection SHALL find
    /// the correct ancestor.
    #[test]
    fn workspace_at_random_ancestor_level(
        (segments, workspace_level) in workspace_placement_strategy(),
        start_offset in 0usize..3,
    ) {
        let tmp = TempDir::new().unwrap();
        let root = tmp.path().to_path_buf();

        // Place workspace markers at workspace_level
        let mut workspace_dir = root.clone();
        for seg in segments.iter().take(workspace_level) {
            workspace_dir = workspace_dir.join(seg);
        }
        fs::create_dir_all(&workspace_dir).unwrap();
        create_workspace_markers(&workspace_dir);

        // Start from a directory at or below the workspace level
        let mut start_dir = workspace_dir.clone();
        for seg in segments.iter().skip(workspace_level) {
            start_dir = start_dir.join(seg);
        }
        // Add optional extra depth
        for i in 0..start_offset {
            start_dir = start_dir.join(format!("deep{i}"));
        }
        fs::create_dir_all(&start_dir).unwrap();

        let result = detect_workspace(Some(&start_dir));
        prop_assert!(
            result.is_ok(),
            "Expected workspace found from {:?}, got error: {:?}",
            start_dir,
            result.err()
        );
        let expected = workspace_dir;
        prop_assert_eq!(result.unwrap(), expected);
    }

    /// Property 16d: Detection from the workspace directory itself SHALL return
    /// that same directory.
    #[test]
    fn workspace_detected_at_start_dir(
        segments in dir_segments_strategy(),
    ) {
        let tmp = TempDir::new().unwrap();
        let root = tmp.path().to_path_buf();

        // Build a path and place workspace markers at the end
        let mut workspace_dir = root;
        for seg in &segments {
            workspace_dir = workspace_dir.join(seg);
        }
        fs::create_dir_all(&workspace_dir).unwrap();
        create_workspace_markers(&workspace_dir);

        // Start detection from the workspace directory itself
        let result = detect_workspace(Some(&workspace_dir));
        prop_assert!(
            result.is_ok(),
            "Expected workspace found at {:?}, got error: {:?}",
            workspace_dir,
            result.err()
        );
        let expected = workspace_dir;
        prop_assert_eq!(result.unwrap(), expected);
    }

    /// Property 16e: A directory with catalog/agents.json but wrong package name
    /// SHALL NOT be returned as a valid workspace (returns error).
    #[test]
    fn wrong_package_name_is_not_valid_workspace(
        segments in dir_segments_strategy(),
        wrong_name in "[a-z][a-z0-9-]{2,20}",
    ) {
        let tmp = TempDir::new().unwrap();
        let root = tmp.path().to_path_buf();

        // Build directory tree
        let mut dir = root;
        for seg in &segments {
            dir = dir.join(seg);
        }
        fs::create_dir_all(&dir).unwrap();

        // Place catalog/agents.json and package.json with WRONG name
        let catalog_dir = dir.join("catalog");
        fs::create_dir_all(&catalog_dir).unwrap();
        fs::write(catalog_dir.join("agents.json"), "[]").unwrap();
        fs::write(
            dir.join("package.json"),
            format!(r#"{{"name": "{}"}}"#, wrong_name),
        )
        .unwrap();

        let result = detect_workspace(Some(&dir));
        // Should be an error (either InvalidWorkspace or WorkspaceNotFound)
        prop_assert!(
            result.is_err(),
            "Wrong package name should not produce a valid workspace, got: {:?}",
            result.ok()
        );
        // Specifically, it should be InvalidWorkspace since catalog exists
        match result.unwrap_err() {
            vfa_tui::error::TuiError::InvalidWorkspace { .. } => {}
            other => {
                prop_assert!(
                    false,
                    "Expected InvalidWorkspace error, got: {:?}",
                    other
                );
            }
        }
    }
}
