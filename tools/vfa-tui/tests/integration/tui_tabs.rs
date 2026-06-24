//! Integration test for v2 operator-console tab rendering (Task 11.3).
//! Verifies `App::render_tab` dispatches each v2 `Tab` to its widget via a
//! ratatui `TestBackend` without panicking and with tab-appropriate content.
//! Validates: Requirements 17.x (operator console tabs)

use std::path::{Path, PathBuf};

use ratatui::backend::TestBackend;
use ratatui::Terminal;

use vfa_tui::app::App;
use vfa_tui::catalog::store::CatalogStore;
use vfa_tui::ui::nav::Tab;
use vfa_tui::ui::theme::{Theme, ThemeMode};

fn fixtures_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
}

fn app() -> App {
    let root = fixtures_root();
    let catalog = CatalogStore::load(&root);
    App::new(catalog, root, uuid::Uuid::new_v4(), true)
}

/// Render `tab` and return the flattened buffer text.
fn render_tab_text(tab: Tab) -> String {
    let mut app = app();
    app.nav.current_tab = tab;
    let theme = Theme::new(true, ThemeMode::Dark);

    let backend = TestBackend::new(120, 30);
    let mut terminal = Terminal::new(backend).unwrap();
    terminal
        .draw(|frame| app.render_tab(frame.area(), frame, &theme))
        .unwrap();

    terminal
        .backend()
        .buffer()
        .content
        .iter()
        .map(|c| c.symbol().chars().next().unwrap_or(' '))
        .collect()
}

#[test]
fn overview_tab_shows_catalog_summary() {
    let text = render_tab_text(Tab::Overview);
    assert!(text.contains("Overview"), "overview title present");
    assert!(text.contains("Agents"), "overview lists agent count");
    assert!(text.contains("Skills"), "overview lists skill count");
}

#[test]
fn dependencies_tab_renders_real_graph() {
    // The dependency graph is built from the loaded catalog, so it must render
    // its header without panicking.
    let text = render_tab_text(Tab::Dependencies);
    assert!(
        text.contains("Dependency") || text.contains("Deps"),
        "dependencies tab renders the graph widget: {text:?}"
    );
}

#[test]
fn data_dependent_tabs_render_without_panic() {
    // Coverage, violations and audit render their widgets even with empty data
    // (pending the scan/index pipeline) — the key property is no panic and a
    // non-empty frame.
    for tab in [Tab::CoverageMatrix, Tab::PolicyViolations, Tab::AuditLog] {
        let label = format!("{tab:?}");
        let text = render_tab_text(tab);
        assert!(
            text.trim().chars().any(|c| c != ' '),
            "tab {label} produced an empty frame"
        );
    }
}
