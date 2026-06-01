use ratatui::style::{Color, Modifier, Style};

/// Detected terminal color capability level.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorSupport {
    /// Terminal supports 256 colors (xterm-256color, etc.)
    TrueColor256,
    /// Terminal supports only basic 8 ANSI colors
    Basic8,
    /// No color output (--no-color flag or dumb terminal)
    None,
}

/// Detect the terminal's color support level.
///
/// Detection logic:
/// 1. If `no_color` is true (--no-color flag), returns `ColorSupport::None`
/// 2. Checks `COLORTERM` env var for "truecolor" or "24bit" → 256
/// 3. Checks `TERM` env var for "256color" substring → 256
/// 4. If `TERM` is "dumb" or empty → None
/// 5. Otherwise → Basic8 (safe fallback)
///
/// Note: Per Requirement 18.6, we do NOT use env vars to alter display content
/// or sort order. Color detection is the one permitted use of env vars for
/// rendering (it affects style, not content structure).
pub fn detect_color_support(no_color: bool) -> ColorSupport {
    if no_color {
        return ColorSupport::None;
    }

    // Check NO_COLOR convention (https://no-color.org/)
    if std::env::var("NO_COLOR").is_ok() {
        return ColorSupport::None;
    }

    // Check COLORTERM for truecolor/24bit support (implies 256+)
    if let Ok(colorterm) = std::env::var("COLORTERM") {
        let ct = colorterm.to_lowercase();
        if ct.contains("truecolor") || ct.contains("24bit") {
            return ColorSupport::TrueColor256;
        }
    }

    // Check TERM for 256color support
    if let Ok(term) = std::env::var("TERM") {
        let t = term.to_lowercase();
        if t.contains("256color") {
            return ColorSupport::TrueColor256;
        }
        if t == "dumb" || t.is_empty() {
            return ColorSupport::None;
        }
    }

    // Default: assume basic 8-color support
    ColorSupport::Basic8
}

/// Theme configuration supporting --no-color mode and 8-color fallback.
///
/// The theme is deterministic: given the same `ColorSupport` level, all style
/// methods return identical `Style` values (Requirement 18.1).
pub struct Theme {
    pub no_color: bool,
    pub color_support: ColorSupport,
}

impl Theme {
    /// Create a new theme with automatic color detection.
    pub fn new(no_color: bool) -> Self {
        let color_support = detect_color_support(no_color);
        Self {
            no_color,
            color_support,
        }
    }

    /// Create a theme with explicit color support level (useful for testing).
    pub fn with_color_support(color_support: ColorSupport) -> Self {
        Self {
            no_color: color_support == ColorSupport::None,
            color_support,
        }
    }

    /// Whether to use 256-color palette.
    fn use_256_colors(&self) -> bool {
        self.color_support == ColorSupport::TrueColor256
    }

    /// Whether any color output is enabled.
    fn has_color(&self) -> bool {
        self.color_support != ColorSupport::None
    }

    pub fn sidebar_style(&self) -> Style {
        if !self.has_color() {
            Style::default()
        } else {
            Style::default().fg(Color::White)
        }
    }

    pub fn sidebar_selected(&self) -> Style {
        if !self.has_color() {
            Style::default().add_modifier(Modifier::REVERSED)
        } else {
            // Cyan is a basic ANSI color, works in both 8 and 256 color modes
            Style::default()
                .fg(Color::Black)
                .bg(Color::Cyan)
                .add_modifier(Modifier::BOLD)
        }
    }

    pub fn list_item(&self) -> Style {
        if !self.has_color() {
            Style::default()
        } else {
            Style::default().fg(Color::White)
        }
    }

    pub fn list_selected(&self) -> Style {
        if !self.has_color() {
            Style::default().add_modifier(Modifier::REVERSED)
        } else if self.use_256_colors() {
            Style::default()
                .fg(Color::Black)
                .bg(Color::LightBlue)
                .add_modifier(Modifier::BOLD)
        } else {
            // 8-color fallback: use basic blue background
            Style::default()
                .fg(Color::Black)
                .bg(Color::Blue)
                .add_modifier(Modifier::BOLD)
        }
    }

