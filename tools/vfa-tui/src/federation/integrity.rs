//! Asset integrity verification engine (Tasks 7.16 / 7.17).
//!
//! Reads an [`AssetIntegrity`] manifest and verifies each listed file against
//! its recorded SHA-256 hash, reporting `Pass`, `Fail`, or `Missing` for
//! every entry.  Parallel verification (up to 8 concurrent I/O ops) is
//! provided via the async variant.

#![deny(warnings)]

use std::path::{Path, PathBuf};
use std::sync::Arc;

use sha2::{Digest, Sha256};
use tokio::sync::Semaphore;

use crate::models::integrity::AssetIntegrity;

// ---------------------------------------------------------------------------
// Public types
// ---------------------------------------------------------------------------

/// Classification of a single integrity check.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IntegrityStatus {
    /// Hash matches the manifest entry.
    Pass,
    /// File exists but hash does not match.
    Fail,
    /// File does not exist on disk.
    Missing,
}

/// Result of checking one file's integrity.
#[derive(Debug, Clone)]
pub struct IntegrityResult {
    /// Relative path as recorded in the manifest.
    pub path: String,
    /// Hash from the manifest.
    pub expected_hash: String,
    /// Computed hash, or `None` when the file is absent.
    pub actual_hash: Option<String>,
    /// Pass / Fail / Missing.
    pub status: IntegrityStatus,
}

// ---------------------------------------------------------------------------
// Hash helper
// ---------------------------------------------------------------------------

/// Compute SHA-256 hex digest of `bytes`.
pub(crate) fn sha256_hex(bytes: &[u8]) -> String {
    use std::fmt::Write as _;
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    let result = hasher.finalize();
    result.iter().fold(String::new(), |mut s, b| {
        let _ = write!(s, "{b:02x}");
        s
    })
}

