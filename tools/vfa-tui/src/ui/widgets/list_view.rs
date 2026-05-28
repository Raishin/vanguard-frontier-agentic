use ratatui::{
    layout::Rect,
    style::Modifier,
    widgets::{Block, Borders, List, ListItem, ListState},
    Frame,
};

use super::super::theme::Theme;

/// Render a scrollable list with highlighted selection.
/// Navigation stops at boundaries (no wrap).
pub fn render_list_view(
    items: &[String],
    state: &mut ListState,
    title: &str,
    area: Rect,
    frame: &mut Frame,
    theme: &Theme,
) {
    let list_items: Vec<ListItem> = items
        .iter()
        .map(|item| ListItem::new(item.as_str()).style(theme.list_item()))
        .collect();

    let list = List::new(list_items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(title.to_string())
                .border_style(theme.border_style()),
        )
        .highlight_style(theme.list_selected().add_modifier(Modifier::BOLD))
        .highlight_symbol("> ");

    frame.render_stateful_widget(list, area, state);
}
