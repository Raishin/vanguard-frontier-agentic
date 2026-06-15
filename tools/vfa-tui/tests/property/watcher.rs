//! Feature: rust-tui-v2, Property 30 — filesystem watcher event classification.
//! Validates: Requirements 4.x (live reload routing)
//!
//! `classify_path` deterministically routes a changed path to exactly one of
//! Registry / Catalog / Workspace, with registry taking precedence, and returns
//! None for unrelated paths. (Debounce/coalescing *timing* is covered by the
//! in-crate `watcher` unit test; this pins the pure routing that feeds it.)

use std::path::{Path, PathBuf};

use proptest::prelude::*;
use proptest::test_runner::Config;

use vfa_tui::catalog::watcher::{classify_path, WatcherEvent};

proptest! {
    #![proptest_config(Config::with_cases(256))]

    /// A change exactly at the registry path classifies as Registry.
    #[test]
    fn registry_path_is_registry(sub in "[a-z]{1,8}") {
        let catalog = PathBuf::from("/root/catalog");
        let registry = PathBuf::from("/root/reg/workspaces.toml");
        let ws = vec![PathBuf::from("/root/ws1")];
        let _ = sub; // keep signature uniform
        let got = classify_path(&registry, &catalog, Some(&registry), &ws);
        prop_assert_eq!(got, Some(WatcherEvent::Registry));
    }

    /// A change under the catalog dir (and not the registry) classifies as
    /// Catalog, carrying the changed path.
    #[test]
    fn catalog_subpath_is_catalog(rel in "[a-z][a-z0-9/_-]{0,20}") {
        let catalog = PathBuf::from("/root/catalog");
        let registry = PathBuf::from("/root/reg/workspaces.toml");
        let ws = vec![PathBuf::from("/root/ws1")];
        let changed = catalog.join(&rel);
        let got = classify_path(&changed, &catalog, Some(&registry), &ws);
        prop_assert_eq!(got, Some(WatcherEvent::Catalog(changed)));
    }

    /// A change under a workspace path (and not registry/catalog) classifies as
    /// Workspace.
    #[test]
    fn workspace_subpath_is_workspace(idx in 0usize..2, rel in "[a-z][a-z0-9/_-]{0,20}") {
        let catalog = PathBuf::from("/root/catalog");
        let registry = PathBuf::from("/root/reg/workspaces.toml");
        let ws = vec![PathBuf::from("/root/ws1"), PathBuf::from("/root/ws2")];
        let changed = ws[idx].join(&rel);
        let got = classify_path(&changed, &catalog, Some(&registry), &ws);
        prop_assert_eq!(got, Some(WatcherEvent::Workspace(changed)));
    }

    /// An unrelated path classifies as None.
    #[test]
    fn unrelated_path_is_none(rel in "[a-z][a-z0-9/_-]{0,20}") {
        let catalog = PathBuf::from("/root/catalog");
        let registry = PathBuf::from("/root/reg/workspaces.toml");
        let ws = vec![PathBuf::from("/root/ws1")];
        let changed = Path::new("/elsewhere/totally").join(&rel);
        prop_assert_eq!(classify_path(&changed, &catalog, Some(&registry), &ws), None);
    }

    /// Exactly one classification ever fires (mutual exclusivity of the routing):
    /// the result is None or one specific variant, never ambiguous.
    #[test]
    fn classification_is_single_valued(rel in "[a-z][a-z0-9/_-]{0,16}", bucket in 0usize..4) {
        let catalog = PathBuf::from("/root/catalog");
        let registry = PathBuf::from("/root/reg/workspaces.toml");
        let ws = vec![PathBuf::from("/root/wsA")];
        let changed = match bucket {
            0 => registry.clone(),
            1 => catalog.join(&rel),
            2 => ws[0].join(&rel),
            _ => PathBuf::from("/nowhere").join(&rel),
        };
        let got = classify_path(&changed, &catalog, Some(&registry), &ws);
        let expected = match bucket {
            0 => Some(WatcherEvent::Registry),
            1 => Some(WatcherEvent::Catalog(changed.clone())),
            2 => Some(WatcherEvent::Workspace(changed.clone())),
            _ => None,
        };
        prop_assert_eq!(got, expected);
    }
}

#[test]
fn registry_takes_precedence_when_under_catalog() {
    // If the registry file lives inside the catalog dir, registry wins.
    let catalog = PathBuf::from("/root/catalog");
    let registry = catalog.join("workspaces.toml");
    let got = classify_path(&registry, &catalog, Some(&registry), &[]);
    assert_eq!(got, Some(WatcherEvent::Registry));
}

#[test]
fn no_registry_configured_still_routes_catalog() {
    let catalog = PathBuf::from("/root/catalog");
    let changed = catalog.join("agents.json");
    let got = classify_path(&changed, &catalog, None, &[]);
    assert_eq!(got, Some(WatcherEvent::Catalog(changed)));
}
