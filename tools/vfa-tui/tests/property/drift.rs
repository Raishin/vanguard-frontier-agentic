//! Feature: rust-tui-v2, Property 21 — Drift classification.
//! Validates: Requirements 10.2, 10.3, 27.2
//!
//! `classify_drift` distinguishes None / ContentDrift / VersionDrift by the
//! documented hash-then-version rules, and `detect_drift` returns only
//! confirmed, genuinely-drifted assets in deterministic order.

use std::collections::HashMap;
use std::path::PathBuf;

use proptest::prelude::*;
use proptest::test_runner::Config;

use vfa_tui::federation::drift::{classify_drift, detect_drift, DriftKind};
use vfa_tui::federation::scanner::InstalledAsset;

fn installed(id: &str, ws: &str, hash: &str, version: Option<&str>, confirmed: bool) -> InstalledAsset {
    InstalledAsset {
        workspace_path: PathBuf::from(ws),
        asset_id: id.to_string(),
        installed_version: version.map(|s| s.to_string()),
        content_hash: hash.to_string(),
        detection_methods: vec![],
        confirmed,
        harness: "claude".to_string(),
    }
}

proptest! {
    #![proptest_config(Config::with_cases(256))]

    /// No canonical hash ⇒ no baseline ⇒ None.
    #[test]
    fn no_canonical_hash_is_none(hash in "[a-f0-9]{8}", v in proptest::option::of("[0-9.]{1,5}")) {
        let a = installed("agents/x", "ws", &hash, v.as_deref(), true);
        prop_assert_eq!(classify_drift(&a, None, v.as_deref()), DriftKind::None);
    }

    /// Equal hashes ⇒ None regardless of versions.
    #[test]
    fn equal_hash_is_none(hash in "[a-f0-9]{8}",
        iv in proptest::option::of("[0-9.]{1,5}"),
        cv in proptest::option::of("[0-9.]{1,5}")) {
        let a = installed("agents/x", "ws", &hash, iv.as_deref(), true);
        prop_assert_eq!(classify_drift(&a, Some(&hash), cv.as_deref()), DriftKind::None);
    }

    /// Differing hashes ⇒ ContentDrift iff versions match (both none or equal),
    /// else VersionDrift.
    #[test]
    fn hash_differs_splits_content_vs_version(
        iv in proptest::option::of("[0-9]{1,3}"),
        cv in proptest::option::of("[0-9]{1,3}"),
    ) {
        let a = installed("agents/x", "ws", "AAAA", iv.as_deref(), true);
        let kind = classify_drift(&a, Some("BBBB"), cv.as_deref());
        let versions_same = match (iv.as_deref(), cv.as_deref()) {
            (None, None) => true,
            (Some(x), Some(y)) => x == y,
            _ => false,
        };
        if versions_same {
            prop_assert_eq!(kind, DriftKind::ContentDrift);
        } else {
            prop_assert_eq!(kind, DriftKind::VersionDrift);
        }
    }

    /// `detect_drift` only reports confirmed assets that actually drifted, in
    /// order sorted by (asset_id, workspace_path).
    #[test]
    fn detect_drift_filters_and_sorts(
        n in 1usize..12,
        seed in any::<u64>(),
    ) {
        // Deterministically derive a mix of confirmed/drifted assets from seed.
        let mut assets = Vec::new();
        let mut hashes = HashMap::new();
        for i in 0..n {
            let id = format!("agents/a{}", (seed.wrapping_add(i as u64)) % 5);
            let ws = format!("ws{}", i % 3);
            let confirmed = (seed >> i) & 1 == 1;
            let drifted = (seed >> (i + 8)) & 1 == 1;
            let canon = "CANON";
            hashes.insert(id.clone(), canon.to_string());
            let hash = if drifted { "OTHER" } else { canon };
            assets.push(installed(&id, &ws, hash, None, confirmed));
        }
        let records = detect_drift(&assets, &hashes, &HashMap::new());

        // Every record corresponds to a confirmed, hash-differing asset.
        for r in &records {
            prop_assert_ne!(r.kind.clone(), DriftKind::None);
            let matched = assets.iter().any(|a|
                a.confirmed && a.asset_id == r.asset_id
                && a.workspace_path == r.workspace_path
                && a.content_hash != hashes[&a.asset_id]);
            prop_assert!(matched, "record without a matching confirmed drift: {:?}", r);
        }

        // Deterministic ordering.
        let mut sorted = records.clone();
        sorted.sort_by(|a, b| (a.asset_id.as_str(), &a.workspace_path)
            .cmp(&(b.asset_id.as_str(), &b.workspace_path)));
        prop_assert_eq!(records, sorted);
    }
}
