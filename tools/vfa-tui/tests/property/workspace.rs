use proptest::prelude::*;
use proptest::test_runner::Config;
use std::fs;
use tempfile::TempDir;
use vfa_tui::workspace::detect_workspace;

// Property 16: Create temp dir trees, verify workspace detection finds correct root
// or returns error.
proptest! {
    #![proptest_config(Config::with_cases(64))]

    #[test]
    fn valid_workspace_found(depth in 1usize..5) {
        let tmp = TempDir::new().unwrap();
        let root = tmp.path();

        // Create valid workspace at root
        let catalog_dir = root.join("catalog");
        fs::create_dir_all(&catalog_dir).unwrap();
        fs::write(catalog_dir.join("agents.json"), "[]").unwrap();
        fs::write(
            root.join("package.json"),
            r#"{"name": "@raishin/vanguard-frontier-agentic"}"#,
        ).unwrap();

        // Create nested subdirectories
        let mut sub = root.to_path_buf();
        for i in 0..depth {
            sub = sub.join(format!("sub{i}"));
        }
        fs::create_dir_all(&sub).unwrap();

        let result = detect_workspace(Some(&sub));
        prop_assert!(result.is_ok(), "should find workspace from {:?}", sub);
        prop_assert_eq!(result.unwrap(), root.to_path_buf());
    }

    #[test]
    fn invalid_workspace_not_found(depth in 1usize..4) {
        let tmp = TempDir::new().unwrap();
        let root = tmp.path();

        // Create nested subdirectories without a valid workspace
        let mut sub = root.to_path_buf();
        for i in 0..depth {
            sub = sub.join(format!("sub{i}"));
        }
        fs::create_dir_all(&sub).unwrap();

        let result = detect_workspace(Some(&sub));
        prop_assert!(result.is_err(), "should NOT find workspace from {:?}", sub);
    }
}
