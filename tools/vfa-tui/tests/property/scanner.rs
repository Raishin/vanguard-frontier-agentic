//! Feature: rust-tui-v2, Properties 18 & 19 — multi-strategy confirmation and
//! VFA-EXPORT metadata parsing.
//! Validates: Requirements 7.2, 7.x (export metadata)
//!
//! P18: `is_confirmed` is true exactly when ≥2 *distinct* detection strategies
//!      fired; `match_content_signature` only fires on real overlap.
//! P19: `parse_export_metadata` round-trips a well-formed `# VFA-EXPORT:` /
//!      `// VFA-EXPORT:` JSON header and never panics on malformed input.

use proptest::prelude::*;
use proptest::test_runner::Config;

use vfa_tui::federation::scanner::{is_confirmed, DetectionMethod, ExportMeta, WorkspaceScanner};

fn arb_method() -> impl Strategy<Value = DetectionMethod> {
    prop_oneof![
        Just(DetectionMethod::Filename),
        Just(DetectionMethod::MetadataComment),
        Just(DetectionMethod::ContentSignature),
    ]
}

proptest! {
    #![proptest_config(Config::with_cases(256))]

    /// P18 — confirmation requires ≥2 distinct strategies; duplicates collapse.
    #[test]
    fn is_confirmed_counts_distinct(methods in proptest::collection::vec(arb_method(), 0..8)) {
        let distinct = methods.iter().collect::<std::collections::HashSet<_>>().len();
        prop_assert_eq!(is_confirmed(&methods), distinct >= 2);
    }

    /// P18 — repeating a single strategy never confirms, no matter how many times.
    #[test]
    fn repeated_single_strategy_never_confirms(n in 1usize..10, which in 0usize..3) {
        let m = [
            DetectionMethod::Filename,
            DetectionMethod::MetadataComment,
            DetectionMethod::ContentSignature,
        ][which]
            .clone();
        let methods = vec![m; n];
        prop_assert!(!is_confirmed(&methods));
    }

    /// P18 — identical content with ≥3 distinct non-empty lines matches; content
    /// with <3 lines never matches.
    #[test]
    fn content_signature_identical_matches(n in 0usize..10) {
        let lines: Vec<String> = (0..n).map(|i| format!("line number {i} content")).collect();
        let blob = lines.join("\n");
        let matched = WorkspaceScanner::match_content_signature(&blob, &blob);
        // Identical blobs match iff there are at least 3 distinct non-empty lines.
        prop_assert_eq!(matched, n >= 3);
    }

    /// P18 — fully disjoint content never matches (overlap 0 < 0.40).
    #[test]
    fn content_signature_disjoint_never_matches(n in 3usize..10, m in 3usize..10) {
        let a = (0..n).map(|i| format!("alpha {i}")).collect::<Vec<_>>().join("\n");
        let b = (0..m).map(|i| format!("bravo {i}")).collect::<Vec<_>>().join("\n");
        prop_assert!(!WorkspaceScanner::match_content_signature(&a, &b));
    }

    /// P19 — a well-formed VFA-EXPORT header (either comment style) round-trips.
    #[test]
    fn export_metadata_round_trips(
        id in "[a-zA-Z0-9/_-]{1,24}",
        version in proptest::option::of("[0-9]{1,2}\\.[0-9]{1,2}\\.[0-9]{1,2}"),
        installed_at in proptest::option::of("[0-9T:Z-]{1,24}"),
        use_slash in any::<bool>(),
    ) {
        let meta = ExportMeta {
            id,
            version,
            installed_at,
        };
        let json = serde_json::to_string(&meta).unwrap();
        let marker = if use_slash { "//" } else { "#" };
        let content = format!("{marker} VFA-EXPORT: {json}\nsome other content\n");
        prop_assert_eq!(WorkspaceScanner::parse_export_metadata(&content), Some(meta));
    }

    /// P19 — a marker beyond the first 20 lines is ignored.
    #[test]
    fn export_metadata_only_in_header(pad in 20usize..40) {
        let meta = ExportMeta { id: "agents/x".into(), version: None, installed_at: None };
        let json = serde_json::to_string(&meta).unwrap();
        let prefix = "\n".repeat(pad);
        let content = format!("{prefix}# VFA-EXPORT: {json}\n");
        prop_assert_eq!(WorkspaceScanner::parse_export_metadata(&content), None);
    }

    /// P19 — arbitrary text without a valid marker never yields metadata and
    /// never panics.
    #[test]
    fn export_metadata_robust_on_garbage(content in "[ -~\\n]{0,200}") {
        // We only require no panic; if it parses, it must at least contain the
        // marker substring somewhere in the first 20 lines.
        let parsed = WorkspaceScanner::parse_export_metadata(&content);
        if parsed.is_some() {
            let header: String = content.lines().take(20).collect::<Vec<_>>().join("\n");
            prop_assert!(header.contains("VFA-EXPORT:"));
        }
    }
}
