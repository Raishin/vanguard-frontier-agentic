use proptest::prelude::*;
use proptest::test_runner::Config;
use std::path::{Path, PathBuf};
use vfa_tui::security::validate::{validate_argument, validate_path, validate_registry_path};

// Feature: rust-tui, Property 6: Argument validation rejects shell metacharacters (Req 20.2, 20.3)
// Feature: rust-tui, Property 7: Path validation rejects traversal and unsafe characters (Req 20.5, 22.3)

// Shell metacharacters that should be rejected.
const SHELL_METACHARACTERS: &[char] = &[
    ';', '|', '&', '$', '`', '\\', '<', '>', '(', ')', '{', '}', '!', '#', '*', '?', '[', ']',
    '\n', '\r', '\0',
];

// **Validates: Requirements 8.2, 8.5**
// Property 7: Path validation rejects directory traversal
//
// For any path string, if the canonicalized resolved path references a location
// outside the designated workspace root directory, validate_path SHALL return Err.
// For any path that resolves to a location within the workspace root,
// validate_path SHALL return Ok with the canonical path.
proptest! {
    #![proptest_config(Config::with_cases(256))]

    /// Paths that traverse outside the workspace via ../ segments are rejected.
    #[test]
    fn path_traversal_outside_workspace_rejected(
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
        prop_assert!(result.is_err(),
            "path {:?} should be rejected as outside workspace {:?}",
            traversal, workspace);
    }

    /// Paths within the workspace return Ok with the canonical path.
    #[test]
    fn path_within_workspace_accepted(
        subdir in "[a-z]{1,6}",
        filename in "[a-z]{1,6}"
    ) {
        let tmp = tempfile::TempDir::new().unwrap();
        let workspace = tmp.path().join("workspace");
        std::fs::create_dir_all(&workspace).unwrap();

        // Create a real file inside the workspace
        let dir = workspace.join(&subdir);
        std::fs::create_dir_all(&dir).unwrap();
        let file_path = dir.join(format!("{}.txt", filename));
        std::fs::write(&file_path, "test content").unwrap();

        let result = validate_path(&file_path, &workspace);
        prop_assert!(result.is_ok(),
            "path {:?} within workspace {:?} should be accepted, got {:?}",
            file_path, workspace, result);

        // The returned path should be canonical (absolute, no symlinks)
        let canonical = result.unwrap();
        prop_assert!(canonical.is_absolute(),
            "canonical path {:?} should be absolute", canonical);
        prop_assert!(canonical.starts_with(workspace.canonicalize().unwrap()),
            "canonical path {:?} should start with workspace root", canonical);
    }

    /// Absolute paths outside the workspace are rejected.
    #[test]
    fn absolute_path_outside_workspace_rejected(
        filename in "[a-z]{1,8}"
    ) {
        let workspace_tmp = tempfile::TempDir::new().unwrap();
        let outside_tmp = tempfile::TempDir::new().unwrap();

        let workspace = workspace_tmp.path().join("workspace");
        std::fs::create_dir_all(&workspace).unwrap();

        // Create a file outside the workspace
        let outside_file = outside_tmp.path().join(format!("{}.txt", filename));
        std::fs::write(&outside_file, "secret").unwrap();

        let result = validate_path(&outside_file, &workspace);
        prop_assert!(result.is_err(),
            "absolute path {:?} outside workspace {:?} should be rejected",
            outside_file, workspace);
    }
}

// **Validates: Requirements 8.3**
// Property 8: Argument validation rejects shell metacharacters
//
// For any string, if it contains at least one shell metacharacter
// (;|&$`\<>(){}!#*?[], newline, CR, null byte), validate_argument SHALL return Err.
// For any string composed entirely of safe characters (alphanumeric, hyphen,
// underscore, period, forward slash, space), validate_argument SHALL return Ok.
proptest! {
    #![proptest_config(Config::with_cases(256))]

    /// Strings containing at least one shell metacharacter are rejected.
    #[test]
    fn metachar_strings_rejected(
        prefix in "[a-zA-Z0-9]{0,5}",
        metachar_idx in 0usize..21,
        suffix in "[a-zA-Z0-9]{0,5}"
    ) {
        let metachar = SHELL_METACHARACTERS[metachar_idx % SHELL_METACHARACTERS.len()];
        let input = format!("{prefix}{metachar}{suffix}");
        let result = validate_argument(&input);
        prop_assert!(result.is_err(),
            "should reject string containing metachar {:?}: {:?}", metachar, input);
    }

    /// Strings composed entirely of safe characters (alphanumeric, hyphen,
    /// underscore, period, forward slash, space) are accepted.
    #[test]
    fn safe_strings_accepted(s in "[a-zA-Z0-9_. /-]{1,50}") {
        let result = validate_argument(&s);
        prop_assert!(result.is_ok(),
            "should accept safe string: {:?}, got {:?}", s, result);
    }

    /// Strings with multiple metacharacters are still rejected.
    #[test]
    fn multiple_metachars_rejected(
        idx1 in 0usize..21,
        idx2 in 0usize..21,
        filler in "[a-zA-Z0-9]{0,3}"
    ) {
        let mc1 = SHELL_METACHARACTERS[idx1 % SHELL_METACHARACTERS.len()];
        let mc2 = SHELL_METACHARACTERS[idx2 % SHELL_METACHARACTERS.len()];
        let input = format!("{mc1}{filler}{mc2}");
        let result = validate_argument(&input);
        prop_assert!(result.is_err(),
            "should reject string with multiple metacharacters: {:?}", input);
    }

    /// Empty string is safe (contains no metacharacters).
    #[test]
    fn empty_string_accepted(_dummy in 0..1u8) {
        let result = validate_argument("");
        prop_assert!(result.is_ok(), "empty string should be accepted");
    }
}

