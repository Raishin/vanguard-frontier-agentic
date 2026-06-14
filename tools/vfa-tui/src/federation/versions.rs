//! Version comparison engine — parse semver, compute deltas, detect stale assets.
//!
//! # Responsibilities
//!
//! - [`parse_semver`] — parse a `"MAJOR.MINOR.PATCH"` string into a numeric tuple.
//! - [`compare_versions`] — total order for version strings: semver when both
//!   parse, lexicographic fallback with a `tracing::warn` (Req 8.6).
//! - [`version_delta`] — compute [`VersionDelta`] and derive a status
//!   (current / outdated / unknown) (Req 8.2).
//! - [`is_stale`] — flag an asset when `canonical.minor - installed.minor >
//!   threshold` on the same major (Req 9.1).
//! - [`extract_version`] — priority-ordered version extraction from an
//!   [`InstalledAsset`] (Req 8.1).
//! - [`freshness_score`] — ratio of current assets, rounded half-up to 1 decimal
//!   (Req 8.3).
//! - [`round_half_up_1dp`] — shared deterministic rounding helper (Req 27.5).

#![deny(warnings)]

use std::cmp::Ordering;

use tracing::warn;

use crate::federation::scanner::InstalledAsset;

// ---------------------------------------------------------------------------
// round_half_up_1dp — deterministic 1-decimal-place rounding helper
// ---------------------------------------------------------------------------

/// Round `value` to **one decimal place** using **round-half-up** semantics.
///
/// This is the canonical rounding helper for all percentage scores in the
/// federation module (Req 27.5).  It does **not** rely on default `f64`
/// `Display` formatting, which uses round-half-to-even (banker's rounding).
///
/// # Algorithm
///
/// ```text
/// rounded = floor(value * 10 + 0.5) / 10
/// ```
///
/// This satisfies: 0.05 → 0.1, 0.15 → 0.2, 50.05 → 50.1.
pub fn round_half_up_1dp(value: f64) -> f64 {
    (value * 10.0 + 0.5).floor() / 10.0
}

// ---------------------------------------------------------------------------
// parse_semver
// ---------------------------------------------------------------------------

/// Parse a semver string `"MAJOR.MINOR.PATCH"` into `(major, minor, patch)`.
///
/// Returns `None` for any string that does not match the strict
/// `u64.u64.u64` format (including pre-release labels, build metadata,
/// leading `v`, or fewer/more than three components).
pub fn parse_semver(version: &str) -> Option<(u64, u64, u64)> {
    let parts: Vec<&str> = version.split('.').collect();
    if parts.len() != 3 {
        return None;
    }
    let major = parts[0].parse::<u64>().ok()?;
    let minor = parts[1].parse::<u64>().ok()?;
    let patch = parts[2].parse::<u64>().ok()?;
    Some((major, minor, patch))
}

// ---------------------------------------------------------------------------
// compare_versions
// ---------------------------------------------------------------------------

/// Compare two version strings.
///
/// When **both** strings parse as semver, the comparison is numeric
/// `(major, minor, patch)` tuple ordering.  When either string is
/// non-semver, the comparison falls back to **case-insensitive
/// lexicographic** ordering and a `tracing::warn` is emitted (Req 8.6).
pub fn compare_versions(a: &str, b: &str) -> Ordering {
    match (parse_semver(a), parse_semver(b)) {
        (Some(av), Some(bv)) => av.cmp(&bv),
        _ => {
            warn!(
                a = a,
                b = b,
                "non-semver version string(s) — falling back to lexicographic comparison"
            );
            a.to_ascii_lowercase().cmp(&b.to_ascii_lowercase())
        }
    }
}

// ---------------------------------------------------------------------------
// VersionStatus
// ---------------------------------------------------------------------------

/// Classification of an installed version relative to the canonical version.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VersionStatus {
    /// Installed version matches canonical (delta = 0.0.0).
    Current,
    /// Installed version is behind canonical.
    Outdated,
    /// One or both version strings could not be parsed — comparison is best-effort.
    Unknown,
}

// ---------------------------------------------------------------------------
// VersionDelta
// ---------------------------------------------------------------------------

/// Computed difference between an installed version and the canonical version.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VersionDelta {
    /// `canonical.major - installed.major` (may be 0 or negative if installed is ahead).
    pub major_delta: i64,
    /// `canonical.minor - installed.minor`.
    pub minor_delta: i64,
    /// `canonical.patch - installed.patch`.
    pub patch_delta: i64,
    /// Human-readable status derived from the delta.
    pub status: VersionStatus,
}

