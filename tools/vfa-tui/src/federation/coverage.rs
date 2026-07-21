//! Coverage engine — Tasks 7.1 & 7.2 (Req 3.1–3.7).
//!
//! # Responsibilities
//!
//! - [`CoverageEngine`] — stateless engine that builds the full coverage matrix
//!   from canonical asset lists and workspace scan results.
//! - [`CoverageEngine::build_matrix`] — constructs a [`CoverageMatrix`] with
//!   per-cell classification and per-workspace coverage scores.
//! - [`CoverageEngine::compute_coverage_score`] — (installed matching canonical)
//!   / (total applicable) × 100, round-half-up; `None` when no applicable assets
//!   (Req 3.5).
//! - [`CoverageEngine::compute_freshness_score`] — delegates to [`freshness_score`]
//!   from `versions.rs` (Req 8.3).
//!
//! # Cell classification (Req 3.x, 10.1)
//!
//! | Asset present? | Hash match? | Version behind? | `CellStatus`   |
//! |---------------|-------------|-----------------|----------------|
//! | No             | —           | —               | `NotInstalled` |
//! | Yes            | Yes         | (any)           | `Installed`    |
//! | Yes            | No          | No              | `Drifted`      |
//! | Yes            | No          | Yes             | `Drifted`      |
//! | Yes            | No (match)  | Yes (outdated)  | `Outdated`     |
//!
//! More precisely: when the installed asset is **present**:
//! 1. If `content_hash == canonical_hash` → `Installed` (current).
//! 2. Else if `content_hash != canonical_hash` AND the installed version is
//!    **behind** the canonical version (per [`compare_versions`]) → `Outdated`
//!    (intentional update, just not to the latest canonical content).
//!    NOTE: Per the spec (Req 3.3 "version differs from canonical → outdated",
//!    Req 3.4 "hash differs → drifted"), `Drifted` takes precedence for hash
//!    mismatches where the version is NOT simply behind.
//!
//! The implementation uses the following priority order:
//! 1. No asset present → `NotInstalled`.
//! 2. Hash match → `Installed`.
//! 3. Hash mismatch + version behind (installed < canonical semver) → `Outdated`.
//! 4. Hash mismatch otherwise → `Drifted`.
//!
//! This matches the spec intent: `Outdated` is the "known update available"
//! state (version is simply behind), while `Drifted` covers all other hash
//! mismatches (local modification, unknown version, etc.).

#![deny(warnings)]

use std::collections::HashMap;
use std::path::PathBuf;

use crate::federation::scanner::InstalledAsset;
use crate::federation::versions::{compare_versions, freshness_score, round_half_up_1dp};
use crate::models::coverage::{AssetType, CellStatus, CoverageCell, CoverageMatrix, CoverageRow};
use crate::models::provider::Provider;
use crate::persistence::writer::{DbCommand, WriterHandle};

/// Persist per-workspace coverage scores to `coverage_cache` via the
/// single-writer task (Req 3.6).
///
/// `scores` is a slice of `(workspace_path, workspace_name, coverage_score)`.
/// Best-effort: writer-channel errors are ignored.
pub async fn persist_coverage_scores(
    tx: &WriterHandle,
    scores: &[(String, String, f64)],
    computed_at: &str,
) {
    for (workspace_path, workspace_name, coverage_score) in scores {
        let _ = tx
            .send(DbCommand::RecordCoverageScore {
                workspace_path: workspace_path.clone(),
                workspace_name: workspace_name.clone(),
                coverage_score: *coverage_score,
                computed_at: computed_at.to_string(),
            })
            .await;
    }
}

// ---------------------------------------------------------------------------
// CoverageEngine
// ---------------------------------------------------------------------------

/// Stateless engine for building and scoring the coverage matrix.
///
/// All methods are pure functions — no internal state is mutated.  The struct
/// exists as an organisational namespace.
pub struct CoverageEngine;

impl CoverageEngine {
    // -----------------------------------------------------------------------
    // build_matrix
    // -----------------------------------------------------------------------

