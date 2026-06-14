//! Feature: rust-tui-v2, Property 20 — Semantic version comparison.
//! Validates: Requirements 8.1, 8.2, 8.6, 9.1, 27.5
//!
//! `compare_versions` must impose a total order that agrees with numeric
//! `(major, minor, patch)` tuple ordering whenever both operands parse as
//! semver, and `freshness_score` / `round_half_up_1dp` must stay within range
//! and be deterministic.

use std::cmp::Ordering;

use proptest::prelude::*;
use proptest::test_runner::Config;

use vfa_tui::federation::versions::{
    compare_versions, freshness_score, is_stale, parse_semver, round_half_up_1dp, version_delta,
    VersionStatus,
};

/// Render a `(major, minor, patch)` tuple back into a strict semver string.
fn semver_string(v: (u64, u64, u64)) -> String {
    format!("{}.{}.{}", v.0, v.1, v.2)
}

proptest! {
    #![proptest_config(Config::with_cases(512))]

    /// `parse_semver` round-trips any strict `u64.u64.u64` string.
    #[test]
    fn parse_semver_round_trips(major in 0u64..10_000, minor in 0u64..10_000, patch in 0u64..10_000) {
        let s = format!("{major}.{minor}.{patch}");
        prop_assert_eq!(parse_semver(&s), Some((major, minor, patch)));
    }

    /// When both strings parse, `compare_versions` agrees exactly with numeric
    /// tuple ordering (Req 8.2).
    #[test]
    fn compare_agrees_with_tuple_order(
        a in (0u64..50, 0u64..50, 0u64..50),
        b in (0u64..50, 0u64..50, 0u64..50),
    ) {
        let ord = compare_versions(&semver_string(a), &semver_string(b));
        prop_assert_eq!(ord, a.cmp(&b));
    }

    /// `compare_versions` is antisymmetric: cmp(a,b) is the reverse of cmp(b,a).
    #[test]
    fn compare_is_antisymmetric(
        a in (0u64..50, 0u64..50, 0u64..50),
        b in (0u64..50, 0u64..50, 0u64..50),
    ) {
        let sa = semver_string(a);
        let sb = semver_string(b);
        prop_assert_eq!(compare_versions(&sa, &sb), compare_versions(&sb, &sa).reverse());
    }

    /// `compare_versions` is transitive across three semver values.
    #[test]
    fn compare_is_transitive(
        a in (0u64..30, 0u64..30, 0u64..30),
        b in (0u64..30, 0u64..30, 0u64..30),
        c in (0u64..30, 0u64..30, 0u64..30),
    ) {
        let (sa, sb, sc) = (semver_string(a), semver_string(b), semver_string(c));
        if compare_versions(&sa, &sb) != Ordering::Greater
            && compare_versions(&sb, &sc) != Ordering::Greater
        {
            prop_assert_ne!(compare_versions(&sa, &sc), Ordering::Greater);
        }
    }

    /// Equal version strings always compare Equal and yield a Current delta.
    #[test]
    fn equal_versions_are_current(v in (0u64..1000, 0u64..1000, 0u64..1000)) {
        let s = semver_string(v);
        prop_assert_eq!(compare_versions(&s, &s), Ordering::Equal);
        let delta = version_delta(&s, &s);
        prop_assert_eq!(delta.status, VersionStatus::Current);
        prop_assert_eq!((delta.major_delta, delta.minor_delta, delta.patch_delta), (0, 0, 0));
    }

    /// A non-semver operand forces a deterministic case-insensitive lexicographic
    /// fallback (Req 8.6) — never a panic — and `version_delta` reports Unknown.
    #[test]
    fn non_semver_falls_back_without_panic(garbage in "[^0-9.][a-zA-Z0-9 .-]{0,16}") {
        let ord = compare_versions(&garbage, "1.2.3");
        prop_assert_eq!(ord, garbage.to_ascii_lowercase().as_str().cmp("1.2.3"));
        prop_assert_eq!(version_delta(&garbage, "1.2.3").status, VersionStatus::Unknown);
    }

    /// `is_stale` only fires within the same major train and strictly beyond the
    /// threshold; a cross-major divergence is never "stale" (Req 9.1).
    #[test]
    fn is_stale_respects_major_train_and_threshold(
        major in 0u64..10,
        installed_minor in 0u64..50,
        gap in 0u64..50,
        threshold in 0u32..5,
    ) {
        let installed = format!("{major}.{installed_minor}.0");
        let canonical = format!("{major}.{}.0", installed_minor + gap);
        let expected = gap > threshold as u64;
        prop_assert_eq!(is_stale(&installed, &canonical, threshold), expected);

        // Different major is never flagged stale by this function.
        let other_major = format!("{}.{}.0", major + 1, installed_minor + gap);
        prop_assert!(!is_stale(&installed, &other_major, threshold));
    }

    /// `freshness_score` stays within [0, 100] and is monotonic in the numerator.
    #[test]
    fn freshness_score_in_range_and_monotonic(total in 1usize..500, current in 0usize..500) {
        let current = current.min(total);
        let score = freshness_score(current, total);
        prop_assert!((0.0..=100.0).contains(&score), "score {score} out of range");
        if current + 1 <= total {
            prop_assert!(freshness_score(current + 1, total) >= score);
        }
        // Zero total is always 0.0.
        prop_assert_eq!(freshness_score(current, 0), 0.0);
    }

    /// `round_half_up_1dp` rounds to one decimal place with half-up semantics
    /// and is idempotent (Req 27.5).
    #[test]
    fn round_half_up_is_one_dp_and_idempotent(value in 0.0f64..1_000_000.0) {
        let r = round_half_up_1dp(value);
        // At most one decimal digit: r * 10 is (near) an integer.
        let scaled = r * 10.0;
        prop_assert!((scaled - scaled.round()).abs() < 1e-6, "not 1dp: {r}");
        prop_assert_eq!(round_half_up_1dp(r), r);
    }
}

#[test]
fn round_half_up_known_boundaries() {
    assert_eq!(round_half_up_1dp(0.05), 0.1);
    assert_eq!(round_half_up_1dp(0.15), 0.2);
    assert_eq!(round_half_up_1dp(50.05), 50.1);
}
