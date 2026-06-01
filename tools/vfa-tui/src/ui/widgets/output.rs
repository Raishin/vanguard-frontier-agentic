use ratatui::{
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

use crate::subprocess::stream::OutputStream;

use super::super::theme::Theme;

/// A line of subprocess output with stream identification.
#[derive(Debug, Clone)]
pub struct OutputLine {
    pub content: String,
    pub stream: OutputStream,
}

/// Render subprocess output panel showing stdout/stderr lines with differentiation.
/// Supports scrolling via the `scroll` parameter (vertical offset in lines).
pub fn render_output(
    lines: &[OutputLine],
    title: &str,
    area: Rect,
    frame: &mut Frame,
    theme: &Theme,
) {
    render_output_scrolled(lines, title, area, frame, 0, theme);
}

/// Render subprocess output panel with explicit scroll offset.
/// `scroll` is the number of lines to skip from the top.
pub fn render_output_scrolled(
    lines: &[OutputLine],
    title: &str,
    area: Rect,
    frame: &mut Frame,
    scroll: u16,
    theme: &Theme,
) {
    let display_lines: Vec<Line> = lines
        .iter()
        .map(|line| {
            let style = if theme.no_color {
                Style::default()
            } else {
                match line.stream {
                    OutputStream::Stdout => Style::default().fg(Color::White),
                    OutputStream::Stderr => Style::default().fg(Color::Red),
                }
            };
            Line::from(vec![Span::styled(line.content.clone(), style)])
        })
        .collect();

    let paragraph = Paragraph::new(display_lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(title.to_string())
                .border_style(theme.border_style()),
        )
        .wrap(Wrap { trim: false })
        .scroll((scroll, 0));

    frame.render_widget(paragraph, area);
}
