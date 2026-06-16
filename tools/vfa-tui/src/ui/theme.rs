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

// ---------------------------------------------------------------------------
// Theme mode + detection (Requirement 35)
// ---------------------------------------------------------------------------

/// Resolved theme mode — always Dark or Light at runtime (never System/Auto).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThemeMode {
    Dark,
    Light,
}

/// CLI-level theme preference before resolution (Req 35.3).
#[derive(Debug, Clone, Copy, PartialEq, Eq, clap::ValueEnum)]
pub enum ThemePreference {
    /// Detect from terminal background (default).
    Auto,
    /// Force dark palette.
    Dark,
    /// Force light palette.
    Light,
}

/// Parse the `COLORFGBG` environment variable value into a [`ThemeMode`].
///
/// Format is `fg;bg` (some terminals emit `fg;default;bg`); the **last**
/// field is the background ANSI colour index. A background index ≥ 7 means a
/// light terminal (Req 35.2). Returns `None` when the value is empty or the
/// background field cannot be parsed, so the caller can fall through to the
/// next heuristic.
pub fn parse_colorfgbg(value: &str) -> Option<ThemeMode> {
    let bg_str = value.rsplit(';').next()?.trim();
    let bg_idx: u8 = bg_str.parse().ok()?;
    Some(if bg_idx >= 7 {
        ThemeMode::Light
    } else {
        ThemeMode::Dark
    })
}

/// Pure classification of detection inputs into a [`ThemeMode`].
///
/// Priority (Req 35.1 → 35.2 → fallback):
/// 1. `luma` from the `terminal-light` OSC 11 query: `> 0.6` → Light, else Dark.
/// 2. `COLORFGBG` parsing via [`parse_colorfgbg`].
/// 3. Default to Dark (safe assumption for most developer terminals).
///
/// Kept side-effect free so the full decision tree is unit-testable without
/// touching the terminal or environment.
fn classify_theme(luma: Option<f32>, colorfgbg: Option<&str>) -> ThemeMode {
    if let Some(l) = luma {
        return if l > 0.6 {
            ThemeMode::Light
        } else {
            ThemeMode::Dark
        };
    }
    if let Some(value) = colorfgbg {
        if let Some(mode) = parse_colorfgbg(value) {
            return mode;
        }
    }
    ThemeMode::Dark
}

/// Detect the system theme from terminal background luminance (Req 35.1–35.2).
///
/// Primary: `terminal-light` (OSC 11 escape query). Secondary: `COLORFGBG`.
/// Fallback: Dark. Performs terminal/environment I/O — only call from an
/// interactive (TUI) startup path, never from headless mode (Req 35.9).
pub fn detect_system_theme() -> ThemeMode {
    let luma = terminal_light::luma().ok();
    let colorfgbg = std::env::var("COLORFGBG").ok();
    classify_theme(luma, colorfgbg.as_deref())
}

