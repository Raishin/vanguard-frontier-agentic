//! Integration test for the TUI live-reload methods wired into the watcher
//! event loop (Task 9.1). The `tokio::select!` plumbing is exercised at runtime;
//! here we verify the reload behavior the watcher branch invokes.
//! Validates: Requirements 4.x (live reload), 5.x (safe rollback on parse error)

use std::path::{Path, PathBuf};

use vfa_tui::app::App;
use vfa_tui::catalog::store::{CatalogStore, ReloadOutcome};

fn fixtures_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests").join("fixtures")
}

/// Copy `tests/fixtures/catalog/*.json` into a fresh temp workspace root.
fn temp_workspace() -> tempfile::TempDir {
    let tmp = tempfile::TempDir::new().unwrap();
    let src = fixtures_root().join("catalog");
    let dst = tmp.path().join("catalog");
    std::fs::create_dir_all(&dst).unwrap();
    for entry in std::fs::read_dir(&src).unwrap() {
        let e = entry.unwrap();
        std::fs::copy(e.path(), dst.join(e.file_name())).unwrap();
    }
    tmp
}

fn app_for(ws: &Path) -> App {
    let catalog = CatalogStore::load(ws);
    App::new(catalog, ws.to_path_buf(), uuid::Uuid::new_v4(), true)
}

#[test]
fn reload_catalog_picks_up_deleted_file() {
    let ws = temp_workspace();
    let mut app = app_for(ws.path());
    assert_eq!(app.catalog.skill_count(), 3);

    // Remove skills.json on disk, then reload the whole catalog.
    std::fs::remove_file(ws.path().join("catalog").join("skills.json")).unwrap();
    app.reload_catalog();

    assert_eq!(app.catalog.skill_count(), 0, "deleted skills picked up on reload");
    assert!(app.dirty, "reload marks the app dirty for re-render");
}

#[test]
fn reload_catalog_file_retains_previous_on_parse_error() {
    let ws = temp_workspace();
    let mut app = app_for(ws.path());
    let before = app.catalog.agent_count();
    assert_eq!(before, 5);

    // Corrupt agents.json, then reload just that file.
    let agents = ws.path().join("catalog").join("agents.json");
    std::fs::write(&agents, b"{ this is not valid json").unwrap();
    let outcome = app.reload_catalog_file(&agents);

    assert!(
        matches!(outcome, ReloadOutcome::RetainedPrevious { .. }),
        "a parse error must retain the previous good catalog, got {outcome:?}"
    );
    assert_eq!(app.catalog.agent_count(), 5, "previous agents retained on parse error");
}

#[test]
fn reload_catalog_file_unchanged_is_noop() {
    let ws = temp_workspace();
    let mut app = app_for(ws.path());
    let agents = ws.path().join("catalog").join("agents.json");

    // No modification since load → the content hash matches → Unchanged.
    let outcome = app.reload_catalog_file(&agents);
    assert!(
        matches!(outcome, ReloadOutcome::Unchanged),
        "unmodified file should be Unchanged, got {outcome:?}"
    );
    assert_eq!(app.catalog.agent_count(), 5);
}