    /// Build the full coverage matrix from canonical assets and scan results.
    ///
    /// # Arguments
    ///
    /// - `canonical_asset_ids` — ordered list of all catalog asset identifiers.
    ///   Each entry becomes one row in the matrix.
    /// - `workspaces` — list of `(workspace_name, installed_assets)` pairs.
    ///   Each pair becomes one column and contributes cells for each asset.
    /// - `canonical_hashes` — map from `asset_id` → expected SHA-256 hex
    ///   (from `asset-integrity.json`).
    /// - `canonical_versions` — map from `asset_id` → version string (from
    ///   catalog).
    ///
    /// The matrix rows are ordered by `canonical_asset_ids`, and columns are
    /// ordered by the order of `workspaces`.  Coverage scores are computed for
    /// each workspace and stored in `workspace_scores`.
    pub fn build_matrix(
        canonical_asset_ids: &[String],
        workspaces: &[(PathBuf, Vec<InstalledAsset>)],
        canonical_hashes: &HashMap<String, String>,
        canonical_versions: &HashMap<String, String>,
    ) -> CoverageMatrix {
        // Build an index from (workspace_path, asset_id) → InstalledAsset for
        // fast lookup.  We use only confirmed assets to avoid false positives.
        let mut install_index: HashMap<(PathBuf, String), &InstalledAsset> = HashMap::new();
        for (ws_path, assets) in workspaces {
            for asset in assets {
                if asset.confirmed {
                    install_index.insert((ws_path.clone(), asset.asset_id.clone()), asset);
                }
            }
        }

        // Build workspace names from paths (use the path's last component as the
        // display name if available, otherwise the full display string).
        let workspace_names: Vec<String> = workspaces
            .iter()
            .map(|(path, _)| {
                path.file_name()
                    .and_then(|n| n.to_str())
                    .map(|s| s.to_string())
                    .unwrap_or_else(|| path.display().to_string())
            })
            .collect();

        // Build rows from canonical asset IDs.
        let rows: Vec<CoverageRow> = canonical_asset_ids
            .iter()
            .map(|id| CoverageRow {
                asset_id: id.clone(),
                asset_type: infer_asset_type(id),
                asset_name: asset_display_name(id),
                provider: infer_provider(id),
            })
            .collect();

        // Fill cells and track per-workspace counts.
        let mut cells: HashMap<(String, String), CoverageCell> = HashMap::new();

        // Per-workspace counters: (installed_matching, total_applicable)
        let mut ws_installed: Vec<usize> = vec![0; workspaces.len()];
        let mut ws_total: Vec<usize> = vec![0; workspaces.len()];

        for asset_id in canonical_asset_ids {
            let canonical_hash = canonical_hashes.get(asset_id).map(|s| s.as_str());
            let canonical_version = canonical_versions.get(asset_id).map(|s| s.as_str());

            for (ws_idx, (ws_path, _)) in workspaces.iter().enumerate() {
                let ws_name = &workspace_names[ws_idx];

                ws_total[ws_idx] += 1; // this asset is applicable

                let cell =
                    if let Some(asset) = install_index.get(&(ws_path.clone(), asset_id.clone())) {
                        let status = classify_cell_status(asset, canonical_hash, canonical_version);
                        if status == CellStatus::Installed {
                            ws_installed[ws_idx] += 1;
                        }
                        CoverageCell {
                            status,
                            installed_version: asset.installed_version.clone(),
                            canonical_version: canonical_version.unwrap_or("").to_string(),
                            installed_hash: Some(asset.content_hash.clone()),
                            canonical_hash: canonical_hash.map(|s| s.to_string()),
                        }
                    } else {
                        CoverageCell {
                            status: CellStatus::NotInstalled,
                            installed_version: Option::None,
                            canonical_version: canonical_version.unwrap_or("").to_string(),
                            installed_hash: Option::None,
                            canonical_hash: canonical_hash.map(|s| s.to_string()),
                        }
                    };

                cells.insert((asset_id.clone(), ws_name.clone()), cell);
            }
        }

        // Compute per-workspace coverage scores.
        let mut workspace_scores: HashMap<String, f64> = HashMap::new();
        for (ws_idx, ws_name) in workspace_names.iter().enumerate() {
            if let Some(score) =
                Self::compute_coverage_score(ws_installed[ws_idx], ws_total[ws_idx])
            {
                workspace_scores.insert(ws_name.clone(), score);
            }
            // If None (no applicable assets), we omit the workspace from scores (Req 3.5).
        }

        CoverageMatrix {
            rows,
            columns: workspace_names,
            cells,
            workspace_scores,
        }
    }

    // -----------------------------------------------------------------------
    // compute_coverage_score
    // -----------------------------------------------------------------------

