pub mod layout;
pub mod nav;
pub mod theme;
pub mod widgets;

use std::io::{self, Stdout};

use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, layout::Rect, Frame, Terminal};

/// Manages the terminal lifecycle: raw mode, alternate screen, and rendering.
///
/// Guarantees terminal state restoration on drop (normal exit, error, or panic).
/// The panic hook installed via [`install_panic_hook`] provides an additional
/// safety net for restoration before the panic message is printed.
pub struct TerminalManager {
    terminal: Terminal<CrosstermBackend<Stdout>>,
}

impl TerminalManager {
    /// Initialize terminal: enable raw mode, switch to alternate screen, hide cursor.
    ///
    /// This must be called before any rendering. The terminal is restored
    /// automatically when the `TerminalManager` is dropped.
    pub fn new() -> anyhow::Result<Self> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, crossterm::cursor::Hide)?;
        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend)?;
        Ok(Self { terminal })
    }

    /// Restore the terminal to its original state: disable raw mode,
    /// leave alternate screen, show cursor.
    pub fn restore(&mut self) -> anyhow::Result<()> {
        disable_raw_mode()?;
        execute!(
            self.terminal.backend_mut(),
            LeaveAlternateScreen,
            crossterm::cursor::Show
        )?;
        Ok(())
    }

    /// Draw a single frame. On terminal resize, ratatui automatically picks up
    /// the new dimensions from the backend, so calling `draw` after a resize
    /// event produces a correctly laid-out frame.
    pub fn draw<F>(&mut self, f: F) -> anyhow::Result<()>
    where
        F: FnOnce(&mut Frame),
    {
        self.terminal.draw(f)?;
        Ok(())
    }

    /// Return the current terminal size as a [`Rect`].
    pub fn size(&self) -> anyhow::Result<Rect> {
        Ok(self.terminal.size()?.into())
    }
}

impl Drop for TerminalManager {
    fn drop(&mut self) {
        // Best-effort restoration — errors are intentionally ignored here
        // because we may be dropping during a panic or error unwind.
        let _ = self.restore();
    }
}

/// Install a panic hook that restores the terminal before printing the panic message.
///
/// This must be called **before** any terminal setup so that panics occurring
/// during or after terminal initialization still produce readable output.
/// The hook restores: raw mode disabled, alternate screen left, cursor shown.
pub fn install_panic_hook() {
    let original_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        // Best-effort terminal restoration before the panic message is printed.
        // Errors are ignored because we're already in a panic path.
        let _ = disable_raw_mode();
        let _ = execute!(io::stdout(), LeaveAlternateScreen, crossterm::cursor::Show);
        original_hook(panic_info);
    }));
}
