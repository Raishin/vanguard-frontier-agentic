use ratatui::layout::{Constraint, Direction, Layout, Rect};

/// Computed layout regions for the application.
///
/// The layout is deterministic: identical terminal dimensions always produce
/// identical region coordinates (Requirement 18.1). No randomness, no caches,
/// no environment variables influence the computation — only the terminal
/// width and height (passed via `area`).
pub struct AppLayout {
    pub sidebar: Rect,
    pub main_content: Rect,
    pub status_bar: Rect,
    pub help_bar: Rect,
}

/// Fixed sidebar width for wide terminals (>80 columns).
const SIDEBAR_WIDTH_WIDE: u16 = 20;
/// Sidebar width for medium terminals (41–80 columns).
const SIDEBAR_WIDTH_MEDIUM: u16 = 16;
/// Sidebar width for narrow terminals (≤40 columns).
const SIDEBAR_WIDTH_NARROW: u16 = 12;

/// Status bar height (always 1 line at the bottom).
const STATUS_BAR_HEIGHT: u16 = 1;
/// Help bar height (always 1 line at the very bottom).
const HELP_BAR_HEIGHT: u16 = 1;

/// Minimum main content width before sidebar is hidden.
const MIN_CONTENT_WIDTH: u16 = 10;

/// Compute the layout regions given the terminal area.
///
/// Layout structure (Requirement 16.3):
/// - Vertical: main area | status bar (1 line) | help bar (1 line)
/// - Main area horizontal: sidebar (fixed width ~20 chars) | content (remaining)
///
/// The sidebar width adapts to terminal width:
/// - >80 columns: 20 chars
/// - 41–80 columns: 16 chars
/// - ≤40 columns: 12 chars
///
/// This function is pure — same `area` always produces the same `AppLayout`.
pub fn compute_layout(area: Rect) -> AppLayout {
    // Vertical split: main | status_bar | help_bar
    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(3),
            Constraint::Length(STATUS_BAR_HEIGHT),
            Constraint::Length(HELP_BAR_HEIGHT),
        ])
        .split(area);

    let main_area = vertical[0];
    let status_bar = vertical[1];
    let help_bar = vertical[2];

    // Horizontal split of main area: sidebar | content
    let sidebar_width = compute_sidebar_width(main_area.width);

    let horizontal = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(sidebar_width),
            Constraint::Min(MIN_CONTENT_WIDTH),
        ])
        .split(main_area);

    let sidebar = horizontal[0];
    let main_content = horizontal[1];

    AppLayout {
        sidebar,
        main_content,
        status_bar,
        help_bar,
    }
}

/// Compute the sidebar width based on available terminal width.
///
/// Returns a fixed width that adapts to the terminal size:
/// - Wide (>80): 20 columns
/// - Medium (41–80): 16 columns
/// - Narrow (≤40): 12 columns
fn compute_sidebar_width(terminal_width: u16) -> u16 {
    if terminal_width > 80 {
        SIDEBAR_WIDTH_WIDE
    } else if terminal_width > 40 {
        SIDEBAR_WIDTH_MEDIUM
    } else {
        SIDEBAR_WIDTH_NARROW
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compute_layout_standard_terminal() {
        let area = Rect::new(0, 0, 120, 40);
        let layout = compute_layout(area);
        assert_eq!(layout.sidebar.width, 20);
        assert_eq!(layout.main_content.x, 20);
        assert_eq!(layout.status_bar.height, 1);
        assert_eq!(layout.help_bar.height, 1);
        assert_eq!(layout.status_bar.y + 1, layout.help_bar.y);
    }

    #[test]
    fn compute_layout_small_terminal() {
        let area = Rect::new(0, 0, 50, 20);
        let layout = compute_layout(area);
        assert_eq!(layout.sidebar.width, 16);
        assert!(layout.main_content.width > 0);
    }

    #[test]
    fn compute_layout_very_small_terminal() {
        let area = Rect::new(0, 0, 30, 10);
        let layout = compute_layout(area);
        assert_eq!(layout.sidebar.width, 12);
        assert!(layout.main_content.width > 0);
    }

    #[test]
    fn compute_layout_minimum_viable() {
        let area = Rect::new(0, 0, 20, 5);
        let layout = compute_layout(area);
        // Should not panic even with tiny terminal
        assert!(layout.sidebar.width > 0 || layout.main_content.width > 0);
    }

    #[test]
    fn compute_layout_deterministic() {
        // Requirement 18.1: same dimensions produce identical layout
        let area = Rect::new(0, 0, 100, 30);
        let layout1 = compute_layout(area);
        let layout2 = compute_layout(area);
        assert_eq!(layout1.sidebar, layout2.sidebar);
        assert_eq!(layout1.main_content, layout2.main_content);
        assert_eq!(layout1.status_bar, layout2.status_bar);
        assert_eq!(layout1.help_bar, layout2.help_bar);
    }

    #[test]
    fn status_bar_is_second_to_last_line() {
        let area = Rect::new(0, 0, 80, 24);
        let layout = compute_layout(area);
        // Status bar is 1 line above help bar
        assert_eq!(layout.status_bar.y, area.height - 2);
        assert_eq!(layout.help_bar.y, area.height - 1);
    }

    #[test]
    fn help_bar_is_last_line() {
        let area = Rect::new(0, 0, 80, 24);
        let layout = compute_layout(area);
        assert_eq!(layout.help_bar.y + layout.help_bar.height, area.height);
    }

    #[test]
    fn sidebar_width_constants() {
        assert_eq!(compute_sidebar_width(120), SIDEBAR_WIDTH_WIDE);
        assert_eq!(compute_sidebar_width(81), SIDEBAR_WIDTH_WIDE);
        assert_eq!(compute_sidebar_width(80), SIDEBAR_WIDTH_MEDIUM);
        assert_eq!(compute_sidebar_width(41), SIDEBAR_WIDTH_MEDIUM);
        assert_eq!(compute_sidebar_width(40), SIDEBAR_WIDTH_NARROW);
        assert_eq!(compute_sidebar_width(20), SIDEBAR_WIDTH_NARROW);
    }

    #[test]
    fn main_content_fills_remaining_width() {
        let area = Rect::new(0, 0, 100, 30);
        let layout = compute_layout(area);
        assert_eq!(layout.sidebar.width + layout.main_content.width, area.width);
    }
}