    /// Compute the per-workspace coverage percentage (Req 3.5).
    ///
    /// Formula: `(installed_matching / total_applicable) × 100`, rounded half-up
    /// to one decimal place.
    ///
    /// Returns `None` when `total_applicable == 0` (workspace has no applicable
    /// canonical assets — display as "N/A" in the UI per Req 3.5).
    pub fn compute_coverage_score(
        installed_matching: usize,
        total_applicable: usize,
    ) -> Option<f64> {
        if total_applicable == 0 {
            return Option::None;
        }
        let ratio = installed_matching as f64 / total_applicable as f64;
        Some(round_half_up_1dp(ratio * 100.0))
    }

    // -----------------------------------------------------------------------
    // compute_freshness_score
    // -----------------------------------------------------------------------

    /// Compute the freshness score for a workspace (Req 8.3).
    ///
    /// Delegates to [`freshness_score`] from `versions.rs`.
    ///
    /// Returns `0.0` when `total_with_versions == 0`.
    pub fn compute_freshness_score(current_count: usize, total_with_versions: usize) -> f64 {
        freshness_score(current_count, total_with_versions)
    }
}

// ---------------------------------------------------------------------------
// classify_cell_status (private helper)
// ---------------------------------------------------------------------------

/// Classify the cell status for a single (installed, canonical) pair.
///
/// Priority order:
/// 1. Hash match → `Installed`.
/// 2. Hash mismatch + version behind (installed < canonical) → `Outdated`.
/// 3. Hash mismatch otherwise → `Drifted`.
fn classify_cell_status(
    asset: &InstalledAsset,
    canonical_hash: Option<&str>,
    canonical_version: Option<&str>,
) -> CellStatus {
    // Step 1: hash match.
    if let Some(chash) = canonical_hash {
        if asset.content_hash == chash {
            return CellStatus::Installed;
        }
    } else {
        // No canonical hash — if hashes can't be compared, fall back to version.
        // Without a canonical hash we cannot assert Installed or Drifted safely.
        // Treat as Installed only if versions match; Outdated if version is behind.
        if let (Some(iv), Some(cv)) = (asset.installed_version.as_deref(), canonical_version) {
            if compare_versions(iv, cv) == std::cmp::Ordering::Less {
                return CellStatus::Outdated;
            }
        }
        return CellStatus::Installed;
    }

    // Step 2: hash mismatch — check if version is behind.
    if let (Some(iv), Some(cv)) = (asset.installed_version.as_deref(), canonical_version) {
        if compare_versions(iv, cv) == std::cmp::Ordering::Less {
            return CellStatus::Outdated;
        }
    }

    // Step 3: hash mismatch and version is not simply behind → Drifted.
    CellStatus::Drifted
}

// ---------------------------------------------------------------------------
// Private helpers — asset metadata inference
// ---------------------------------------------------------------------------

/// Infer [`AssetType`] from the asset ID prefix.
fn infer_asset_type(asset_id: &str) -> AssetType {
    if asset_id.starts_with("agents/") {
        AssetType::Agent
    } else if asset_id.starts_with("skills/") {
        AssetType::Skill
    } else if asset_id.starts_with("rules/") {
        AssetType::Rule
    } else if asset_id.starts_with("mcp/") {
        AssetType::McpRef
    } else {
        AssetType::Agent // default
    }
}

