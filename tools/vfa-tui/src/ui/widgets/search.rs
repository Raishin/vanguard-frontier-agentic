use ratatui::{
    layout::Rect,
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use super::super::theme::Theme;

/// Render the search input widget.
pub fn render_search_input(
    query: &str,
    active: bool,
    area: Rect,
    frame: &mut Frame,
    theme: &Theme,
) {
    let display = if active {
        format!("/{query}_")
    } else if query.is_empty() {
        String::new()
    } else {
        format!("/{query}")
    };

    let style = if active {
        theme.search_style()
    } else {
        theme.detail_value()
    };

    let line = Line::from(vec![Span::styled(display, style)]);
    let paragraph = Paragraph::new(line).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Search")
            .border_style(theme.border_style()),
    );

    frame.render_widget(paragraph, area);
}