    pub fn detail_key(&self) -> Style {
        if !self.has_color() {
            Style::default().add_modifier(Modifier::BOLD)
        } else {
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD)
        }
    }

    pub fn detail_value(&self) -> Style {
        if !self.has_color() {
            Style::default()
        } else {
            Style::default().fg(Color::White)
        }
    }

    pub fn status_bar(&self) -> Style {
        if !self.has_color() {
            Style::default().add_modifier(Modifier::REVERSED)
        } else {
            Style::default().fg(Color::Black).bg(Color::White)
        }
    }

    pub fn help_bar(&self) -> Style {
        if !self.has_color() {
            Style::default().add_modifier(Modifier::DIM)
        } else if self.use_256_colors() {
            Style::default().fg(Color::DarkGray)
        } else {
            // 8-color: use dim modifier with white
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::DIM)
        }
    }

    pub fn error_style(&self) -> Style {
        if !self.has_color() {
            Style::default().add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)
        }
    }

    pub fn search_style(&self) -> Style {
        if !self.has_color() {
            Style::default().add_modifier(Modifier::UNDERLINED)
        } else {
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD)
        }
    }

    pub fn border_style(&self) -> Style {
        if !self.has_color() {
            Style::default()
        } else if self.use_256_colors() {
            Style::default().fg(Color::Gray)
        } else {
            Style::default().fg(Color::White)
        }
    }

    /// Style for the focused panel border (Requirement 16.6).
    /// Visually distinct from unfocused borders to indicate active panel.
    pub fn border_focused(&self) -> Style {
        if !self.has_color() {
            Style::default().add_modifier(Modifier::BOLD)
        } else {
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD)
        }
    }

    /// Style for unfocused panel border (Requirement 16.6).
    /// Subdued compared to focused border.
    pub fn border_unfocused(&self) -> Style {
        if !self.has_color() {
            Style::default().add_modifier(Modifier::DIM)
        } else if self.use_256_colors() {
            Style::default().fg(Color::DarkGray)
        } else {
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::DIM)
        }
    }

    /// Style for validation gate status: NotRun (dim/gray).
    pub fn gate_not_run(&self) -> Style {
        if !self.has_color() {
            Style::default().add_modifier(Modifier::DIM)
        } else if self.use_256_colors() {
            Style::default().fg(Color::DarkGray)
        } else {
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::DIM)
        }
    }

    /// Style for validation gate status: Running (yellow).
    pub fn gate_running(&self) -> Style {
        if !self.has_color() {
            Style::default().add_modifier(Modifier::BOLD)
        } else {
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD)
        }
    }

    /// Style for validation gate status: Passed (green).
    pub fn gate_passed(&self) -> Style {
        if !self.has_color() {
            Style::default()
        } else {
            Style::default().fg(Color::Green)
        }
    }

    /// Style for validation gate status: Failed (red).
    pub fn gate_failed(&self) -> Style {
        if !self.has_color() {
            Style::default().add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)
        }
    }

    /// Style for validation gate status: TimedOut (magenta).
    pub fn gate_timed_out(&self) -> Style {
        if !self.has_color() {
            Style::default().add_modifier(Modifier::BOLD)
        } else {
            Style::default()
                .fg(Color::Magenta)
                .add_modifier(Modifier::BOLD)
        }
    }

    /// Style for filter chips display.
    pub fn filter_chip(&self) -> Style {
        if !self.has_color() {
            Style::default().add_modifier(Modifier::REVERSED)
        } else {
            Style::default().fg(Color::Black).bg(Color::Cyan)
        }
    }

    /// Style for sparkline bar (filled portion).
    pub fn sparkline_filled(&self) -> Style {
        if !self.has_color() {
            Style::default()
        } else {
            Style::default().fg(Color::Cyan)
        }
    }

    /// Style for sparkline bar (empty portion).
    pub fn sparkline_empty(&self) -> Style {
        if !self.has_color() {
            Style::default().add_modifier(Modifier::DIM)
        } else if self.use_256_colors() {
            Style::default().fg(Color::DarkGray)
        } else {
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::DIM)
        }
    }

    /// Style for help overlay title.
    pub fn help_overlay_title(&self) -> Style {
        if !self.has_color() {
            Style::default().add_modifier(Modifier::BOLD)
        } else {
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD)
        }
    }

    /// Style for help overlay section header.
    pub fn help_overlay_section(&self) -> Style {
        if !self.has_color() {
            Style::default().add_modifier(Modifier::BOLD)
        } else {
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD)
        }
    }

    /// Style for completion suggestion (highlighted).
    pub fn completion_highlight(&self) -> Style {
        if !self.has_color() {
            Style::default().add_modifier(Modifier::REVERSED)
        } else if self.use_256_colors() {
            Style::default().fg(Color::Black).bg(Color::LightGreen)
        } else {
            Style::default().fg(Color::Black).bg(Color::Green)
        }
    }

    /// Style for completion suggestion (normal).
    pub fn completion_normal(&self) -> Style {
        if !self.has_color() {
            Style::default()
        } else {
            Style::default().fg(Color::White)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_color_mode_uses_no_fg_color() {
        let theme = Theme::with_color_support(ColorSupport::None);
        assert_eq!(theme.sidebar_style().fg, None);
        assert_eq!(theme.list_item().fg, None);
        assert_eq!(theme.detail_value().fg, None);
    }

    #[test]
    fn color_mode_has_colors() {
        let theme = Theme::with_color_support(ColorSupport::TrueColor256);
        assert!(theme.sidebar_style().fg.is_some());
        assert!(theme.list_selected().bg.is_some());
        assert!(theme.detail_key().fg.is_some());
    }

    #[test]
    fn eight_color_mode_uses_basic_colors() {
        let theme = Theme::with_color_support(ColorSupport::Basic8);
        // 8-color mode should still have colors
        assert!(theme.sidebar_style().fg.is_some());
        assert!(theme.list_selected().bg.is_some());
        // list_selected uses Blue (basic) instead of LightBlue (256)
        assert_eq!(theme.list_selected().bg, Some(Color::Blue));
    }

    #[test]
    fn focused_border_differs_from_unfocused() {
        let theme = Theme::with_color_support(ColorSupport::TrueColor256);
        let focused = theme.border_focused();
        let unfocused = theme.border_unfocused();
        // Focused should be visually distinct
        assert_ne!(focused.fg, unfocused.fg);
    }

    #[test]
    fn no_color_focused_uses_modifiers() {
        let theme = Theme::with_color_support(ColorSupport::None);
        let focused = theme.border_focused();
        let unfocused = theme.border_unfocused();
        // In no-color mode, focused uses BOLD, unfocused uses DIM
        assert!(focused.add_modifier.contains(Modifier::BOLD));
        assert!(unfocused.add_modifier.contains(Modifier::DIM));
    }

    #[test]
    fn color_support_detection_no_color_flag() {
        assert_eq!(detect_color_support(true), ColorSupport::None);
    }

    #[test]
    fn theme_deterministic_same_support_level() {
        // Requirement 18.1: same inputs produce same styles
        let theme1 = Theme::with_color_support(ColorSupport::TrueColor256);
        let theme2 = Theme::with_color_support(ColorSupport::TrueColor256);
        assert_eq!(theme1.sidebar_style(), theme2.sidebar_style());
        assert_eq!(theme1.list_selected(), theme2.list_selected());
        assert_eq!(theme1.error_style(), theme2.error_style());
        assert_eq!(theme1.border_focused(), theme2.border_focused());
    }

    #[test]
    fn with_color_support_sets_no_color_correctly() {
        let theme_none = Theme::with_color_support(ColorSupport::None);
        assert!(theme_none.no_color);

        let theme_256 = Theme::with_color_support(ColorSupport::TrueColor256);
        assert!(!theme_256.no_color);

        let theme_8 = Theme::with_color_support(ColorSupport::Basic8);
        assert!(!theme_8.no_color);
    }

    #[test]
    fn gate_styles_have_color_in_color_mode() {
        let theme = Theme::with_color_support(ColorSupport::TrueColor256);
        assert!(theme.gate_passed().fg.is_some());
        assert!(theme.gate_failed().fg.is_some());
        assert!(theme.gate_running().fg.is_some());
        assert!(theme.gate_timed_out().fg.is_some());
    }

    #[test]
    fn help_bar_8color_uses_dim() {
        let theme = Theme::with_color_support(ColorSupport::Basic8);
        let style = theme.help_bar();
        assert!(style.add_modifier.contains(Modifier::DIM));
    }
}