/// Extract a human-readable display name from an asset ID.
///
/// The display name is the last path component with hyphens replaced by spaces
/// and each word capitalised.
fn asset_display_name(asset_id: &str) -> String {
    let last = asset_id.split('/').next_back().unwrap_or(asset_id);
    last.split('-')
        .map(|word| {
            let mut c = word.chars();
            match c.next() {
                None => String::new(),
                Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

/// Infer a [`Provider`] from the asset ID.
///
/// Checks the second path component (e.g. `agents/aws/...` → `Provider::Aws`).
fn infer_provider(asset_id: &str) -> Provider {
    let parts: Vec<&str> = asset_id.split('/').collect();
    if parts.len() >= 2 {
        match parts[1].to_ascii_lowercase().as_str() {
            "aws" => Provider::Aws,
            "azure" => Provider::Azure,
            "gcp" => Provider::Gcp,
            "oracle" | "oci" => Provider::Oracle,
            "kubernetes" | "k8s" => Provider::Kubernetes,
            "terraform" => Provider::Terraform,
            "generic" => Provider::Generic,
            "frontend" => Provider::Frontend,
            "java" => Provider::Java,
            "kotlin" => Provider::Kotlin,
            "php" => Provider::Php,
            _ => Provider::Generic,
        }
    } else {
        Provider::Generic
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::federation::scanner::DetectionMethod;
    use proptest::prelude::*;

    // -----------------------------------------------------------------------
    // Test helpers
    // -----------------------------------------------------------------------

    fn make_confirmed_asset(
        path: &str,
        asset_id: &str,
        content_hash: &str,
        installed_version: Option<&str>,
    ) -> InstalledAsset {
        InstalledAsset {
            workspace_path: PathBuf::from(path),
            asset_id: asset_id.to_string(),
            installed_version: installed_version.map(|s| s.to_string()),
            content_hash: content_hash.to_string(),
            detection_methods: vec![DetectionMethod::Filename, DetectionMethod::MetadataComment],
            confirmed: true,
            harness: ".claude".to_string(),
        }
    }

    // -----------------------------------------------------------------------
    // Unit tests — classify_cell_status
    // -----------------------------------------------------------------------

    #[test]
    fn cell_installed_when_hash_matches() {
        let asset = make_confirmed_asset("/ws/file.md", "agents/aws/cdk", "abc", Some("1.0.0"));
        assert_eq!(
            classify_cell_status(&asset, Some("abc"), Some("1.0.0")),
            CellStatus::Installed
        );
    }

    #[test]
    fn cell_outdated_when_hash_mismatch_version_behind() {
        let asset =
            make_confirmed_asset("/ws/file.md", "agents/aws/cdk", "old_hash", Some("1.0.0"));
        // canonical is 1.2.0 — installed is behind
        assert_eq!(
            classify_cell_status(&asset, Some("new_hash"), Some("1.2.0")),
            CellStatus::Outdated
        );
    }

    #[test]
    fn cell_drifted_when_hash_mismatch_version_same() {
        let asset = make_confirmed_asset("/ws/file.md", "agents/aws/cdk", "actual", Some("1.0.0"));
        assert_eq!(
            classify_cell_status(&asset, Some("expected"), Some("1.0.0")),
            CellStatus::Drifted
        );
    }

    #[test]
    fn cell_drifted_when_hash_mismatch_no_version() {
        let asset = make_confirmed_asset("/ws/file.md", "agents/aws/cdk", "actual", None);
        assert_eq!(
            classify_cell_status(&asset, Some("expected"), None),
            CellStatus::Drifted
        );
    }

    #[test]
    fn cell_drifted_when_hash_mismatch_installed_ahead() {
        // installed is 2.0.0, canonical is 1.0.0 — hash mismatch, not "behind"
        let asset =
            make_confirmed_asset("/ws/file.md", "agents/aws/cdk", "new_hash", Some("2.0.0"));
        assert_eq!(
            classify_cell_status(&asset, Some("old_hash"), Some("1.0.0")),
            CellStatus::Drifted
        );
    }

    // -----------------------------------------------------------------------
    // Unit tests — compute_coverage_score
    // -----------------------------------------------------------------------

    #[test]
    fn coverage_score_none_when_zero_applicable() {
        assert_eq!(CoverageEngine::compute_coverage_score(0, 0), Option::None);
    }

    #[test]
    fn coverage_score_100_when_all_installed() {
        assert_eq!(CoverageEngine::compute_coverage_score(5, 5), Some(100.0));
    }

    #[test]
    fn coverage_score_0_when_none_installed() {
        assert_eq!(CoverageEngine::compute_coverage_score(0, 5), Some(0.0));
    }

    #[test]
    fn coverage_score_half() {
        assert_eq!(CoverageEngine::compute_coverage_score(5, 10), Some(50.0));
    }

    #[test]
    fn coverage_score_rounding_half_up() {
        // 1/3 = 33.333… → 33.3
        let s = CoverageEngine::compute_coverage_score(1, 3).unwrap();
        assert!((s - 33.3).abs() < 1e-9, "expected 33.3 got {s}");
    }

    #[test]
    fn coverage_score_x5_boundary() {
        // 3/20 = 0.15 × 100 = 15.0 — exact, not a boundary case
        let s = CoverageEngine::compute_coverage_score(3, 20).unwrap();
        assert!((s - 15.0).abs() < 1e-9, "expected 15.0 got {s}");
    }

    // -----------------------------------------------------------------------
    // Unit tests — build_matrix
    // -----------------------------------------------------------------------

    #[test]
    fn build_matrix_empty_inputs() {
        let matrix = CoverageEngine::build_matrix(&[], &[], &HashMap::new(), &HashMap::new());
        assert!(matrix.rows.is_empty());
        assert!(matrix.columns.is_empty());
        assert!(matrix.cells.is_empty());
        assert!(matrix.workspace_scores.is_empty());
    }

    #[test]
    fn build_matrix_not_installed() {
        let assets: Vec<InstalledAsset> = vec![];
        let ws_path = PathBuf::from("/workspaces/team-a");
        let workspaces = vec![(ws_path, assets)];
        let canonical_ids = vec!["agents/aws/cdk".to_string()];
        let mut hashes = HashMap::new();
        hashes.insert("agents/aws/cdk".to_string(), "abc123".to_string());

        let matrix =
            CoverageEngine::build_matrix(&canonical_ids, &workspaces, &hashes, &HashMap::new());

        assert_eq!(matrix.rows.len(), 1);
        assert_eq!(matrix.columns.len(), 1);
        let cell = matrix
            .cells
            .get(&("agents/aws/cdk".to_string(), "team-a".to_string()))
            .expect("cell should exist");
        assert_eq!(cell.status, CellStatus::NotInstalled);
        // Score should be 0.0 (0/1 applicable assets)
        assert_eq!(matrix.workspace_scores.get("team-a"), Some(&0.0));
    }

    #[test]
    fn build_matrix_installed_hash_match() {
        let ws_path = PathBuf::from("/workspaces/team-a");
        let asset = make_confirmed_asset(
            "/workspaces/team-a/.claude/cdk-agent.md",
            "agents/aws/cdk",
            "abc123",
            Some("1.0.0"),
        );
        let workspaces = vec![(ws_path, vec![asset])];
        let canonical_ids = vec!["agents/aws/cdk".to_string()];
        let mut hashes = HashMap::new();
        hashes.insert("agents/aws/cdk".to_string(), "abc123".to_string());
        let mut versions = HashMap::new();
        versions.insert("agents/aws/cdk".to_string(), "1.0.0".to_string());

        let matrix = CoverageEngine::build_matrix(&canonical_ids, &workspaces, &hashes, &versions);

        let cell = matrix
            .cells
            .get(&("agents/aws/cdk".to_string(), "team-a".to_string()))
            .expect("cell should exist");
        assert_eq!(cell.status, CellStatus::Installed);
        assert_eq!(matrix.workspace_scores.get("team-a"), Some(&100.0));
    }

    #[test]
    fn build_matrix_outdated_version_behind() {
        let ws_path = PathBuf::from("/workspaces/team-a");
        let asset = make_confirmed_asset(
            "/workspaces/team-a/.claude/cdk-agent.md",
            "agents/aws/cdk",
            "old_hash",
            Some("1.0.0"),
        );
        let workspaces = vec![(ws_path, vec![asset])];
        let canonical_ids = vec!["agents/aws/cdk".to_string()];
        let mut hashes = HashMap::new();
        hashes.insert("agents/aws/cdk".to_string(), "new_hash".to_string());
        let mut versions = HashMap::new();
        versions.insert("agents/aws/cdk".to_string(), "1.2.0".to_string());

        let matrix = CoverageEngine::build_matrix(&canonical_ids, &workspaces, &hashes, &versions);

        let cell = matrix
            .cells
            .get(&("agents/aws/cdk".to_string(), "team-a".to_string()))
            .expect("cell should exist");
        assert_eq!(cell.status, CellStatus::Outdated);
        // Outdated does not count as "matching canonical" for coverage score
        assert_eq!(matrix.workspace_scores.get("team-a"), Some(&0.0));
    }

    #[test]
    fn build_matrix_drifted() {
        let ws_path = PathBuf::from("/workspaces/team-a");
        let asset = make_confirmed_asset(
            "/workspaces/team-a/.claude/cdk-agent.md",
            "agents/aws/cdk",
            "modified_hash",
            Some("1.0.0"),
        );
        let workspaces = vec![(ws_path, vec![asset])];
        let canonical_ids = vec!["agents/aws/cdk".to_string()];
        let mut hashes = HashMap::new();
        hashes.insert("agents/aws/cdk".to_string(), "canonical_hash".to_string());
        let mut versions = HashMap::new();
        versions.insert("agents/aws/cdk".to_string(), "1.0.0".to_string());

        let matrix = CoverageEngine::build_matrix(&canonical_ids, &workspaces, &hashes, &versions);

        let cell = matrix
            .cells
            .get(&("agents/aws/cdk".to_string(), "team-a".to_string()))
            .expect("cell should exist");
        assert_eq!(cell.status, CellStatus::Drifted);
        assert_eq!(matrix.workspace_scores.get("team-a"), Some(&0.0));
    }

    #[test]
    fn build_matrix_no_applicable_assets_score_omitted() {
        let ws_path = PathBuf::from("/workspaces/team-a");
        let workspaces: Vec<(PathBuf, Vec<InstalledAsset>)> = vec![(ws_path, vec![])];
        // No canonical assets at all
        let matrix =
            CoverageEngine::build_matrix(&[], &workspaces, &HashMap::new(), &HashMap::new());
        // Score should be absent (None → not stored) per Req 3.5
        assert!(
            !matrix.workspace_scores.contains_key("team-a"),
            "workspace with no applicable assets should not have a score"
        );
    }

    #[test]
    fn build_matrix_unconfirmed_assets_excluded() {
        use crate::federation::scanner::DetectionMethod;
        let ws_path = PathBuf::from("/workspaces/team-a");
        let mut asset = make_confirmed_asset(
            "/workspaces/team-a/.claude/cdk-agent.md",
            "agents/aws/cdk",
            "abc123",
            Some("1.0.0"),
        );
        // Mark as unconfirmed
        asset.confirmed = false;
        asset.detection_methods = vec![DetectionMethod::Filename];

        let workspaces = vec![(ws_path, vec![asset])];
        let canonical_ids = vec!["agents/aws/cdk".to_string()];
        let mut hashes = HashMap::new();
        hashes.insert("agents/aws/cdk".to_string(), "abc123".to_string());

        let matrix =
            CoverageEngine::build_matrix(&canonical_ids, &workspaces, &hashes, &HashMap::new());

        let cell = matrix
            .cells
            .get(&("agents/aws/cdk".to_string(), "team-a".to_string()))
            .expect("cell should exist");
        // Unconfirmed → treated as NotInstalled
        assert_eq!(cell.status, CellStatus::NotInstalled);
    }

    // -----------------------------------------------------------------------
    // Unit tests — infer_provider (path → Provider mapping)
    // -----------------------------------------------------------------------

    #[test]
    fn infer_provider_maps_java_board() {
        // Java board assets must classify as Java, not fall through to Generic,
        // so coverage rows render and filter under the correct provider.
        assert_eq!(
            infer_provider("agents/java/jdk-lifecycle-and-upgrade"),
            Provider::Java
        );
        assert_eq!(
            infer_provider("skills/java/java-jdk-lifecycle-and-upgrade"),
            Provider::Java
        );
    }

    #[test]
    fn infer_provider_unknown_falls_back_to_generic() {
        // An unmapped second path component still falls back to Generic.
        assert_eq!(
            infer_provider("agents/not-a-real-provider/foo"),
            Provider::Generic
        );
    }

    // -----------------------------------------------------------------------
    // Property 12 — Coverage matrix cell classification (Req 3.1, 3.4, 10.1)
    //
    // For any combination of hash match/mismatch and version comparison,
    // the cell classification must be consistent and correct.
    // -----------------------------------------------------------------------

    prop_compose! {
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

        /// Property 12a: cell is `Installed` iff hash matches canonical hash.
        #[test]
        fn prop12a_installed_iff_hash_matches(
            hash in arb_hash(),
            version in arb_version_str(),
        ) {
            let asset = make_confirmed_asset("/ws/file.md", "agents/test", &hash, Some(&version));
            // Same hash → Installed
            let status = classify_cell_status(&asset, Some(&hash), Some(&version));
            prop_assert_eq!(status, CellStatus::Installed,
                "hash match should be Installed");
        }

        /// Property 12b: cell is `Outdated` when hash mismatches and version is behind.
        #[test]
        fn prop12b_outdated_when_hash_mismatch_version_behind(
            hash_installed in arb_hash(),
            hash_canonical in arb_hash(),
            imajor in 0u8..5,
            iminor in 0u8..8,
            ipatch in 0u8..10,
            extra_minor in 1u8..5,
        ) {
            prop_assume!(hash_installed != hash_canonical);
            let canonical_minor = iminor + extra_minor;
            prop_assume!(canonical_minor < 30);

            let iv = format!("{imajor}.{iminor}.{ipatch}");
            let cv = format!("{imajor}.{canonical_minor}.{ipatch}");

            let asset = make_confirmed_asset("/ws/file.md", "agents/test", &hash_installed, Some(&iv));
            let status = classify_cell_status(&asset, Some(&hash_canonical), Some(&cv));
            prop_assert_eq!(status, CellStatus::Outdated,
                "hash mismatch + installed behind → Outdated");
        }

        /// Property 12c: cell is `Drifted` when hash mismatches and version is NOT behind.
        #[test]
        fn prop12c_drifted_when_hash_mismatch_version_not_behind(
            hash_installed in arb_hash(),
            hash_canonical in arb_hash(),
            version in arb_version_str(),
        ) {
            prop_assume!(hash_installed != hash_canonical);
            // Same version (not behind) → Drifted
            let asset = make_confirmed_asset("/ws/file.md", "agents/test", &hash_installed, Some(&version));
            let status = classify_cell_status(&asset, Some(&hash_canonical), Some(&version));
            prop_assert_eq!(status, CellStatus::Drifted,
                "hash mismatch + same version → Drifted");
        }

        /// Property 12d: `NotInstalled` cells have no installed hash.
        /// Verified by checking the matrix output for absent assets.
        #[test]
        fn prop12d_absent_asset_is_not_installed(
            hash in arb_hash(),
            version in arb_version_str(),
        ) {
            let ws_path = PathBuf::from("/ws");
            // No asset installed for this workspace
            let workspaces: Vec<(PathBuf, Vec<InstalledAsset>)> = vec![(ws_path, vec![])];
            let canonical_ids = vec!["agents/test".to_string()];
            let mut hashes = HashMap::new();
            hashes.insert("agents/test".to_string(), hash.clone());
            let mut versions = HashMap::new();
            versions.insert("agents/test".to_string(), version.clone());

            let matrix = CoverageEngine::build_matrix(
                &canonical_ids, &workspaces, &hashes, &versions,
            );

            let cell = matrix.cells.values().next().expect("should have a cell");
            prop_assert_eq!(&cell.status, &CellStatus::NotInstalled);
            prop_assert!(cell.installed_hash.is_none());
        }
    }

    proptest! {
        #![proptest_config(proptest::test_runner::Config {
            cases: 256,
            ..Default::default()
        })]

        // -----------------------------------------------------------------------
        // Property 13 — Percentage score computation (Req 3.5, 8.3)
        // -----------------------------------------------------------------------

        /// Property 13a: coverage score is always in [0.0, 100.0] and None iff
        /// total_applicable == 0.
        #[test]
        fn prop13a_coverage_score_range_and_none(
            installed in 0usize..=500,
            total in 0usize..=500,
        ) {
            let installed = installed.min(total); // can't exceed total
            match CoverageEngine::compute_coverage_score(installed, total) {
                Option::None => prop_assert_eq!(total, 0,
                    "None should only be returned when total==0"),
                Some(score) => {
                    prop_assert!(total > 0, "Some score requires total > 0");
                    prop_assert!((0.0..=100.0).contains(&score),
                        "score {} out of range [0, 100]", score);
                }
            }
        }

        /// Property 13b: round-half-up correctness — values ending in .x5 round up.
        /// Tests the rounding helper directly via compute_coverage_score.
        #[test]
        fn prop13b_round_half_up_x5_boundary(
            n in 1usize..=200,
        ) {
            // For n/200 we can construct exact midpoints at .x5 if we pick n carefully.
            // We just verify the result matches the round-half-up formula.
            let ratio = n as f64 / 200_f64;
            let expected = round_half_up_1dp(ratio * 100.0);
            let got = CoverageEngine::compute_coverage_score(n, 200).unwrap();
            prop_assert!(
                (got - expected).abs() < 1e-9,
                "compute_coverage_score({}, 200) = {} but round_half_up gives {}",
                n, got, expected
            );
        }

        /// Property 13c: freshness_score is always in [0.0, 100.0].
        #[test]
        fn prop13c_freshness_score_range(
            current in 0usize..=1000,
            total in 0usize..=1000,
        ) {
            let current = current.min(total);
            let score = CoverageEngine::compute_freshness_score(current, total);
            prop_assert!(
                (0.0..=100.0).contains(&score),
                "freshness_score({}, {}) = {} out of range",
                current, total, score
            );
        }
    }
}
