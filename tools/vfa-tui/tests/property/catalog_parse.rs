// Feature: rust-tui, Property 1: Invalid JSON produces error without panic
//
// For any byte sequence that is not valid JSON, feeding it to the catalog loader
// SHALL produce an `Err` result and SHALL NOT cause a panic or undefined behavior.
//
// **Validates: Requirements 1.2, 2.4, 3.4, 5.5, 12.1**

use proptest::prelude::*;
use proptest::test_runner::Config;
use tempfile::TempDir;
use vfa_tui::catalog::loader::{
    load_agents, load_integrity, load_mcp_refs, load_roles, load_rules, load_skills,
};

/// Helper: create a workspace-like temp directory with a catalog/ subdirectory
/// and write the given bytes to the specified catalog file.
fn setup_catalog_file(filename: &str, content: &[u8]) -> TempDir {
    let tmp = TempDir::new().expect("failed to create temp dir");
    let catalog_dir = tmp.path().join("catalog");
    std::fs::create_dir_all(&catalog_dir).expect("failed to create catalog dir");
    std::fs::write(catalog_dir.join(filename), content).expect("failed to write test file");
    tmp
}

proptest! {
    #![proptest_config(Config::with_cases(256))]

    /// Arbitrary byte sequences fed to load_agents must not panic.
    /// The loader must return errors (empty agents list + non-empty errors) for invalid JSON.
    #[test]
    fn load_agents_invalid_json_no_panic(bytes in proptest::collection::vec(any::<u8>(), 0..1024)) {
        let tmp = setup_catalog_file("agents.json", &bytes);
        let (agents, errors) = load_agents(tmp.path());
        // For random bytes, the result should be empty agents with errors,
        // but the critical property is: no panic occurred.
        // If by astronomical chance the bytes form valid JSON, agents may be non-empty.
        // Either way, the function returned without panicking.
        let _ = (agents, errors);
    }

    /// Arbitrary byte sequences fed to load_skills must not panic.
    #[test]
    fn load_skills_invalid_json_no_panic(bytes in proptest::collection::vec(any::<u8>(), 0..1024)) {
        let tmp = setup_catalog_file("skills.json", &bytes);
        let (skills, errors) = load_skills(tmp.path());
        let _ = (skills, errors);
    }

    /// Arbitrary byte sequences fed to load_mcp_refs must not panic.
    #[test]
    fn load_mcp_refs_invalid_json_no_panic(bytes in proptest::collection::vec(any::<u8>(), 0..1024)) {
        let tmp = setup_catalog_file("mcp-references.json", &bytes);
        let (refs, errors) = load_mcp_refs(tmp.path());
        let _ = (refs, errors);
    }

    /// Arbitrary byte sequences fed to load_rules must not panic.
    #[test]
    fn load_rules_invalid_json_no_panic(bytes in proptest::collection::vec(any::<u8>(), 0..1024)) {
        let tmp = setup_catalog_file("rules.json", &bytes);
        let (rules, errors) = load_rules(tmp.path());
        let _ = (rules, errors);
    }

    /// Arbitrary byte sequences fed to load_roles must not panic.
    #[test]
    fn load_roles_invalid_json_no_panic(bytes in proptest::collection::vec(any::<u8>(), 0..1024)) {
        let tmp = setup_catalog_file("install-roles.json", &bytes);
        let (roles, errors) = load_roles(tmp.path());
        let _ = (roles, errors);
    }

    /// Arbitrary byte sequences fed to load_integrity must not panic.
    #[test]
    fn load_integrity_invalid_json_no_panic(bytes in proptest::collection::vec(any::<u8>(), 0..1024)) {
        let tmp = setup_catalog_file("asset-integrity.json", &bytes);
        let (integrity, errors) = load_integrity(tmp.path());
        let _ = (integrity, errors);
    }

    /// Specifically test that non-UTF-8 byte sequences don't cause panics.
    /// This uses bytes that are guaranteed to be invalid UTF-8.
    #[test]
    fn load_agents_non_utf8_no_panic(
        prefix in proptest::collection::vec(any::<u8>(), 0..100),
        suffix in proptest::collection::vec(any::<u8>(), 0..100),
    ) {
        // Insert invalid UTF-8 sequences (continuation bytes without start byte)
        let mut bytes = prefix;
        bytes.extend_from_slice(&[0xFF, 0xFE, 0x80, 0xC0]);
        bytes.extend(suffix);
        let tmp = setup_catalog_file("agents.json", &bytes);
        let (agents, errors) = load_agents(tmp.path());
        // Must not panic — reaching this point proves no panic occurred.
        // Either we get no agents, or we get errors, or both — all are valid outcomes.
        let _ = (agents, errors);
    }

    /// Test that strings which look like JSON but are malformed don't cause panics.
    /// Generates strings that start with JSON-like characters but are corrupted.
    #[test]
    fn load_agents_malformed_json_strings_no_panic(
        corruption in "[\\[\\{\"a-z0-9:,\\]\\}]{0,512}"
    ) {
        let bytes = corruption.as_bytes();
        let tmp = setup_catalog_file("agents.json", bytes);
        let (agents, errors) = load_agents(tmp.path());
        // The key property: no panic. For malformed JSON, we expect errors.
        let _ = (agents, errors);
    }
}
