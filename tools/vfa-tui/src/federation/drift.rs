//! Drift detection engine — Tasks 7.3 & 7.4 (Req 10.1–10.5).
//!
//! # Responsibilities
//!
//! - [`DriftKind`] — discriminates no drift, content drift, and version drift.
//! - [`classify_drift`] — pure function classifying a single (installed, canonical)
//!   pair by comparing hash and version fields (Req 10.1, 10.3).
//! - [`DriftRecord`] — carries all diagnostic fields for one drifted asset.
//! - [`detect_drift`] — batch detection over a slice of [`InstalledAsset`]s,
//!   returning a deterministically sorted [`Vec<DriftRecord>`] (Req 10.3, 10.4).
//!
//! # Drift taxonomy (Req 10.3)
//!
//! | Hash match | Version same | Result          |
//! |-----------|-------------|-----------------|
//! | Yes        | (any)        | `None`          |
//! | No         | Yes (or both None) | `ContentDrift`  |
//! | No         | No           | `VersionDrift`  |
//!
//! "Version same" means both installed and canonical versions are `Some` and equal,
//! **or** both are `None` (no version metadata on either side).  Any other
//! combination (one Some, one None, or both Some but different) is treated as
//! "versions differ" → `VersionDrift`.
//!
//! # Headless exit-code contract (Req 10.4)
//!
//! - `ContentDrift` → exit code 1 (unauthorized modification).
//! - `VersionDrift` → exit code 0 (intentional update).
//!
//! The exit-code logic lives in the headless reporter, not here.

#![deny(warnings)]

use std::collections::HashMap;
use std::path::PathBuf;

use crate::federation::scanner::InstalledAsset;

// ---------------------------------------------------------------------------
// DriftKind
// ---------------------------------------------------------------------------

/// Classification of drift for a single installed asset.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DriftKind {
    /// Content hash matches canonical — no drift.
    None,
    /// Content hash differs AND version metadata matches (or is absent on both
    /// sides) — the asset was modified without a version bump (Req 10.3).
    ContentDrift,
    /// Content hash differs AND version strings differ — the asset was
    /// intentionally updated to a different version (Req 10.3).
    VersionDrift,
}

// ---------------------------------------------------------------------------
// classify_drift
// ---------------------------------------------------------------------------

/// Classify drift for a single installed asset against its canonical state.
///
/// # Arguments
///
/// - `installed` — the detected installed asset.
/// - `canonical_hash` — expected SHA-256 hex from `asset-integrity.json`, or
///   `None` if the asset has no entry in the manifest.
/// - `canonical_version` — canonical version string from the catalog, or
///   `None` if no version is recorded.
///
/// # Classification rules (Req 10.3)
///
/// 1. If `canonical_hash` is `None` → treat as no canonical baseline, return
///    [`DriftKind::None`] (can't determine drift without a reference hash).
/// 2. If `installed.content_hash == canonical_hash` → [`DriftKind::None`].
/// 3. Hashes differ — compare versions:
///    - Both versions are `None` **or** both are `Some` and equal →
///      [`DriftKind::ContentDrift`].
///    - Otherwise (one is `None` while the other is `Some`, or both `Some` but
///      different) → [`DriftKind::VersionDrift`].
pub fn classify_drift(
    installed: &InstalledAsset,
    canonical_hash: Option<&str>,
    canonical_version: Option<&str>,
) -> DriftKind {
    let canon_hash = match canonical_hash {
        Some(h) => h,
        None => return DriftKind::None,
    };

    if installed.content_hash == canon_hash {
        return DriftKind::None;
    }

    // Hashes differ — determine drift kind from version metadata.
    let installed_ver = installed.installed_version.as_deref();

    let versions_same = match (installed_ver, canonical_version) {
        (None, None) => true,
        (Some(iv), Some(cv)) => iv == cv,
        _ => false,
    };

    if versions_same {
        DriftKind::ContentDrift
    } else {
        DriftKind::VersionDrift
    }
}

// ---------------------------------------------------------------------------
// DriftRecord
// ---------------------------------------------------------------------------

/// Diagnostic record for a single drifted asset (Req 10.2).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DriftRecord {
    /// Absolute path to the installed file.
    pub workspace_path: PathBuf,
    /// Canonical asset identifier.
    pub asset_id: String,
    /// Expected SHA-256 hex from `asset-integrity.json`.
    pub expected_hash: String,
    /// Actual SHA-256 hex computed from the installed file.
    pub actual_hash: String,
    /// Drift classification.
    pub kind: DriftKind,
}

