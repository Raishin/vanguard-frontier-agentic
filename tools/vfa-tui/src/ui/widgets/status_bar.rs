use ratatui::{
    layout::Rect,
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

use super::super::theme::Theme;

/// Render the status bar showing item counts, active filters, and session info.
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
