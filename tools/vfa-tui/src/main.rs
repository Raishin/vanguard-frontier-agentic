#![deny(warnings)]
#![allow(dead_code)]
#![allow(unused_imports)]

mod app;
mod catalog;
mod cli;
mod error;
mod logging;
mod models;
mod search;
mod security;
mod subprocess;
mod ui;
mod workspace;

use clap::Parser;

use crate::cli::Cli;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    // Install panic hook for terminal restoration
    ui::install_panic_hook();

    // Detect workspace
    let workspace_root = workspace::detect::detect_workspace(cli.workspace.as_deref())?;

    // Init logging
    let session_id = uuid::Uuid::new_v4();
    logging::audit::init_logging(
        cli.log_file.as_deref(),
        &format!("{:?}", cli.log_level).to_lowercase(),
        session_id,
    )?;

    // Load catalog
    let catalog = catalog::store::CatalogStore::load(&workspace_root);
    if !catalog.load_errors.is_empty() {
        for err in &catalog.load_errors {
            tracing::warn!(%err, "catalog load warning");
        }
    }

    // Setup terminal
    let mut terminal_mgr = ui::TerminalManager::new()?;

    // Create app
    let mut app = app::App::new(catalog, workspace_root, session_id, cli.no_color);

    // Event loop
    loop {
        terminal_mgr.draw(|frame| app.render(frame))?;

        if crossterm::event::poll(std::time::Duration::from_millis(100))? {
            if let crossterm::event::Event::Key(key) = crossterm::event::read()? {
                app.handle_key_event(key);
            }
        }

        app.tick();

        if app.should_quit {
            break;
        }
    }

    terminal_mgr.restore()?;
    Ok(())
}
