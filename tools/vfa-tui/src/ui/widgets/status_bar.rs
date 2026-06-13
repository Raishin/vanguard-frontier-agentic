use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Paragraph, Widget},
    Frame,
};

use super::super::theme::Theme;

/// Render the status bar showing item counts, active filters, and session info.
///
/// **v1 API** — kept intact so existing `app.rs` consumers are unaffected.
pub fn render_status_bar(
    visible: usize,
    total: usize,
    filters: &str,
    session_id: &str,
    area: Rect,
    frame: &mut Frame,
    theme: &Theme,
) {
    let filter_part = if filters.is_empty() {
        String::new()
    } else {
        format!(" | Filter: {filters}")
    };

    let short_session = if session_id.len() > 8 {
        &session_id[..8]
    } else {
        session_id
    };

    let content = format!(" {visible}/{total} items{filter_part} | Session: {short_session}");

    let line = Line::from(vec![Span::styled(content, theme.status_bar())]);
    let paragraph = Paragraph::new(line).style(theme.status_bar());
    frame.render_widget(paragraph, area);
}

// ── v2 status bar ─────────────────────────────────────────────────────────────

/// Aggregated platform metrics displayed in the v2 status line.
///
/// Req 16.6: show active workspace count, total assets, aggregate compliance
/// score, and active warnings.
#[derive(Debug, Default, Clone)]
pub struct StatusBarV2 {
    /// Number of workspaces currently active / reachable.
    pub active_workspaces: usize,
    /// Total number of catalog assets loaded.
    pub total_assets: usize,
    /// Aggregate compliance score across all workspaces (0.0–100.0).
    /// `None` when no workspaces have been scanned yet.
    pub aggregate_compliance: Option<f64>,
    /// Number of active (unresolved) warnings / violations.
    pub active_warnings: usize,
}

impl StatusBarV2 {
    pub fn new() -> Self {
        Self::default()
    }
}

/// Build the v2 status line as a `Vec<Span>`.
///
/// Exposed separately so it can be embedded inside a larger status layout.
pub fn build_status_bar_v2_spans(status: &StatusBarV2, theme: &Theme) -> Vec<Span<'static>> {
    let ws_str = format!(" WS:{}", status.active_workspaces);
    let assets_str = format!(" | Assets:{}", status.total_assets);

    let compliance_str = match status.aggregate_compliance {
        Some(score) => format!(" | Compliance:{:.1}%", score),
        None => " | Compliance:N/A".to_string(),
    };

    let warn_str = format!(" | Warnings:{} ", status.active_warnings);

    // Warning count gets special styling when non-zero
    let warn_style = if status.active_warnings > 0 {
        if theme.no_color {
            Style::default().add_modifier(Modifier::BOLD)
        } else {
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD)
        }
    } else {
        theme.status_bar()
    };

    // Compliance score gets color-coded: green ≥80, yellow ≥50, red <50
    let compliance_style = match status.aggregate_compliance {
        Some(score) if score >= 80.0 => {
            if theme.no_color {
                theme.status_bar()
            } else {
                Style::default().fg(Color::Green).bg(
                    theme
                        .status_bar()
                        .bg
                        .unwrap_or(Color::White),
                )
            }
        }
        Some(score) if score >= 50.0 => {
            if theme.no_color {
                theme.status_bar()
            } else {
                Style::default().fg(Color::Yellow).bg(
                    theme
                        .status_bar()
                        .bg
                        .unwrap_or(Color::White),
                )
            }
        }
        Some(_) => {
            if theme.no_color {
                Style::default().add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::Red).bg(
                    theme
                        .status_bar()
                        .bg
                        .unwrap_or(Color::White),
                )
            }
        }
        None => theme.status_bar(),
    };

    vec![
        Span::styled(ws_str, theme.status_bar()),
        Span::styled(assets_str, theme.status_bar()),
        Span::styled(compliance_str, compliance_style),
        Span::styled(warn_str, warn_style),
    ]
}

/// Render the v2 status bar into a [`Frame`].
///
/// Req 16.6: active workspace count, total assets, aggregate compliance score,
/// active warnings.
pub fn render_status_bar_v2(
    status: &StatusBarV2,
    area: Rect,
    frame: &mut Frame,
    theme: &Theme,
) {
    let spans = build_status_bar_v2_spans(status, theme);
    let line = Line::from(spans);
    let paragraph = Paragraph::new(line).style(theme.status_bar());
    frame.render_widget(paragraph, area);
}

/// Render the v2 status bar into a [`Buffer`] (useful for testing).
pub fn render_status_bar_v2_buffer(
    status: &StatusBarV2,
    area: Rect,
    buf: &mut Buffer,
    theme: &Theme,
) {
    let spans = build_status_bar_v2_spans(status, theme);
    let line = Line::from(spans);
    let paragraph = Paragraph::new(line).style(theme.status_bar());
    Widget::render(paragraph, area, buf);
}