/// Verify a single file at `disk_path` against `expected_hash`.
fn check_file(path: &str, disk_path: &Path, expected_hash: &str) -> IntegrityResult {
    match std::fs::read(disk_path) {
        Err(_) => IntegrityResult {
            path: path.to_string(),
            expected_hash: expected_hash.to_string(),
            actual_hash: None,
            status: IntegrityStatus::Missing,
        },
        Ok(bytes) => {
            let actual = sha256_hex(&bytes);
            let status = if actual == expected_hash {
                IntegrityStatus::Pass
            } else {
                IntegrityStatus::Fail
            };
            IntegrityResult {
                path: path.to_string(),
                expected_hash: expected_hash.to_string(),
                actual_hash: Some(actual),
                status,
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Synchronous API
// ---------------------------------------------------------------------------

/// Verify every file in `manifest` (trees + root_files) against the filesystem.
///
/// `repo_root` is the directory that relative paths in the manifest are
/// resolved against.
///
/// Results are returned in deterministic order (sorted by path).
pub fn verify_integrity(manifest: &AssetIntegrity, repo_root: &Path) -> Vec<IntegrityResult> {
    let mut results: Vec<IntegrityResult> = Vec::new();

    // Walk all files from every tree
    for tree in &manifest.trees {
        for file in &tree.files {
            let disk_path = repo_root.join(&file.path);
            results.push(check_file(&file.path, &disk_path, &file.sha256));
        }
    }

    // Root files
    for file in &manifest.root_files {
        let disk_path = repo_root.join(&file.path);
        results.push(check_file(&file.path, &disk_path, &file.sha256));
    }

    results.sort_by(|a, b| a.path.cmp(&b.path));
    results
}

// ---------------------------------------------------------------------------
// Async / parallel API (up to 8 concurrent I/O operations)
// ---------------------------------------------------------------------------

/// Async version of [`verify_integrity`] with up to 8 concurrent I/O ops.
///
/// `repo_root` is cloned into each task so callers need not hold a `'static`
/// reference.
pub async fn verify_integrity_parallel(
    manifest: &AssetIntegrity,
    repo_root: &Path,
) -> Vec<IntegrityResult> {
    const MAX_CONCURRENT: usize = 8;
    let sem = Arc::new(Semaphore::new(MAX_CONCURRENT));

    // Collect all (path_str, expected_hash, disk_path) triples
    let mut entries: Vec<(String, String, PathBuf)> = Vec::new();

    for tree in &manifest.trees {
        for file in &tree.files {
            entries.push((
                file.path.clone(),
                file.sha256.clone(),
                repo_root.join(&file.path),
            ));
        }
    }
    for file in &manifest.root_files {
        entries.push((
            file.path.clone(),
            file.sha256.clone(),
            repo_root.join(&file.path),
        ));
    }

    let mut handles = Vec::with_capacity(entries.len());

    for (path_str, expected, disk_path) in entries {
        let permit = Arc::clone(&sem);
        let handle = tokio::spawn(async move {
            let _guard = permit.acquire_owned().await.expect("semaphore closed");
            // Offload blocking I/O to the blocking thread pool
            tokio::task::spawn_blocking(move || check_file(&path_str, &disk_path, &expected))
                .await
                .expect("spawn_blocking panicked")
        });
        handles.push(handle);
    }

    let mut results: Vec<IntegrityResult> = Vec::with_capacity(handles.len());
    for handle in handles {
        results.push(handle.await.expect("task panicked"));
    }

    results.sort_by(|a, b| a.path.cmp(&b.path));
    results
}

// ---------------------------------------------------------------------------
// Manifest-change detection helper
// ---------------------------------------------------------------------------

/// Returns `true` when `current_manifest_hash` differs from the previously
/// cached hash, indicating the manifest was regenerated.
///
/// If `cached_hash` is `None` (never cached), returns `true` so callers
/// treat the first encounter as a change.
pub fn manifest_changed(current_manifest_hash: &str, cached_hash: Option<&str>) -> bool {
    match cached_hash {
        None => true,
        Some(cached) => current_manifest_hash != cached,
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::integrity::{AssetIntegrity, IntegrityFile, IntegrityScope, IntegrityTree};
    use proptest::prelude::*;
    use std::fs;
    use tempfile::TempDir;

    // -----------------------------------------------------------------------
    // Helper: build a minimal AssetIntegrity manifest
    // -----------------------------------------------------------------------

    fn make_manifest(files: Vec<IntegrityFile>) -> AssetIntegrity {
        AssetIntegrity {
            manifest_version: 1,
            algorithm: "sha256".to_string(),
            scope: IntegrityScope {
                trees: vec![],
                root_files: vec![],
            },
            trees: vec![IntegrityTree {
                tree: "test-tree".to_string(),
                aggregate_sha256: "0".to_string(),
                files,
            }],
            root_files: vec![],
            aggregate_sha256: "0".to_string(),
        }
    }

    // -----------------------------------------------------------------------
    // Deterministic unit tests
    // -----------------------------------------------------------------------

    #[test]
    fn pass_when_hash_matches() {
        let dir = TempDir::new().unwrap();
        let content = b"hello world";
        let hash = sha256_hex(content);
        let rel = "file.txt";
        fs::write(dir.path().join(rel), content).unwrap();

        let manifest = make_manifest(vec![IntegrityFile {
            path: rel.to_string(),
            sha256: hash.clone(),
            bytes: content.len() as u64,
        }]);

        let results = verify_integrity(&manifest, dir.path());
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].status, IntegrityStatus::Pass);
        assert_eq!(results[0].actual_hash.as_deref(), Some(hash.as_str()));
    }

    #[test]
    fn fail_when_content_changed() {
        let dir = TempDir::new().unwrap();
        let original = b"original content";
        let hash = sha256_hex(original);
        let rel = "file.txt";
        // Write different content
        fs::write(dir.path().join(rel), b"tampered content").unwrap();

        let manifest = make_manifest(vec![IntegrityFile {
            path: rel.to_string(),
            sha256: hash,
            bytes: original.len() as u64,
        }]);

        let results = verify_integrity(&manifest, dir.path());
        assert_eq!(results[0].status, IntegrityStatus::Fail);
        assert!(results[0].actual_hash.is_some());
    }

    #[test]
    fn missing_when_file_absent() {
        let dir = TempDir::new().unwrap();
        let manifest = make_manifest(vec![IntegrityFile {
            path: "nonexistent.txt".to_string(),
            sha256: "abc123".to_string(),
            bytes: 0,
        }]);

        let results = verify_integrity(&manifest, dir.path());
        assert_eq!(results[0].status, IntegrityStatus::Missing);
        assert!(results[0].actual_hash.is_none());
    }

    #[test]
    fn results_sorted_by_path() {
        let dir = TempDir::new().unwrap();
        let content = b"x";
        let hash = sha256_hex(content);

        for name in &["z.txt", "a.txt", "m.txt"] {
            fs::write(dir.path().join(name), content).unwrap();
        }

        let manifest = make_manifest(vec![
            IntegrityFile {
                path: "z.txt".into(),
                sha256: hash.clone(),
                bytes: 1,
            },
            IntegrityFile {
                path: "a.txt".into(),
                sha256: hash.clone(),
                bytes: 1,
            },
            IntegrityFile {
                path: "m.txt".into(),
                sha256: hash.clone(),
                bytes: 1,
            },
        ]);

        let results = verify_integrity(&manifest, dir.path());
        let paths: Vec<&str> = results.iter().map(|r| r.path.as_str()).collect();
        assert_eq!(paths, vec!["a.txt", "m.txt", "z.txt"]);
    }

    #[test]
    fn manifest_changed_returns_true_when_different() {
        assert!(manifest_changed("new_hash", Some("old_hash")));
    }

    #[test]
    fn manifest_changed_returns_false_when_same() {
        assert!(!manifest_changed("same_hash", Some("same_hash")));
    }

    #[test]
    fn manifest_changed_returns_true_when_no_cache() {
        assert!(manifest_changed("any_hash", None));
    }

    // Async parallel test
    #[tokio::test]
    async fn parallel_verify_matches_sync() {
        let dir = TempDir::new().unwrap();
        let content = b"parallel test content";
        let hash = sha256_hex(content);

        for i in 0..10 {
            fs::write(dir.path().join(format!("file{i}.txt")), content).unwrap();
        }

        let files: Vec<IntegrityFile> = (0..10)
            .map(|i| IntegrityFile {
                path: format!("file{i}.txt"),
                sha256: hash.clone(),
                bytes: content.len() as u64,
            })
            .collect();

        let manifest = make_manifest(files);

        let sync_results = verify_integrity(&manifest, dir.path());
        let async_results = verify_integrity_parallel(&manifest, dir.path()).await;

        assert_eq!(sync_results.len(), async_results.len());
        for (s, a) in sync_results.iter().zip(async_results.iter()) {
            assert_eq!(s.path, a.path);
            assert_eq!(s.status, a.status);
            assert_eq!(s.actual_hash, a.actual_hash);
        }
    }

    // -----------------------------------------------------------------------
    // Property tests (proptest) — Property 14
    // -----------------------------------------------------------------------

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(256))]

        /// Property 14a — Pass for untouched files, Fail for mutated files,
        /// Missing for deleted files.
        ///
        /// Strategy:
        ///   - Generate N files with random content
        ///   - Hash them into a manifest
        ///   - Optionally mutate some files (by appending a byte)
        ///   - Optionally delete some files
        ///   - Verify the classify matches expectations
        #[test]
        fn prop_integrity_classify_correct(
            // raw_files: list of (file_name_suffix, content_bytes, mutate, delete)
            raw_files in prop::collection::vec(
                (
                    0u8..200,                          // file index / name disambiguator
                    prop::collection::vec(any::<u8>(), 1..50), // content
                    prop::bool::ANY,                   // mutate?
                    prop::bool::ANY,                   // delete?
                ),
                1..10,
            ),
        ) {
            let dir = TempDir::new().unwrap();
            let mut manifest_files: Vec<IntegrityFile> = Vec::new();

            // State tracking for assertions
            let mut expect_pass: Vec<String> = Vec::new();
            let mut expect_fail: Vec<String> = Vec::new();
            let mut expect_missing: Vec<String> = Vec::new();

            for (iter_idx, (_idx, content, mutate, delete)) in raw_files.iter().enumerate() {
                // Use iter_idx (not the generated _idx) to guarantee unique file names.
                let name = format!("f{iter_idx}.bin");
                let path = dir.path().join(&name);
                let hash = sha256_hex(content);

                fs::write(&path, content).unwrap();

                manifest_files.push(IntegrityFile {
                    path: name.clone(),
                    sha256: hash,
                    bytes: content.len() as u64,
                });

                if *delete {
                    fs::remove_file(&path).unwrap();
                    expect_missing.push(name);
                } else if *mutate {
                    // Append a byte to change the hash
                    let mut appended = content.clone();
                    appended.push(0xFF);
                    fs::write(&path, &appended).unwrap();
                    expect_fail.push(name);
                } else {
                    expect_pass.push(name);
                }
            }

            let manifest = make_manifest(manifest_files);
            let results = verify_integrity(&manifest, dir.path());

            // Build result maps for O(1) lookup
            let result_map: std::collections::HashMap<&str, &IntegrityStatus> =
                results.iter().map(|r| (r.path.as_str(), &r.status)).collect();

            for name in &expect_pass {
                let got = result_map.get(name.as_str()).copied();
                prop_assert!(
                    got == Some(&IntegrityStatus::Pass),
                    "expected Pass for {:?}, got {:?}",
                    name,
                    got
                );
            }
            for name in &expect_fail {
                let got = result_map.get(name.as_str()).copied();
                prop_assert!(
                    got == Some(&IntegrityStatus::Fail),
                    "expected Fail for {:?}, got {:?}",
                    name,
                    got
                );
            }
            for name in &expect_missing {
                let got = result_map.get(name.as_str()).copied();
                prop_assert!(
                    got == Some(&IntegrityStatus::Missing),
                    "expected Missing for {:?}, got {:?}",
                    name,
                    got
                );
            }
        }

        /// Property 14b — manifest_changed is a pure equality check.
        #[test]
        fn prop_manifest_changed_pure(
            hash_a in "[0-9a-f]{64}",
            hash_b in "[0-9a-f]{64}",
        ) {
            // When hashes are identical the function must return false
            prop_assert!(!manifest_changed(&hash_a, Some(&hash_a)));
            // When hashes differ the function must return true (unless they happen to be equal)
            if hash_a != hash_b {
                prop_assert!(manifest_changed(&hash_a, Some(&hash_b)));
            }
        }
    }
}
