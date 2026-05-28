use ratatui::layout::{Constraint, Direction, Layout, Rect};

/// Computed layout regions for the application.
pub struct AppLayout {
    pub sidebar: Rect,
    pub main_content: Rect,
    pub status_bar: Rect,
    pub help_bar: Rect,
}

/// Compute the layout regions given the terminal area.
///
/// Layout structure:
/// - Vertical: main area | status bar (1 line) | help bar (1 line)
/// - Main area horizontal: sidebar (20 cols min, ~20%) | content (rest)
pub fn compute_layout(area: Rect) -> AppLayout {
    // Vertical split: main | status_bar | help_bar
    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(3),
            Constraint::Length(1),
            Constraint::Length(1),
        ])
        .split(area);

    let main_area = vertical[0];
    let status_bar = vertical[1];
    let help_bar = vertical[2];

    // Horizontal split of main area: sidebar | content
    let sidebar_width = if main_area.width > 80 {
        20
    } else if main_area.width > 40 {
        16
    } else {
        12
    };

    let horizontal = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(sidebar_width), Constraint::Min(10)])
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
}