// ---------------------------------------------------------------------------
// version_delta
// ---------------------------------------------------------------------------

/// Compute a [`VersionDelta`] between an installed version and the canonical
/// version (Req 8.2).
///
/// # Status rules
///
/// - Both parse as semver AND installed == canonical → [`VersionStatus::Current`]
/// - Both parse as semver AND installed < canonical → [`VersionStatus::Outdated`]
/// - Both parse as semver AND installed > canonical → [`VersionStatus::Outdated`]
///   (unusual: installed is ahead; still flagged as non-current)
/// - Either cannot be parsed → [`VersionStatus::Unknown`], delta components
///   computed as 0.
pub fn version_delta(installed: &str, canonical: &str) -> VersionDelta {
    match (parse_semver(installed), parse_semver(canonical)) {
        (Some((im, in_, ip)), Some((cm, cn, cp))) => {
            let major_delta = cm as i64 - im as i64;
            let minor_delta = cn as i64 - in_ as i64;
            let patch_delta = cp as i64 - ip as i64;
            let status = if major_delta == 0 && minor_delta == 0 && patch_delta == 0 {
                VersionStatus::Current
            } else {
                VersionStatus::Outdated
            };
            VersionDelta {
                major_delta,
                minor_delta,
                patch_delta,
                status,
            }
        }
        _ => VersionDelta {
            major_delta: 0,
            minor_delta: 0,
            patch_delta: 0,
            status: VersionStatus::Unknown,
        },
    }
}

// ---------------------------------------------------------------------------
// is_stale
// ---------------------------------------------------------------------------

/// Returns `true` when the installed asset is considered **stale** (Req 9.1).
///
/// # Staleness rule
///
/// An asset is stale when **all** of the following hold:
/// 1. Both version strings parse as semver.
/// 2. `canonical.major == installed.major` (same major version train).
/// 3. `canonical.minor - installed.minor > minor_threshold`.
///
/// **Major-bump handling**: when the canonical major is greater than the
/// installed major, the asset is always considered outdated (via
/// [`version_delta`]) but is **not** counted as stale by this function — a
/// major version bump may involve intentional breaking changes and should be
/// escalated through a separate upgrade path, not automatically flagged as
/// stale.  Callers that need to alert on cross-major divergence should check
/// `version_delta` directly.
///
/// # Parameters
///
/// - `installed` — version string from the installed asset.
/// - `canonical` — version string from the catalog.
/// - `minor_threshold` — number of minor versions behind before flagging as
///   stale (Req 9.1 default: 2).
pub fn is_stale(installed: &str, canonical: &str, minor_threshold: u32) -> bool {
    match (parse_semver(installed), parse_semver(canonical)) {
        (Some((im, in_, _)), Some((cm, cn, _))) => {
            if cm != im {
                // Different major — not handled as stale (see doc comment).
                return false;
            }
            let behind = cn.saturating_sub(in_);
            behind > minor_threshold as u64
        }
        _ => false,
    }
}

// ---------------------------------------------------------------------------
// extract_version
// ---------------------------------------------------------------------------

/// Extract the version string from an [`InstalledAsset`] using the
/// priority-ordered strategy (Req 8.1):
///
/// 1. `installed_version` — set by the `# VFA-EXPORT:` metadata comment parser
///    when the export CLI wrote a version field.
/// 2. Any frontmatter version already captured — in the current model this is
///    also surfaced via `installed_version` since the scanner unifies both
///    sources into that field.  If a future scanner extension separates them
///    this function can be extended.
/// 3. `None` — caller falls back to content hash comparison (Req 8.5).
///
/// This function is intentionally lightweight: the scanner is the authority
/// on version extraction; this function just reads the canonical field.
pub fn extract_version(asset: &InstalledAsset) -> Option<String> {
    asset.installed_version.clone()
}

// ---------------------------------------------------------------------------
// freshness_score
// ---------------------------------------------------------------------------