#[cfg(test)]
fn buf_content(buf: &Buffer) -> String {
    buf.content
        .iter()
        .map(|c| c.symbol().chars().next().unwrap_or(' '))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ui::theme::{ColorSupport, Theme};
    use ratatui::backend::TestBackend;
    use ratatui::buffer::Buffer;
    use ratatui::layout::Rect;
    use ratatui::Terminal;

    // ── v1 status bar tests (kept) ────────────────────────────────────────────

    #[test]
    fn v1_status_bar_renders_counts() {
        let theme = Theme::with_color_support(ColorSupport::None);
        let backend = TestBackend::new(60, 1);
        let mut terminal = Terminal::new(backend).unwrap();
        terminal
            .draw(|frame| {
                render_status_bar(5, 20, "", "abc12345xyz", frame.area(), frame, &theme);
            })
            .unwrap();
        let buf = terminal.backend().buffer().clone();
        let content = buf_content(&buf);
        assert!(content.contains("5/20"), "v1 bar should show visible/total");
    }

    #[test]
    fn v1_status_bar_shows_filter() {
        let theme = Theme::with_color_support(ColorSupport::None);
        let backend = TestBackend::new(80, 1);
        let mut terminal = Terminal::new(backend).unwrap();
        terminal
            .draw(|frame| {
                render_status_bar(3, 10, "aws", "session1", frame.area(), frame, &theme);
            })
            .unwrap();
        let buf = terminal.backend().buffer().clone();
        let content = buf_content(&buf);
        assert!(content.contains("Filter: aws"), "v1 bar should show filter");
    }

    // ── v2 status bar tests ────────────────────────────────────────────────────

    #[test]
    fn v2_status_bar_shows_workspace_count() {
        let status = StatusBarV2 {
            active_workspaces: 7,
            total_assets: 42,
            aggregate_compliance: Some(88.5),
            active_warnings: 2,
        };
        let theme = Theme::with_color_support(ColorSupport::None);
        let area = Rect::new(0, 0, 100, 1);
        let mut buf = Buffer::empty(area);
        render_status_bar_v2_buffer(&status, area, &mut buf, &theme);

        let content = buf_content(&buf);
        assert!(content.contains("WS:7"), "v2 bar should show workspace count");
    }

    #[test]
    fn v2_status_bar_shows_total_assets() {
        let status = StatusBarV2 {
            active_workspaces: 3,
            total_assets: 150,
            aggregate_compliance: Some(75.0),
            active_warnings: 0,
        };
        let theme = Theme::with_color_support(ColorSupport::None);
        let area = Rect::new(0, 0, 100, 1);
        let mut buf = Buffer::empty(area);
        render_status_bar_v2_buffer(&status, area, &mut buf, &theme);

        let content = buf_content(&buf);
        assert!(content.contains("Assets:150"), "v2 bar should show total assets");
    }

    #[test]
    fn v2_status_bar_shows_compliance_score() {
        let status = StatusBarV2 {
            active_workspaces: 5,
            total_assets: 30,
            aggregate_compliance: Some(92.3),
            active_warnings: 1,
        };
        let theme = Theme::with_color_support(ColorSupport::None);
        let area = Rect::new(0, 0, 100, 1);
        let mut buf = Buffer::empty(area);
        render_status_bar_v2_buffer(&status, area, &mut buf, &theme);

        let content = buf_content(&buf);
        assert!(content.contains("Compliance:92.3%"), "v2 bar should show compliance score");
    }

    #[test]
    fn v2_status_bar_shows_na_compliance_when_none() {
        let status = StatusBarV2 {
            active_workspaces: 0,
            total_assets: 0,
            aggregate_compliance: None,
            active_warnings: 0,
        };
        let theme = Theme::with_color_support(ColorSupport::None);
        let area = Rect::new(0, 0, 100, 1);
        let mut buf = Buffer::empty(area);
        render_status_bar_v2_buffer(&status, area, &mut buf, &theme);

        let content = buf_content(&buf);
        assert!(content.contains("Compliance:N/A"), "v2 bar should show N/A when no data");
    }

    #[test]
    fn v2_status_bar_shows_active_warnings() {
        let status = StatusBarV2 {
            active_workspaces: 2,
            total_assets: 20,
            aggregate_compliance: Some(60.0),
            active_warnings: 5,
        };
        let theme = Theme::with_color_support(ColorSupport::None);
        let area = Rect::new(0, 0, 100, 1);
        let mut buf = Buffer::empty(area);
        render_status_bar_v2_buffer(&status, area, &mut buf, &theme);

        let content = buf_content(&buf);
        assert!(content.contains("Warnings:5"), "v2 bar should show active warnings count");
    }

    #[test]
    fn v2_status_bar_via_test_backend() {
        let status = StatusBarV2 {
            active_workspaces: 4,
            total_assets: 80,
            aggregate_compliance: Some(77.5),
            active_warnings: 3,
        };
        let theme = Theme::with_color_support(ColorSupport::None);

        let backend = TestBackend::new(100, 1);
        let mut terminal = Terminal::new(backend).unwrap();
        terminal
            .draw(|frame| {
                render_status_bar_v2(&status, frame.area(), frame, &theme);
            })
            .unwrap();

        let buf = terminal.backend().buffer().clone();
        let content = buf_content(&buf);
        assert!(content.contains("WS:4"));
        assert!(content.contains("Assets:80"));
        assert!(content.contains("Compliance:77.5%"));
        assert!(content.contains("Warnings:3"));
    }
}
