//! Integration tests for the primary App::render v2 tab bar surface (Residual 2).
//!
//! Verifies that `App::render` produces the tab-bar + tab-body layout and that
//! switching tabs changes the rendered content.

use std::path::{Path, PathBuf};

use ratatui::backend::TestBackend;
use ratatui::Terminal;

use vfa_tui::app::App;
use vfa_tui::catalog::store::CatalogStore;
use vfa_tui::ui::nav::Tab;

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

fn render_app_text(app: &mut App) -> String {
    let backend = TestBackend::new(120, 30);
    let mut terminal = Terminal::new(backend).unwrap();
    terminal.draw(|frame| app.render(frame)).unwrap();
    terminal
        .backend()
        .buffer()
        .content
        .iter()
        .map(|c| c.symbol().chars().next().unwrap_or(' '))
        .collect()
}

#[test]
fn primary_render_shows_tab_bar() {
    let mut app = app();
    let text = render_app_text(&mut app);
    // The tab bar title "Operator Console" or at least one tab label must appear.
    assert!(
        text.contains("Operator Console") || text.contains("Overview"),
        "primary render must show tab bar; first 200 chars: {:?}",
        &text[..200.min(text.len())]
    );
}

#[test]
fn switching_tabs_changes_active_body() {
    let mut app = app();
    // Start at Overview (default).
    assert_eq!(app.nav.current_tab, Tab::Overview);
    let text_overview = render_app_text(&mut app);

    // Switch to Dependencies tab.
    app.nav.current_tab = Tab::Dependencies;
    let text_deps = render_app_text(&mut app);

    // Content must differ between tabs.
    assert_ne!(
        text_overview, text_deps,
        "switching tabs must change rendered content"
    );
}

#[test]
fn all_tabs_render_without_panic() {
    for tab in Tab::ALL {
        let mut app = app();
        app.nav.current_tab = tab.clone();
        let text = render_app_text(&mut app);
        assert!(
            !text.trim().is_empty(),
            "tab {:?} produced empty frame in primary render",
            tab
        );
    }
}
