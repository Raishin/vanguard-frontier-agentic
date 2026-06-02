use serde::{Deserialize, Serialize};

/// Top-level asset integrity manifest.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AssetIntegrity {
    pub manifest_version: u32,
    pub algorithm: String,
    pub scope: IntegrityScope,
    pub trees: Vec<IntegrityTree>,
    pub root_files: Vec<IntegrityFile>,
    pub aggregate_sha256: String,
}

/// Defines which trees and root files are in scope for integrity checks.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct IntegrityScope {
    pub trees: Vec<String>,
    pub root_files: Vec<String>,
}

/// A directory tree with its aggregate hash and file entries.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct IntegrityTree {
    pub tree: String,
    pub aggregate_sha256: String,
    pub files: Vec<IntegrityFile>,
}

/// A single file entry with its hash and size.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct IntegrityFile {
    pub path: String,
    pub sha256: String,
    pub bytes: u64,
}
