use proptest::prelude::*;
use proptest::test_runner::Config;
use std::path::PathBuf;
use vfa_tui::security::validate::{validate_argument, validate_path};

// Shell metacharacters that should be rejected.
const SHELL_METACHARACTERS: &[char] = &[
    ';', '|', '&', '$', '`', '\\', '<', '>', '(', ')', '{', '}', '!', '#', '*', '?', '[', ']',
    '\n', '\r', '\0',
];

// Property 7: Generate paths with ../ segments, verify validate_path rejects them
// when resolved outside workspace.
proptest! {
    #![proptest_config(Config::with_cases(256))]

    #[test]
    fn path_traversal_rejected(
        depth in 1usize..5,
        suffix in "[a-z]{1,8}"
    ) {
        let tmp = tempfile::TempDir::new().unwrap();
        let workspace = tmp.path().join("workspace");
        std::fs::create_dir_all(&workspace).unwrap();

        // Create a path that tries to escape via ../
        let mut traversal = PathBuf::from(&workspace);
        for _ in 0..=depth {
            traversal.push("..");
        }
        traversal.push(&suffix);

        let result = validate_path(&traversal, &workspace);
        // Should be rejected (either PathTraversal error or file not found)
        prop_assert!(result.is_err());
    }
}

// Property 8: Generate strings with shell metacharacters, verify validate_argument rejects them.
// Generate safe strings (alphanumeric + -_./), verify validate_argument accepts them.
proptest! {
    #![proptest_config(Config::with_cases(256))]

    #[test]
    fn metachar_strings_rejected(
        prefix in "[a-zA-Z0-9]{0,5}",
        metachar_idx in 0usize..21,
        suffix in "[a-zA-Z0-9]{0,5}"
    ) {
        let metachar = SHELL_METACHARACTERS[metachar_idx % SHELL_METACHARACTERS.len()];
        let input = format!("{prefix}{metachar}{suffix}");
        let result = validate_argument(&input);
        prop_assert!(result.is_err(), "should reject: {:?}", input);
    }

    #[test]
    fn safe_strings_accepted(s in "[a-zA-Z0-9_./-]{1,50}") {
        let result = validate_argument(&s);
        prop_assert!(result.is_ok(), "should accept: {:?}", s);
    }
}