// ---------------------------------------------------------------------------
// detect_drift
// ---------------------------------------------------------------------------

/// Run drift detection over a batch of installed assets.
///
/// # Arguments
///
/// - `installed` — slice of all installed assets across one or more
///   workspaces.
/// - `canonical_hashes` — map from `asset_id` → expected SHA-256 hex.
/// - `canonical_versions` — map from `asset_id` → canonical version string.
///
/// Only assets that are **confirmed installed** (`asset.confirmed == true`) are
/// evaluated; unconfirmed detections are excluded to reduce false positives.
///
/// The returned [`Vec<DriftRecord>`] is sorted deterministically by
/// `(asset_id, workspace_path)` so that callers get stable output regardless
/// of scan ordering (Req 27.2).
pub fn detect_drift(
    installed: &[InstalledAsset],
    canonical_hashes: &HashMap<String, String>,
    canonical_versions: &HashMap<String, String>,
) -> Vec<DriftRecord> {
    let mut records: Vec<DriftRecord> = installed
        .iter()
        .filter(|asset| asset.confirmed)
        .filter_map(|asset| {
            let canon_hash = canonical_hashes.get(&asset.asset_id).map(|s| s.as_str());
            let canon_version = canonical_versions.get(&asset.asset_id).map(|s| s.as_str());

            let kind = classify_drift(asset, canon_hash, canon_version);
            if kind == DriftKind::None {
                return Option::None;
            }

            let expected_hash = canon_hash.unwrap_or("").to_string();
            Some(DriftRecord {
                workspace_path: asset.workspace_path.clone(),
                asset_id: asset.asset_id.clone(),
                expected_hash,
                actual_hash: asset.content_hash.clone(),
                kind,
            })
        })
        .collect();

    records.sort_by(|a, b| {
        a.asset_id
            .cmp(&b.asset_id)
            .then_with(|| a.workspace_path.cmp(&b.workspace_path))
    });

    records
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    // -----------------------------------------------------------------------
    // Helpers
    // -----------------------------------------------------------------------

    fn make_asset(
        asset_id: &str,
        content_hash: &str,
        installed_version: Option<&str>,
    ) -> InstalledAsset {
        use crate::federation::scanner::DetectionMethod;
        InstalledAsset {
            workspace_path: PathBuf::from(format!("/ws/{asset_id}")),
            asset_id: asset_id.to_string(),
            installed_version: installed_version.map(|s| s.to_string()),
            content_hash: content_hash.to_string(),
            detection_methods: vec![DetectionMethod::Filename, DetectionMethod::MetadataComment],
            confirmed: true,
            harness: ".claude".to_string(),
        }
    }

    // -----------------------------------------------------------------------
    // Unit tests — classify_drift
    // -----------------------------------------------------------------------

    #[test]
    fn classify_no_canonical_hash_returns_none() {
        let asset = make_asset("agents/aws/cdk", "abc123", Some("1.0.0"));
        assert_eq!(classify_drift(&asset, None, Some("1.0.0")), DriftKind::None);
    }

    #[test]
    fn classify_hash_match_returns_none() {
        let asset = make_asset("agents/aws/cdk", "abc123", Some("1.0.0"));
        assert_eq!(
            classify_drift(&asset, Some("abc123"), Some("1.0.0")),
            DriftKind::None
        );
    }

    #[test]
    fn classify_hash_mismatch_same_version_is_content_drift() {
        let asset = make_asset("agents/aws/cdk", "aaa", Some("1.0.0"));
        assert_eq!(
            classify_drift(&asset, Some("bbb"), Some("1.0.0")),
            DriftKind::ContentDrift
        );
    }

    #[test]
    fn classify_hash_mismatch_both_version_none_is_content_drift() {
        let asset = make_asset("agents/aws/cdk", "aaa", None);
        assert_eq!(
            classify_drift(&asset, Some("bbb"), None),
            DriftKind::ContentDrift
        );
    }

    #[test]
    fn classify_hash_mismatch_different_versions_is_version_drift() {
        let asset = make_asset("agents/aws/cdk", "aaa", Some("1.0.0"));
        assert_eq!(
            classify_drift(&asset, Some("bbb"), Some("2.0.0")),
            DriftKind::VersionDrift
        );
    }

    #[test]
    fn classify_hash_mismatch_installed_version_none_canonical_some_is_version_drift() {
        let asset = make_asset("agents/aws/cdk", "aaa", None);
        assert_eq!(
            classify_drift(&asset, Some("bbb"), Some("1.0.0")),
            DriftKind::VersionDrift
        );
    }

    #[test]
    fn classify_hash_mismatch_installed_some_canonical_none_is_version_drift() {
        let asset = make_asset("agents/aws/cdk", "aaa", Some("1.0.0"));
        assert_eq!(
            classify_drift(&asset, Some("bbb"), None),
            DriftKind::VersionDrift
        );
    }

    // -----------------------------------------------------------------------
    // Unit tests — detect_drift
    // -----------------------------------------------------------------------

    #[test]
    fn detect_drift_empty_returns_empty() {
        let records = detect_drift(&[], &HashMap::new(), &HashMap::new());
        assert!(records.is_empty());
    }

    #[test]
    fn detect_drift_no_mismatch_returns_empty() {
        let asset = make_asset("agents/aws/cdk", "abc123", Some("1.0.0"));
        let mut hashes = HashMap::new();
        hashes.insert("agents/aws/cdk".to_string(), "abc123".to_string());
        let mut versions = HashMap::new();
        versions.insert("agents/aws/cdk".to_string(), "1.0.0".to_string());

        let records = detect_drift(&[asset], &hashes, &versions);
        assert!(records.is_empty());
    }

    #[test]
    fn detect_drift_content_drift_detected() {
        let asset = make_asset("agents/aws/cdk", "actual_hash", Some("1.0.0"));
        let mut hashes = HashMap::new();
        hashes.insert("agents/aws/cdk".to_string(), "expected_hash".to_string());
        let mut versions = HashMap::new();
        versions.insert("agents/aws/cdk".to_string(), "1.0.0".to_string());

        let records = detect_drift(&[asset], &hashes, &versions);
        assert_eq!(records.len(), 1);
        assert_eq!(records[0].kind, DriftKind::ContentDrift);
        assert_eq!(records[0].expected_hash, "expected_hash");
        assert_eq!(records[0].actual_hash, "actual_hash");
    }

    #[test]
    fn detect_drift_version_drift_detected() {
        let asset = make_asset("agents/aws/cdk", "new_hash", Some("2.0.0"));
        let mut hashes = HashMap::new();
        hashes.insert("agents/aws/cdk".to_string(), "old_hash".to_string());
        let mut versions = HashMap::new();
        versions.insert("agents/aws/cdk".to_string(), "1.0.0".to_string());

        let records = detect_drift(&[asset], &hashes, &versions);
        assert_eq!(records.len(), 1);
        assert_eq!(records[0].kind, DriftKind::VersionDrift);
    }

    #[test]
    fn detect_drift_sorted_by_asset_id_then_path() {
        use crate::federation::scanner::DetectionMethod;

        let mut asset_b = make_asset("agents/b", "hash_b", Some("1.0.0"));
        let mut asset_a = make_asset("agents/a", "hash_a", Some("1.0.0"));
        // Make hashes differ from canonical so they generate drift records.
        asset_b.content_hash = "b_actual".to_string();
        asset_a.content_hash = "a_actual".to_string();

        let mut hashes = HashMap::new();
        hashes.insert("agents/b".to_string(), "b_expected".to_string());
        hashes.insert("agents/a".to_string(), "a_expected".to_string());
        let mut versions = HashMap::new();
        versions.insert("agents/b".to_string(), "1.0.0".to_string());
        versions.insert("agents/a".to_string(), "1.0.0".to_string());

        // Intentionally pass b before a to verify sort order.
        let records = detect_drift(&[asset_b, asset_a], &hashes, &versions);
        assert_eq!(records.len(), 2);
        assert_eq!(records[0].asset_id, "agents/a");
        assert_eq!(records[1].asset_id, "agents/b");

        // Suppress unused import warning
        let _ = DetectionMethod::Filename;
    }

    #[test]
    fn detect_drift_unconfirmed_assets_excluded() {
        use crate::federation::scanner::DetectionMethod;
        let mut asset = make_asset("agents/aws/cdk", "actual", Some("1.0.0"));
        asset.confirmed = false;
        asset.detection_methods = vec![DetectionMethod::Filename]; // only 1 method

        let mut hashes = HashMap::new();
        hashes.insert("agents/aws/cdk".to_string(), "expected".to_string());
        let mut versions = HashMap::new();
        versions.insert("agents/aws/cdk".to_string(), "1.0.0".to_string());

        // Unconfirmed — should be excluded even though hash differs.
        let records = detect_drift(&[asset], &hashes, &versions);
        assert!(
            records.is_empty(),
            "unconfirmed assets should not generate drift records"
        );
    }

    // -----------------------------------------------------------------------
    // Property 21 — Drift classification (Req 10.3, 10.4)
    //
    // For any combination of hash match/mismatch and version same/different,
    // classify_drift must return exactly the correct DriftKind.
    // -----------------------------------------------------------------------

    prop_compose! {
        /// Generate a plausible hex hash string (10 hex chars for brevity).
        fn arb_hash()(s in "[0-9a-f]{10}") -> String { s }
    }

    prop_compose! {
        fn arb_version_str()(
            major in 0u8..5,
            minor in 0u8..10,
            patch in 0u8..10,
        ) -> String {
            format!("{major}.{minor}.{patch}")
        }
    }

    proptest! {
        #![proptest_config(proptest::test_runner::Config {
            cases: 256,
            ..Default::default()
        })]

        /// Property 21a: classify_drift returns ContentDrift iff hash differs AND
        /// version is the same (both None or both same Some).
        #[test]
        fn prop21a_content_drift_iff_hash_differs_version_same(
            hash_a in arb_hash(),
            hash_b in arb_hash(),
            version in arb_version_str(),
        ) {
            let asset = make_asset("agents/test", &hash_a, Some(&version));
            let canonical_hash = Some(hash_b.as_str());
            let canonical_version = Some(version.as_str());

            let result = classify_drift(&asset, canonical_hash, canonical_version);

            if hash_a == hash_b {
                prop_assert_eq!(result, DriftKind::None, "hash match → None");
            } else {
                // hash differs, version same → ContentDrift
                prop_assert_eq!(result, DriftKind::ContentDrift,
                    "hash differs, version same → ContentDrift");
            }
        }

        /// Property 21b: classify_drift returns VersionDrift iff hash differs AND
        /// versions differ (one or both Some but not equal, or one None/one Some).
        #[test]
        fn prop21b_version_drift_iff_hash_differs_version_different(
            hash_a in arb_hash(),
            hash_b in arb_hash(),
            ver_a in arb_version_str(),
            ver_b in arb_version_str(),
        ) {
            // Only test when versions are actually different.
            prop_assume!(ver_a != ver_b);

            let asset = make_asset("agents/test", &hash_a, Some(&ver_a));
            let canonical_hash = Some(hash_b.as_str());
            let canonical_version = Some(ver_b.as_str());

            let result = classify_drift(&asset, canonical_hash, canonical_version);

            if hash_a == hash_b {
                prop_assert_eq!(result, DriftKind::None, "hash match → None");
            } else {
                prop_assert_eq!(result, DriftKind::VersionDrift,
                    "hash differs, versions different → VersionDrift");
            }
        }

        /// Property 21c: classify_drift returns None iff hash matches (canonical
        /// hash provided), regardless of versions.
        #[test]
        fn prop21c_none_iff_hash_matches(
            hash in arb_hash(),
            version_a in arb_version_str(),
            version_b in arb_version_str(),
        ) {
            let asset = make_asset("agents/test", &hash, Some(&version_a));
            // Same hash
            let result = classify_drift(&asset, Some(hash.as_str()), Some(&version_b));
            prop_assert_eq!(result, DriftKind::None,
                "hash matches → None regardless of versions");
        }

        /// Property 21d: when canonical_hash is None, always returns None.
        #[test]
        fn prop21d_no_canonical_hash_always_none(
            hash in arb_hash(),
            version in arb_version_str(),
        ) {
            let asset = make_asset("agents/test", &hash, Some(&version));
            let result = classify_drift(&asset, None, Some(&version));
            prop_assert_eq!(result, DriftKind::None,
                "no canonical hash → cannot detect drift → None");
        }

        /// Property 21e: both-None versions with hash mismatch → ContentDrift.
        #[test]
        fn prop21e_both_version_none_hash_mismatch_is_content_drift(
            hash_a in arb_hash(),
            hash_b in arb_hash(),
        ) {
            prop_assume!(hash_a != hash_b);
            let asset = make_asset("agents/test", &hash_a, None);
            let result = classify_drift(&asset, Some(&hash_b), None);
            prop_assert_eq!(result, DriftKind::ContentDrift,
                "hash differs, both versions None → ContentDrift");
        }
    }
}