/// Resolve a CLI [`ThemePreference`] into a concrete [`ThemeMode`] (Req 35.3, 35.9).
///
/// `Auto` triggers system detection, except in headless mode where stdin may
/// not be a TTY — there it defaults to Dark without probing the terminal.
pub fn resolve_theme(preference: ThemePreference, is_headless: bool) -> ThemeMode {
    match preference {
        ThemePreference::Dark => ThemeMode::Dark,
        ThemePreference::Light => ThemeMode::Light,
        ThemePreference::Auto => {
            if is_headless {
                ThemeMode::Dark
            } else {
                detect_system_theme()
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Palette (Requirement 35.4 / 35.5)
// ---------------------------------------------------------------------------

/// Semantic colour palette — all theme-dependent colours in one place.
///
/// Style methods pull from the active palette rather than hardcoding colours,
/// so a single palette swap re-themes the entire UI.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Palette {
    /// Primary text.
    pub fg: Color,
    /// Secondary/muted text.
    pub fg_dim: Color,
    /// `None` = inherit terminal background.
    pub bg: Option<Color>,
    /// Alternate background for contrast (status bar, selections).
    pub bg_alt: Option<Color>,
    /// Primary accent (focused borders, highlights).
    pub accent: Color,
    /// Subdued accent (unfocused, help text).
    pub accent_dim: Color,
    /// Pass, installed, current.
    pub success: Color,
    /// Outdated, stale, warnings.
    pub warning: Color,
    /// Failed, drifted, critical violations.
    pub error: Color,
    /// Informational, running, in-progress.
    pub info: Color,
    /// Default border colour.
    pub border: Color,
    /// Focused panel border.
    pub border_focused: Color,
    /// Foreground in selected/highlighted items.
    pub selection_fg: Color,
    /// Background in selected/highlighted items.
    pub selection_bg: Color,
}

/// Dark palette — optimized for dark terminal backgrounds (Req 35.4).
pub fn dark_palette() -> Palette {
    Palette {
        fg: Color::White,
        fg_dim: Color::DarkGray,
        bg: None,
        bg_alt: Some(Color::White),
        accent: Color::Cyan,
        accent_dim: Color::DarkGray,
        success: Color::Green,
        warning: Color::Yellow,
        error: Color::Red,
        info: Color::Magenta,
        border: Color::Gray,
        border_focused: Color::Cyan,
        selection_fg: Color::Black,
        selection_bg: Color::LightBlue,
    }
}

/// Light palette — optimized for light terminal backgrounds (Req 35.5).
pub fn light_palette() -> Palette {
    Palette {
        fg: Color::Black,
        fg_dim: Color::DarkGray,
        bg: None,
        bg_alt: Some(Color::Black),
        accent: Color::Blue,
        accent_dim: Color::Gray,
        success: Color::Green,
        warning: Color::Indexed(208), // orange-ish for visibility on light bg
        error: Color::Red,
        info: Color::Magenta,
        border: Color::Gray,
        border_focused: Color::Blue,
        selection_fg: Color::White,
        selection_bg: Color::Blue,
    }
}

/// Build the palette for a resolved [`ThemeMode`].
fn palette_for(mode: ThemeMode) -> Palette {
    match mode {
        ThemeMode::Dark => dark_palette(),
        ThemeMode::Light => light_palette(),
    }
}

// ---------------------------------------------------------------------------
// Theme
// ---------------------------------------------------------------------------

/// Theme configuration supporting `--no-color` mode and adaptive light/dark
/// palettes.
///
/// The theme is deterministic: given the same `(ThemeMode, ColorSupport)`,
/// all style methods return identical `Style` values (Requirement 35.8 /
/// Property 34). `ColorSupport::None` always takes precedence over the mode —
/// no colours are emitted regardless of palette (Req 35.7).
pub struct Theme {
    pub no_color: bool,
    pub color_support: ColorSupport,
    pub mode: ThemeMode,
    palette: Palette,
}

impl Theme {
    /// Create a new theme with automatic colour-support detection and an
    /// explicitly resolved [`ThemeMode`].
    pub fn new(no_color: bool, mode: ThemeMode) -> Self {
        let color_support = detect_color_support(no_color);
        Self {
            no_color,
            color_support,
            mode,
            palette: palette_for(mode),
        }
    }

    /// Create a theme with an explicit colour-support level (useful for
    /// testing). Defaults to [`ThemeMode::Dark`] for backward compatibility.
    pub fn with_color_support(color_support: ColorSupport) -> Self {
        Self::with_color_support_mode(color_support, ThemeMode::Dark)
    }

    /// Create a theme with an explicit colour-support level and mode.
    pub fn with_color_support_mode(color_support: ColorSupport, mode: ThemeMode) -> Self {
        Self {
            no_color: color_support == ColorSupport::None,
            color_support,
            mode,
            palette: palette_for(mode),
        }
    }

    /// Toggle between Dark and Light mode at runtime (Req 35.6), rebuilding
    /// the active palette.
    pub fn toggle_mode(&mut self) {
        self.mode = match self.mode {
            ThemeMode::Dark => ThemeMode::Light,
            ThemeMode::Light => ThemeMode::Dark,
        };
        self.palette = palette_for(self.mode);
    }

    /// Whether any colour output is enabled.
    fn has_color(&self) -> bool {
        self.color_support != ColorSupport::None
    }

    pub fn sidebar_style(&self) -> Style {
        if !self.has_color() {
            Style::default()
        } else {
            Style::default().fg(self.palette.fg)
        }
    }

    pub fn sidebar_selected(&self) -> Style {
        if !self.has_color() {
            Style::default().add_modifier(Modifier::REVERSED)
        } else {
            Style::default()
                .fg(self.palette.selection_fg)
                .bg(self.palette.accent)
                .add_modifier(Modifier::BOLD)
        }
    }

    pub fn list_item(&self) -> Style {
        if !self.has_color() {
            Style::default()
        } else {
            Style::default().fg(self.palette.fg)
        }
    }

    pub fn list_selected(&self) -> Style {
        if !self.has_color() {
            Style::default().add_modifier(Modifier::REVERSED)
        } else {
            Style::default()
                .fg(self.palette.selection_fg)
                .bg(self.palette.selection_bg)
                .add_modifier(Modifier::BOLD)
        }
    }

    pub fn detail_key(&self) -> Style {
        if !self.has_color() {
            Style::default().add_modifier(Modifier::BOLD)
        } else {
            Style::default()
                .fg(self.palette.warning)
                .add_modifier(Modifier::BOLD)
        }
    }

    pub fn detail_value(&self) -> Style {
        if !self.has_color() {
            Style::default()
        } else {
            Style::default().fg(self.palette.fg)
        }
    }

    pub fn status_bar(&self) -> Style {
        if !self.has_color() {
            Style::default().add_modifier(Modifier::REVERSED)
        } else {
            // Inverted bar: dark text on light bar (dark mode), and the
            // reverse in light mode. selection_fg / bg_alt are mirror images
            // across the two palettes, giving correct contrast in both.
            let mut style = Style::default().fg(self.palette.selection_fg);
            if let Some(bg) = self.palette.bg_alt {
                style = style.bg(bg);
            }
            style
        }
    }

    pub fn help_bar(&self) -> Style {
        if !self.has_color() {
            Style::default().add_modifier(Modifier::DIM)
        } else {
            Style::default().fg(self.palette.accent_dim)
        }
    }

    pub fn error_style(&self) -> Style {
        if !self.has_color() {
            Style::default().add_modifier(Modifier::BOLD)
        } else {
            Style::default()
                .fg(self.palette.error)
                .add_modifier(Modifier::BOLD)
        }
    }

    pub fn search_style(&self) -> Style {
        if !self.has_color() {
            Style::default().add_modifier(Modifier::UNDERLINED)
        } else {
            Style::default()
                .fg(self.palette.success)
                .add_modifier(Modifier::BOLD)
        }
    }

    pub fn border_style(&self) -> Style {
        if !self.has_color() {
            Style::default()
        } else {
            Style::default().fg(self.palette.border)
        }
    }

    /// Style for the focused panel border (Requirement 16.6).
    /// Visually distinct from unfocused borders to indicate active panel.
    pub fn border_focused(&self) -> Style {
        if !self.has_color() {
            Style::default().add_modifier(Modifier::BOLD)
        } else {
            Style::default()
                .fg(self.palette.border_focused)
                .add_modifier(Modifier::BOLD)
        }
    }

    /// Style for unfocused panel border (Requirement 16.6).
    /// Subdued compared to focused border.
    pub fn border_unfocused(&self) -> Style {
        if !self.has_color() {
            Style::default().add_modifier(Modifier::DIM)
        } else {
            Style::default().fg(self.palette.accent_dim)
        }
    }

    /// Style for validation gate status: NotRun (dim/gray).
    pub fn gate_not_run(&self) -> Style {
        if !self.has_color() {
            Style::default().add_modifier(Modifier::DIM)
        } else {
            Style::default().fg(self.palette.accent_dim)
        }
    }

    /// Style for validation gate status: Running (warning/yellow).
    pub fn gate_running(&self) -> Style {
        if !self.has_color() {
            Style::default().add_modifier(Modifier::BOLD)
        } else {
            Style::default()
                .fg(self.palette.warning)
                .add_modifier(Modifier::BOLD)
        }
    }

    /// Style for validation gate status: Passed (green).
    pub fn gate_passed(&self) -> Style {
        if !self.has_color() {
            Style::default()
        } else {
            Style::default().fg(self.palette.success)
        }
    }

    /// Style for validation gate status: Failed (red).
    pub fn gate_failed(&self) -> Style {
        if !self.has_color() {
            Style::default().add_modifier(Modifier::BOLD)
        } else {
            Style::default()
                .fg(self.palette.error)
                .add_modifier(Modifier::BOLD)
        }
    }

    /// Style for validation gate status: TimedOut (info/magenta).
    pub fn gate_timed_out(&self) -> Style {
        if !self.has_color() {
            Style::default().add_modifier(Modifier::BOLD)
        } else {
            Style::default()
                .fg(self.palette.info)
                .add_modifier(Modifier::BOLD)
        }
    }

    /// Style for filter chips display.
    pub fn filter_chip(&self) -> Style {
        if !self.has_color() {
            Style::default().add_modifier(Modifier::REVERSED)
        } else {
            Style::default()
                .fg(self.palette.selection_fg)
                .bg(self.palette.accent)
        }
    }

    /// Style for sparkline bar (filled portion).
    pub fn sparkline_filled(&self) -> Style {
        if !self.has_color() {
            Style::default()
        } else {
            Style::default().fg(self.palette.accent)
        }
    }

    /// Style for sparkline bar (empty portion).
    pub fn sparkline_empty(&self) -> Style {
        if !self.has_color() {
            Style::default().add_modifier(Modifier::DIM)
        } else {
            Style::default().fg(self.palette.accent_dim)
        }
    }

    /// Style for help overlay title.
    pub fn help_overlay_title(&self) -> Style {
        if !self.has_color() {
            Style::default().add_modifier(Modifier::BOLD)
        } else {
            Style::default()
                .fg(self.palette.accent)
                .add_modifier(Modifier::BOLD)
        }
    }

    /// Style for help overlay section header.
    pub fn help_overlay_section(&self) -> Style {
        if !self.has_color() {
            Style::default().add_modifier(Modifier::BOLD)
        } else {
            Style::default()
                .fg(self.palette.warning)
                .add_modifier(Modifier::BOLD)
        }
    }

    /// Style for completion suggestion (highlighted).
    pub fn completion_highlight(&self) -> Style {
        if !self.has_color() {
            Style::default().add_modifier(Modifier::REVERSED)
        } else {
            Style::default()
                .fg(self.palette.selection_fg)
                .bg(self.palette.success)
        }
    }

    /// Style for completion suggestion (normal).
    pub fn completion_normal(&self) -> Style {
        if !self.has_color() {
            Style::default()
        } else {
            Style::default().fg(self.palette.fg)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

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
    fn eight_color_mode_has_colors() {
        let theme = Theme::with_color_support(ColorSupport::Basic8);
        assert!(theme.sidebar_style().fg.is_some());
        assert!(theme.list_selected().bg.is_some());
        // Dark palette (default) selection background.
        assert_eq!(theme.list_selected().bg, Some(Color::LightBlue));
    }

    #[test]
    fn focused_border_differs_from_unfocused() {
        let theme = Theme::with_color_support(ColorSupport::TrueColor256);
        let focused = theme.border_focused();
        let unfocused = theme.border_unfocused();
        assert_ne!(focused.fg, unfocused.fg);
    }

    #[test]
    fn no_color_focused_uses_modifiers() {
        let theme = Theme::with_color_support(ColorSupport::None);
        let focused = theme.border_focused();
        let unfocused = theme.border_unfocused();
        assert!(focused.add_modifier.contains(Modifier::BOLD));
        assert!(unfocused.add_modifier.contains(Modifier::DIM));
    }

    #[test]
    fn color_support_detection_no_color_flag() {
        assert_eq!(detect_color_support(true), ColorSupport::None);
    }

    #[test]
    fn theme_deterministic_same_support_level() {
        // Requirement 35.8: same inputs produce same styles.
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
    fn no_color_help_bar_uses_dim() {
        let theme = Theme::with_color_support(ColorSupport::None);
        let style = theme.help_bar();
        assert!(style.add_modifier.contains(Modifier::DIM));
    }

    // -----------------------------------------------------------------------
    // Task 15.4 — theme detection + palette tests (Requirement 35)
    // -----------------------------------------------------------------------

    #[test]
    fn classify_falls_back_to_dark_when_detection_fails() {
        // terminal-light failure (None luma) + no COLORFGBG → Dark (Req 35.2).
        assert_eq!(classify_theme(None, None), ThemeMode::Dark);
    }

    #[test]
    fn classify_uses_luma_threshold() {
        // luma > 0.6 → Light, else Dark (Req 35.1). COLORFGBG ignored when
        // luma is present.
        assert_eq!(classify_theme(Some(0.9), Some("15;0")), ThemeMode::Light);
        assert_eq!(classify_theme(Some(0.61), None), ThemeMode::Light);
        assert_eq!(classify_theme(Some(0.6), None), ThemeMode::Dark);
        assert_eq!(classify_theme(Some(0.1), Some("0;15")), ThemeMode::Dark);
    }

    #[test]
    fn colorfgbg_parsing() {
        // "0;15" → bg 15 (≥ 7) → Light; "15;0" → bg 0 → Dark (Req 35.2).
        assert_eq!(parse_colorfgbg("0;15"), Some(ThemeMode::Light));
        assert_eq!(parse_colorfgbg("15;0"), Some(ThemeMode::Dark));
        // Three-field form `fg;default;bg` — last field is the background.
        assert_eq!(parse_colorfgbg("15;default;0"), Some(ThemeMode::Dark));
        assert_eq!(parse_colorfgbg("0;default;15"), Some(ThemeMode::Light));
        // Boundary: index 7 counts as Light.
        assert_eq!(parse_colorfgbg("0;7"), Some(ThemeMode::Light));
        assert_eq!(parse_colorfgbg("0;6"), Some(ThemeMode::Dark));
        // Missing / unparseable → None (caller falls back).
        assert_eq!(parse_colorfgbg(""), None);
        assert_eq!(parse_colorfgbg("not;a;number"), None);
    }

    #[test]
    fn colorfgbg_used_when_luma_missing() {
        assert_eq!(classify_theme(None, Some("0;15")), ThemeMode::Light);
        assert_eq!(classify_theme(None, Some("15;0")), ThemeMode::Dark);
        // Unparseable COLORFGBG with no luma → Dark fallback.
        assert_eq!(classify_theme(None, Some("garbage")), ThemeMode::Dark);
    }

    #[test]
    fn resolve_theme_explicit_overrides_detection() {
        // Explicit flags ignore detection entirely (Req 35.3).
        assert_eq!(resolve_theme(ThemePreference::Dark, false), ThemeMode::Dark);
        assert_eq!(resolve_theme(ThemePreference::Light, false), ThemeMode::Light);
        assert_eq!(resolve_theme(ThemePreference::Dark, true), ThemeMode::Dark);
        assert_eq!(resolve_theme(ThemePreference::Light, true), ThemeMode::Light);
    }

    #[test]
    fn resolve_theme_auto_headless_defaults_dark() {
        // Headless must not probe the terminal — Auto → Dark (Req 35.9).
        assert_eq!(resolve_theme(ThemePreference::Auto, true), ThemeMode::Dark);
    }

    #[test]
    fn light_mode_uses_light_palette_colors() {
        // Req 35.5: Light mode + colour support returns light palette colours.
        let theme = Theme::with_color_support_mode(ColorSupport::TrueColor256, ThemeMode::Light);
        assert_eq!(theme.mode, ThemeMode::Light);
        assert_eq!(theme.sidebar_style().fg, Some(Color::Black));
        assert_eq!(theme.border_focused().fg, Some(Color::Blue));
        assert_eq!(theme.list_selected().bg, Some(Color::Blue));
    }

    #[test]
    fn dark_mode_uses_dark_palette_colors() {
        // Req 35.4: Dark mode + colour support returns dark palette colours.
        let theme = Theme::with_color_support_mode(ColorSupport::TrueColor256, ThemeMode::Dark);
        assert_eq!(theme.mode, ThemeMode::Dark);
        assert_eq!(theme.sidebar_style().fg, Some(Color::White));
        assert_eq!(theme.border_focused().fg, Some(Color::Cyan));
        assert_eq!(theme.list_selected().bg, Some(Color::LightBlue));
    }

    #[test]
    fn no_color_ignores_mode() {
        // Req 35.7: ColorSupport::None emits no colours regardless of mode.
        for mode in [ThemeMode::Dark, ThemeMode::Light] {
            let theme = Theme::with_color_support_mode(ColorSupport::None, mode);
            assert_eq!(theme.sidebar_style().fg, None);
            assert_eq!(theme.list_selected().fg, None);
            assert_eq!(theme.list_selected().bg, None);
            assert_eq!(theme.border_focused().fg, None);
            assert_eq!(theme.gate_passed().fg, None);
        }
    }

    #[test]
    fn runtime_toggle_flips_mode_and_styles() {
        // Req 35.6: toggling flips the mode and produces different output.
        let mut theme = Theme::with_color_support_mode(ColorSupport::TrueColor256, ThemeMode::Dark);
        let dark_fg = theme.sidebar_style().fg;
        let dark_border = theme.border_focused().fg;
        theme.toggle_mode();
        assert_eq!(theme.mode, ThemeMode::Light);
        assert_ne!(theme.sidebar_style().fg, dark_fg);
        assert_ne!(theme.border_focused().fg, dark_border);
        // Toggling back restores the original.
        theme.toggle_mode();
        assert_eq!(theme.mode, ThemeMode::Dark);
        assert_eq!(theme.sidebar_style().fg, dark_fg);
        assert_eq!(theme.border_focused().fg, dark_border);
    }

    fn all_styles(theme: &Theme) -> Vec<Style> {
        vec![
            theme.sidebar_style(),
            theme.sidebar_selected(),
            theme.list_item(),
            theme.list_selected(),
            theme.detail_key(),
            theme.detail_value(),
            theme.status_bar(),
            theme.help_bar(),
            theme.error_style(),
            theme.search_style(),
            theme.border_style(),
            theme.border_focused(),
            theme.border_unfocused(),
            theme.gate_not_run(),
            theme.gate_running(),
            theme.gate_passed(),
            theme.gate_failed(),
            theme.gate_timed_out(),
            theme.filter_chip(),
            theme.sparkline_filled(),
            theme.sparkline_empty(),
            theme.help_overlay_title(),
            theme.help_overlay_section(),
            theme.completion_highlight(),
            theme.completion_normal(),
        ]
    }

    fn mode_strategy() -> impl Strategy<Value = ThemeMode> {
        prop_oneof![Just(ThemeMode::Dark), Just(ThemeMode::Light)]
    }

    fn support_strategy() -> impl Strategy<Value = ColorSupport> {
        prop_oneof![
            Just(ColorSupport::TrueColor256),
            Just(ColorSupport::Basic8),
            Just(ColorSupport::None),
        ]
    }

    proptest! {
        /// Property 34: for any (ThemeMode, ColorSupport), every style method
        /// returns a valid (non-panicking) Style, and two themes built from the
        /// same inputs produce bit-identical output (Req 35.8).
        #[test]
        fn prop34_theme_styles_deterministic(
            mode in mode_strategy(),
            support in support_strategy(),
        ) {
            let a = Theme::with_color_support_mode(support, mode);
            let b = Theme::with_color_support_mode(support, mode);
            // Constructing + calling every method must not panic, and must match.
            prop_assert_eq!(all_styles(&a), all_styles(&b));
            // None support never emits a foreground/background colour.
            if support == ColorSupport::None {
                for style in all_styles(&a) {
                    prop_assert_eq!(style.fg, None);
                    prop_assert_eq!(style.bg, None);
                }
            }
        }
    }
}
