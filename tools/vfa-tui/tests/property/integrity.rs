//! Feature: rust-tui-v2, Property 14 — SHA-256 asset integrity verification.
//! Validates: Requirements 16.x (asset integrity)
//!
//! For any set of files on disk, `verify_integrity` must classify each entry as
//! Pass (file exists and its SHA-256 equals the manifest hash), Fail (file
//! exists but its SHA-256 differs), or Missing (file does not exist), and the
//! sync and parallel implementations must agree.

use proptest::prelude::*;
use proptest::test_runner::Config;
use sha2::{Digest, Sha256};

use vfa_tui::federation::integrity::{
    verify_integrity, verify_integrity_parallel, IntegrityStatus,
};
use vfa_tui::models::integrity::{AssetIntegrity, IntegrityFile, IntegrityScope, IntegrityTree};

fn sha256_hex(bytes: &[u8]) -> String {
    use std::fmt::Write as _;
    let mut h = Sha256::new();
    h.update(bytes);
    let result = h.finalize();
    result.iter().fold(String::new(), |mut s, b| {
        let _ = write!(s, "{b:02x}");
        s
    })
}

/// Build a single-tree manifest from `(relative_path, expected_hash)` entries.
fn manifest_with(files: Vec<(String, String)>) -> AssetIntegrity {
    let entries: Vec<IntegrityFile> = files
        .into_iter()
        .map(|(path, sha256)| IntegrityFile {
            path,
            sha256,
            bytes: 0,
        })
        .collect();
    AssetIntegrity {
        manifest_version: 1,
        algorithm: "sha256".to_string(),
        scope: IntegrityScope {
            trees: vec!["tree".to_string()],
            root_files: vec![],
        },
        trees: vec![IntegrityTree {
            tree: "tree".to_string(),
            aggregate_sha256: String::new(),
            files: entries,
        }],
        root_files: vec![],
        aggregate_sha256: String::new(),
    }
}

proptest! {
    #![proptest_config(Config::with_cases(96))]

    /// A file written with content whose hash matches the manifest is Pass;
    /// flipping a byte of the expected hash makes it Fail; an absent file is
    /// Missing.
    #[test]
    fn classifies_pass_fail_missing(
        present in proptest::collection::vec(
            (proptest::string::string_regex("[a-z0-9_]{1,12}").unwrap(),
             proptest::collection::vec(any::<u8>(), 0..64)),
            1..6),
        absent_names in proptest::collection::vec(
            proptest::string::string_regex("[a-z0-9_]{1,12}").unwrap(), 0..3),
        corrupt_first in any::<bool>(),
    ) {
        let dir = tempfile::TempDir::new().unwrap();

        // Deduplicate names so each path maps to one file.
        let mut seen = std::collections::HashSet::new();
        let present: Vec<_> = present.into_iter()
            .filter(|(n, _)| seen.insert(n.clone()))
            .collect();
        prop_assume!(!present.is_empty());

        let mut manifest_files = Vec::new();
        for (i, (name, content)) in present.iter().enumerate() {
            let path = format!("{name}.bin");
            std::fs::write(dir.path().join(&path), content).unwrap();
            let mut expected = sha256_hex(content);
            // Optionally corrupt the first entry's expected hash to force Fail.
            let corrupt = corrupt_first && i == 0;
            if corrupt {
                expected = sha256_hex(b"definitely-different-content");
            }
            manifest_files.push((path, expected, corrupt));
        }
        for name in &absent_names {
            // Use a distinct extension so it can never collide with a written file.
            manifest_files.push((format!("{name}.absent"), sha256_hex(b"x"), false));
        }

        let manifest = manifest_with(
            manifest_files.iter().map(|(p, h, _)| (p.clone(), h.clone())).collect(),
        );
        let results = verify_integrity(&manifest, dir.path());
        prop_assert_eq!(results.len(), manifest_files.len());

        for r in &results {
            let (_, _, corrupt) = manifest_files.iter().find(|(p, _, _)| *p == r.path).unwrap();
            let expected_status = if r.path.ends_with(".absent") {
                IntegrityStatus::Missing
            } else if *corrupt {
                IntegrityStatus::Fail
            } else {
                IntegrityStatus::Pass
            };
            prop_assert_eq!(&r.status, &expected_status, "path {} misclassified", r.path);
        }

        // Results are returned in deterministic sorted-by-path order.
        let mut sorted = results.clone();
        sorted.sort_by(|a, b| a.path.cmp(&b.path));
        let order: Vec<&String> = results.iter().map(|r| &r.path).collect();
        let sorted_order: Vec<&String> = sorted.iter().map(|r| &r.path).collect();
        prop_assert_eq!(order, sorted_order);

        // Sync and parallel implementations agree on status per path.
        let rt = tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap();
        let par = rt.block_on(verify_integrity_parallel(&manifest, dir.path()));
        for r in &results {
            let p = par.iter().find(|x| x.path == r.path).unwrap();
            prop_assert_eq!(&p.status, &r.status, "sync/parallel disagree for {}", r.path);
        }
    }
}
