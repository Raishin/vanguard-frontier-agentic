//! Feature: rust-tui-v2, Properties 12 & 13 — Coverage matrix classification
//! and percentage scoring.
//! Validates: Requirements 3.5, 8.3
//!
//! P12: every cell is classified Installed / Outdated / Drifted / NotInstalled
//!      per the documented hash-then-version rules; unconfirmed installs are
//!      never counted.
//! P13: `compute_coverage_score` stays in [0, 100], is `None` only for an empty
//!      applicable set, and is monotonic in the installed count.

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use proptest::prelude::*;
use proptest::test_runner::Config;

use vfa_tui::federation::coverage::CoverageEngine;
use vfa_tui::federation::scanner::InstalledAsset;
use vfa_tui::models::coverage::CellStatus;

fn asset(id: &str, hash: &str, version: Option<&str>, ws: &Path) -> InstalledAsset {
    InstalledAsset {
        workspace_path: ws.to_path_buf(),
        asset_id: id.to_string(),
        installed_version: version.map(|s| s.to_string()),
        content_hash: hash.to_string(),
        detection_methods: vec![],
        confirmed: true,
        harness: "claude".to_string(),
    }
}

/// Build a single-asset, single-workspace matrix and return the one cell's status.
fn classify(
    installed_hash: &str,
    installed_version: Option<&str>,
    canonical_hash: Option<&str>,
    canonical_version: Option<&str>,
) -> CellStatus {
    let ws = PathBuf::from("ws-a");
    let id = "agents/x".to_string();
    let a = asset(&id, installed_hash, installed_version, &ws);
    let mut hashes = HashMap::new();
    if let Some(h) = canonical_hash {
        hashes.insert(id.clone(), h.to_string());
    }
    let mut versions = HashMap::new();
    if let Some(v) = canonical_version {
        versions.insert(id.clone(), v.to_string());
    }
    let matrix = CoverageEngine::build_matrix(
        std::slice::from_ref(&id),
        &[(ws, vec![a])],
        &hashes,
        &versions,
    );
    matrix
        .cells
        .get(&(id, "ws-a".to_string()))
        .expect("cell must exist")
        .status
        .clone()
}

proptest! {
    #![proptest_config(Config::with_cases(256))]

    /// P12 — matching content hash always classifies as Installed regardless of
    /// version.
    #[test]
    fn matching_hash_is_installed(
        hash in "[a-f0-9]{16}",
        iv in proptest::option::of("[0-9]{1,2}\\.[0-9]{1,2}\\.[0-9]{1,2}"),
        cv in proptest::option::of("[0-9]{1,2}\\.[0-9]{1,2}\\.[0-9]{1,2}"),
    ) {
        let status = classify(&hash, iv.as_deref(), Some(&hash), cv.as_deref());
        prop_assert_eq!(status, CellStatus::Installed);
    }

    /// P12 — hash mismatch with a strictly-behind semver version is Outdated;
    /// otherwise (same/ahead/unknown version) it is Drifted.
    #[test]
    fn hash_mismatch_outdated_vs_drifted(
        i_major in 0u32..5, i_minor in 0u32..20,
        c_major in 0u32..5, c_minor in 0u32..20,
    ) {
        let iv = format!("{i_major}.{i_minor}.0");
        let cv = format!("{c_major}.{c_minor}.0");
        let status = classify("aaaa", Some(&iv), Some("bbbb"), Some(&cv));
        let behind = (i_major, i_minor) < (c_major, c_minor);
        if behind {
            prop_assert_eq!(status, CellStatus::Outdated);
        } else {
            prop_assert_eq!(status, CellStatus::Drifted);
        }
    }

    /// P12 — an asset that is not installed in the workspace is NotInstalled.
    #[test]
    fn absent_asset_is_not_installed(hash in "[a-f0-9]{8}") {
        let ws = PathBuf::from("ws-a");
        let installed = "agents/installed".to_string();
        let missing = "agents/missing".to_string();
        let a = asset(&installed, &hash, None, &ws);
        let mut hashes = HashMap::new();
        hashes.insert(installed.clone(), hash.clone());
        hashes.insert(missing.clone(), hash.clone());
        let matrix = CoverageEngine::build_matrix(
            &[installed, missing.clone()],
            &[(ws, vec![a])],
            &hashes,
            &HashMap::new(),
        );
        let cell = matrix.cells.get(&(missing, "ws-a".to_string())).unwrap();
        prop_assert_eq!(cell.status.clone(), CellStatus::NotInstalled);
    }

    /// P12 — unconfirmed installs are excluded, so the cell is NotInstalled even
    /// though the asset object exists for that workspace.
    #[test]
    fn unconfirmed_install_is_not_counted(hash in "[a-f0-9]{8}") {
        let ws = PathBuf::from("ws-a");
        let id = "agents/x".to_string();
        let mut a = asset(&id, &hash, None, &ws);
        a.confirmed = false;
        let mut hashes = HashMap::new();
        hashes.insert(id.clone(), hash.clone());
        let matrix = CoverageEngine::build_matrix(
            std::slice::from_ref(&id),
            &[(ws, vec![a])],
            &hashes,
            &HashMap::new(),
        );
        let cell = matrix.cells.get(&(id, "ws-a".to_string())).unwrap();
        prop_assert_eq!(cell.status.clone(), CellStatus::NotInstalled);
        // ... and it must not earn coverage credit.
        prop_assert_eq!(matrix.workspace_scores.get("ws-a"), Some(&0.0));
    }

    /// P13 — coverage score is in [0, 100], None only for an empty applicable
    /// set, and never decreases as installed count grows.
    #[test]
    fn coverage_score_in_range_none_only_when_empty(
        total in 0usize..1000, installed in 0usize..1000,
    ) {
        let installed = installed.min(total);
        let score = CoverageEngine::compute_coverage_score(installed, total);
        match score {
            None => prop_assert_eq!(total, 0),
            Some(s) => {
                prop_assert!(total > 0);
                prop_assert!((0.0..=100.0).contains(&s), "score {s} out of range");
            }
        }
        if total > 0 && installed < total {
            let next = CoverageEngine::compute_coverage_score(installed + 1, total).unwrap();
            prop_assert!(next >= score.unwrap());
        }
    }

    /// P13 — a fully-installed workspace scores exactly 100.0.
    #[test]
    fn full_install_scores_100(n in 1usize..20) {
        let ws = PathBuf::from("ws-a");
        let ids: Vec<String> = (0..n).map(|i| format!("agents/a{i}")).collect();
        let mut hashes = HashMap::new();
        let mut assets = Vec::new();
        for id in &ids {
            hashes.insert(id.clone(), "h".to_string());
            assets.push(asset(id, "h", None, &ws));
        }
        let matrix = CoverageEngine::build_matrix(&ids, &[(ws, assets)], &hashes, &HashMap::new());
        prop_assert_eq!(matrix.workspace_scores.get("ws-a"), Some(&100.0));
    }
}