// =============================================================================
// Property 7 (extended): validate_registry_path rejects control chars / null bytes
// **Validates: Requirements 20.5, 22.3**
//
// For any path string containing a null byte (U+0000), ASCII control character
// (0x01–0x1F, 0x7F), or Unicode C1 control (U+0080–U+009F), validate_registry_path
// SHALL return Err with a message identifying the specific invalid character class.
// For any path string containing only printable characters outside those ranges,
// validate_registry_path SHALL return Ok.
// =============================================================================

proptest! {
    #![proptest_config(Config::with_cases(256))]

    /// Registry paths with null bytes are rejected (Req 22.3, 20.5).
    #[test]
    fn registry_path_null_byte_rejected(
        prefix in "[a-zA-Z0-9/._-]{1,20}",
        suffix in "[a-zA-Z0-9/._-]{0,20}"
    ) {
        let path_str = format!("{prefix}\x00{suffix}");
        let path = Path::new(&path_str);
        let result = validate_registry_path(path);
        prop_assert!(
            result.is_err(),
            "registry path with null byte should be rejected: {:?}",
            path_str
        );
        // The error message must identify the rejection
        let err_msg = result.unwrap_err().to_string();
        prop_assert!(
            err_msg.contains("null") || err_msg.contains("U+0000") || err_msg.contains("registry"),
            "error should describe the invalid character: {}",
            err_msg
        );
    }

    /// Registry paths with ASCII control characters (0x01-0x1F) are rejected (Req 22.3).
    #[test]
    fn registry_path_ascii_control_rejected(
        prefix in "[a-zA-Z0-9/._-]{1,15}",
        ctrl_byte in 1u32..=0x1Fu32,
        suffix in "[a-zA-Z0-9/._-]{0,15}"
    ) {
        let ctrl_char = char::from_u32(ctrl_byte).unwrap();
        let path_str = format!("{prefix}{ctrl_char}{suffix}");
        let path = Path::new(&path_str);
        let result = validate_registry_path(path);
        prop_assert!(
            result.is_err(),
            "registry path with control char U+{:04X} should be rejected",
            ctrl_byte
        );
    }

    /// Registry paths with 0x7F DEL are rejected (Req 22.3).
    #[test]
    fn registry_path_del_rejected(
        prefix in "[a-zA-Z0-9/._-]{1,15}",
        suffix in "[a-zA-Z0-9/._-]{0,15}"
    ) {
        let path_str = format!("{prefix}\x7F{suffix}");
        let path = Path::new(&path_str);
        let result = validate_registry_path(path);
        prop_assert!(
            result.is_err(),
            "registry path with DEL (0x7F) should be rejected"
        );
    }

    /// Registry paths with Unicode C1 controls (U+0080–U+009F) are rejected (Req 22.3).
    #[test]
    fn registry_path_c1_control_rejected(
        prefix in "[a-zA-Z0-9/._-]{1,15}",
        c1_byte in 0x80u32..=0x9Fu32,
        suffix in "[a-zA-Z0-9/._-]{0,15}"
    ) {
        let c1_char = char::from_u32(c1_byte).unwrap();
        let path_str = format!("{prefix}{c1_char}{suffix}");
        let path = Path::new(&path_str);
        let result = validate_registry_path(path);
        prop_assert!(
            result.is_err(),
            "registry path with C1 control U+{:04X} should be rejected",
            c1_byte
        );
    }

    /// Registry paths containing only printable ASCII characters are accepted (Req 22.3).
    #[test]
    fn registry_path_printable_ascii_accepted(
        path_str in "[a-zA-Z0-9/._-]{1,40}"
    ) {
        let path = Path::new(&path_str);
        let result = validate_registry_path(path);
        prop_assert!(
            result.is_ok(),
            "registry path with only printable ASCII should be accepted: {:?}, got {:?}",
            path_str,
            result
        );
    }

    /// validate_path rejects paths with embedded null bytes (Req 20.5).
    /// Note: null bytes in path strings are unusual but must be explicitly rejected.
    #[test]
    fn validate_path_null_byte_in_path_rejected(
        prefix in "[a-z]{1,8}",
        suffix in "[a-z]{0,8}"
    ) {
        let tmp = tempfile::TempDir::new().unwrap();
        let workspace = tmp.path().join("workspace");
        std::fs::create_dir_all(&workspace).unwrap();

        // Construct a path string with embedded null byte
        let path_str = format!("{prefix}\x00{suffix}");
        let path = PathBuf::from(&path_str);
        let result = validate_path(&path, &workspace);
        prop_assert!(
            result.is_err(),
            "path with null byte should be rejected"
        );
    }
}
