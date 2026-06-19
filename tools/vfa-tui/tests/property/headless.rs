//! Feature: rust-tui-v2, Properties 26 & 32 — headless exit codes and
//! always-present status text indicators.
//! Validates: Requirements 11.x (headless exit semantics), accessibility
//!
//! P26: `compute_exit_code` returns the maximum finding severity (0 when empty),
//!      so a clean run exits 0 and the worst finding dominates.
//! P32: every status indicator helper embeds a plain-text marker that survives
//!      regardless of color, and the marker text is always present.

use proptest::prelude::*;
use proptest::test_runner::Config;

use vfa_tui::headless::formats::{
    with_status, STATUS_DRIFT, STATUS_FAIL, STATUS_MISSING, STATUS_PASS, STATUS_STALE, STATUS_WARN,
};
use vfa_tui::headless::reporter::{compute_exit_code, FindingSeverity};

fn arb_severity() -> impl Strategy<Value = FindingSeverity> {
    prop_oneof![
        Just(FindingSeverity::Success),
        Just(FindingSeverity::Compliance),
        Just(FindingSeverity::Operational),
        Just(FindingSeverity::PartialCatalog),
    ]
}

const ALL_INDICATORS: &[&str] = &[
    STATUS_PASS,
    STATUS_FAIL,
    STATUS_WARN,
    STATUS_DRIFT,
    STATUS_STALE,
    STATUS_MISSING,
];

proptest! {
    #![proptest_config(Config::with_cases(256))]

    /// P26 — exit code equals the maximum severity discriminant, 0 when empty.
    #[test]
    fn exit_code_is_max_severity(findings in proptest::collection::vec(arb_severity(), 0..16)) {
        let expected = findings.iter().map(|f| *f as u8).max().unwrap_or(0);
        prop_assert_eq!(compute_exit_code(&findings), expected);
    }

    /// P26 — a clean run (all Success, or empty) exits 0; any non-Success
    /// finding yields a non-zero code.
    #[test]
    fn success_only_is_zero_else_nonzero(findings in proptest::collection::vec(arb_severity(), 0..16)) {
        let code = compute_exit_code(&findings);
        let all_success = findings.iter().all(|f| *f as u8 == 0);
        if all_success {
            prop_assert_eq!(code, 0);
        } else {
            prop_assert!(code > 0);
        }
    }

    /// P32 — `with_status` always contains both the indicator marker and the
    /// payload text, for every indicator.
    #[test]
    fn with_status_always_embeds_marker_and_text(
        idx in 0usize..ALL_INDICATORS.len(),
        text in "[ -~]{0,40}",
    ) {
        let indicator = ALL_INDICATORS[idx];
        let rendered = with_status(indicator, &text);
        prop_assert!(rendered.contains(indicator), "missing marker {indicator} in {rendered:?}");
        prop_assert!(rendered.contains(&text), "missing text in {rendered:?}");
    }
}

#[test]
fn status_markers_are_distinct_and_bracketed() {
    for m in ALL_INDICATORS {
        assert!(
            m.starts_with('[') && m.ends_with(']'),
            "marker {m} not bracketed"
        );
    }
    let unique: std::collections::HashSet<_> = ALL_INDICATORS.iter().collect();
    assert_eq!(
        unique.len(),
        ALL_INDICATORS.len(),
        "status markers must be distinct"
    );
}

#[test]
fn empty_findings_exit_zero() {
    assert_eq!(compute_exit_code(&[]), 0);
}