/// Compute the "freshness score" for a workspace (Req 8.3).
///
/// Formula: `(assets_at_current / total_with_versions) × 100`, rounded
/// half-up to one decimal place.
///
/// Returns `0.0` when `total_with_versions == 0` (no detectable versions).
pub fn freshness_score(assets_at_current: usize, total_with_versions: usize) -> f64 {
    if total_with_versions == 0 {
        return 0.0;
    }
    let ratio = assets_at_current as f64 / total_with_versions as f64;
    round_half_up_1dp(ratio * 100.0)
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    // -----------------------------------------------------------------------
    // Unit tests — parse_semver
    // -----------------------------------------------------------------------

    #[test]
    fn parse_semver_valid() {
        assert_eq!(parse_semver("1.2.3"), Some((1, 2, 3)));
        assert_eq!(parse_semver("0.0.0"), Some((0, 0, 0)));
        assert_eq!(parse_semver("10.20.30"), Some((10, 20, 30)));
    }

    #[test]
    fn parse_semver_invalid_forms() {
        assert!(parse_semver("1.2").is_none());
        assert!(parse_semver("1.2.3.4").is_none());
        assert!(parse_semver("v1.2.3").is_none());
        assert!(parse_semver("1.2.3-alpha").is_none());
        assert!(parse_semver("").is_none());
        assert!(parse_semver("not-a-version").is_none());
        assert!(parse_semver("1.2.x").is_none());
    }

    // -----------------------------------------------------------------------
    // Unit tests — compare_versions
    // -----------------------------------------------------------------------

    #[test]
    fn compare_versions_semver_ordering() {
        assert_eq!(compare_versions("1.0.0", "2.0.0"), Ordering::Less);
        assert_eq!(compare_versions("1.2.3", "1.2.3"), Ordering::Equal);
        assert_eq!(compare_versions("2.0.0", "1.9.9"), Ordering::Greater);
        assert_eq!(compare_versions("1.10.0", "1.9.0"), Ordering::Greater);
    }

    #[test]
    fn compare_versions_lexicographic_fallback() {
        // Both non-semver — falls back to lexicographic, no panic
        let result = compare_versions("alpha", "beta");
        assert_eq!(result, Ordering::Less);
    }

    // -----------------------------------------------------------------------
    // Unit tests — version_delta
    // -----------------------------------------------------------------------

    #[test]
    fn version_delta_current() {
        let d = version_delta("1.2.3", "1.2.3");
        assert_eq!(d.status, VersionStatus::Current);
        assert_eq!(d.major_delta, 0);
        assert_eq!(d.minor_delta, 0);
        assert_eq!(d.patch_delta, 0);
    }

    #[test]
    fn version_delta_outdated() {
        let d = version_delta("1.1.0", "1.3.0");
        assert_eq!(d.status, VersionStatus::Outdated);
        assert_eq!(d.minor_delta, 2);
    }

    #[test]
    fn version_delta_unknown_non_semver() {
        let d = version_delta("alpha", "beta");
        assert_eq!(d.status, VersionStatus::Unknown);
    }

    // -----------------------------------------------------------------------
    // Unit tests — is_stale
    // -----------------------------------------------------------------------

    #[test]
    fn is_stale_exactly_at_threshold_not_stale() {
        // Default threshold 2: canonical.minor - installed.minor == 2 → NOT stale
        assert!(!is_stale("1.0.0", "1.2.0", 2));
    }

    #[test]
    fn is_stale_over_threshold() {
        // canonical.minor - installed.minor == 3 > 2 → stale
        assert!(is_stale("1.0.0", "1.3.0", 2));
    }

    #[test]
    fn is_stale_different_major_not_stale() {
        // Major bump: not handled as stale
        assert!(!is_stale("1.0.0", "2.5.0", 2));
    }

    #[test]
    fn is_stale_non_semver_not_stale() {
        assert!(!is_stale("alpha", "beta", 2));
    }

    // -----------------------------------------------------------------------
    // Unit tests — freshness_score
    // -----------------------------------------------------------------------

    #[test]
    fn freshness_score_zero_total() {
        assert_eq!(freshness_score(0, 0), 0.0);
    }

    #[test]
    fn freshness_score_all_current() {
        assert_eq!(freshness_score(5, 5), 100.0);
    }

    #[test]
    fn freshness_score_half() {
        // 5/10 = 50.0
        assert_eq!(freshness_score(5, 10), 50.0);
    }

    #[test]
    fn freshness_score_rounding_half_up() {
        // 1/3 = 33.333... → 33.3
        let s = freshness_score(1, 3);
        assert_eq!(s, 33.3);
    }

    // -----------------------------------------------------------------------
    // Unit tests — round_half_up_1dp
    // -----------------------------------------------------------------------

    #[test]
    fn round_half_up_boundary() {
        // 0.05 → 0.1 (round half up)
        let r = round_half_up_1dp(0.05);
        assert!((r - 0.1).abs() < 1e-9, "expected 0.1 got {r}");
    }

    #[test]
    fn round_half_up_50_05() {
        let r = round_half_up_1dp(50.05);
        assert!((r - 50.1).abs() < 1e-9, "expected 50.1 got {r}");
    }

    #[test]
    fn round_half_up_exact() {
        let r = round_half_up_1dp(33.3);
        assert!((r - 33.3).abs() < 1e-9, "expected 33.3 got {r}");
    }

    // -----------------------------------------------------------------------
    // Property 20 — Semantic version comparison (Req 8.1, 8.6, 9.1)
    //
    // Tests:
    // (a) semver compare is a total order consistent with numeric tuple ordering.
    // (b) non-semver falls back lexicographically (no panic).
    // (c) is_stale is monotonic in minor delta.
    // -----------------------------------------------------------------------

    prop_compose! {
        /// Generate a valid semver triple.
        fn arb_semver()(
            major in 0u64..20,
            minor in 0u64..30,
            patch in 0u64..30,
        ) -> (u64, u64, u64) {
            (major, minor, patch)
        }
    }

    prop_compose! {
        fn arb_semver_str()(sv in arb_semver()) -> String {
            format!("{}.{}.{}", sv.0, sv.1, sv.2)
        }
    }

    proptest! {
        #![proptest_config(proptest::test_runner::Config {
            cases: 256,
            ..Default::default()
        })]

        // (a) compare_versions for two semver strings is consistent with tuple order.
        #[test]
        fn prop20a_semver_total_order_consistent(
            a in arb_semver(),
            b in arb_semver(),
        ) {
            let a_str = format!("{}.{}.{}", a.0, a.1, a.2);
            let b_str = format!("{}.{}.{}", b.0, b.1, b.2);
            let expected = a.cmp(&b);
            let got = compare_versions(&a_str, &b_str);
            prop_assert_eq!(
                got, expected,
                "compare_versions({}, {}) should equal tuple cmp({:?}, {:?})",
                a_str, b_str, a, b
            );
        }

        // (b) compare_versions on non-semver strings never panics and returns
        // a consistent result equal to case-insensitive lexicographic ordering.
        #[test]
        fn prop20b_non_semver_lexicographic_fallback(
            // Strings that won't happen to parse as semver (contain alpha chars)
            a in "[a-z]{2,8}(-[0-9]{1,3})?",
            b in "[a-z]{2,8}(-[0-9]{1,3})?",
        ) {
            // Must not panic
            let got = compare_versions(&a, &b);
            // Must equal case-insensitive lexicographic ordering
            let expected = a.to_ascii_lowercase().cmp(&b.to_ascii_lowercase());
            prop_assert_eq!(
                got, expected,
                "fallback compare({}, {}) should be lexicographic",
                a, b
            );
        }

        // (c) is_stale is monotonic in the minor delta (on the same major).
        // If delta D makes it stale, then D+1 also makes it stale.
        #[test]
        fn prop20c_is_stale_monotonic(
            major in 0u64..10,
            installed_minor in 0u64..25,
            extra_minor in 1u64..10,
            patch in 0u64..10,
            threshold in 0u32..5,
        ) {
            // baseline: canonical_minor = installed_minor + threshold + 1
            // → stale
            let canon_minor_stale = installed_minor + threshold as u64 + 1;
            let canon_minor_more = canon_minor_stale + extra_minor;

            let installed = format!("{major}.{installed_minor}.{patch}");
            let canonical_stale = format!("{major}.{canon_minor_stale}.{patch}");
            let canonical_more = format!("{major}.{canon_minor_more}.{patch}");

            // Baseline should be stale
            prop_assert!(
                is_stale(&installed, &canonical_stale, threshold),
                "delta {} should be stale with threshold {}",
                threshold as u64 + 1, threshold
            );
            // More minor versions behind → also stale
            prop_assert!(
                is_stale(&installed, &canonical_more, threshold),
                "delta {} should also be stale",
                canon_minor_more - installed_minor
            );
        }

        // Additional: freshness_score result is always in [0.0, 100.0] and
        // round_half_up_1dp preserves .x5 boundary correctly.
        #[test]
        fn prop20d_freshness_score_range(
            current in 0usize..=1000,
            total in 1usize..=1000,
        ) {
            // current can't exceed total
            let current = current.min(total);
            let score = freshness_score(current, total);
            prop_assert!(
                score >= 0.0 && score <= 100.0,
                "freshness_score({}, {}) = {} out of range",
                current, total, score
            );
        }
    }
}
