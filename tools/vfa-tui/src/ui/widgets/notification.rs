/// Toast notification widget.
///
/// Renders a single `Notification` with severity styling and TTL-based
/// visibility.  The widget is a no-op when the notification is expired.
///
/// Req 29.2: text severity prefixes ([INFO]/[WARN]/[ERR]/[OK]) ensure
///           readability without color.
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph, Widget},
    Frame,
};

use crate::models::notification::{Notification, NotificationSeverity};
use crate::ui::theme::Theme;

/// Text prefix for each notification severity (color-independent, Req 29.2).
pub fn notification_prefix(severity: &NotificationSeverity) -> &'static str {
    match severity {
        NotificationSeverity::Info => "[INFO]",
        NotificationSeverity::Warning => "[WARN]",
        NotificationSeverity::Error => "[ERR] ",
        NotificationSeverity::Success => "[OK]  ",
    }
}

fn notification_style(severity: &NotificationSeverity, theme: &Theme) -> Style {
    if theme.no_color {
        return match severity {
            NotificationSeverity::Error => Style::default().add_modifier(Modifier::BOLD),
            NotificationSeverity::Warning => Style::default().add_modifier(Modifier::BOLD),
            NotificationSeverity::Success => Style::default(),
            NotificationSeverity::Info => Style::default().add_modifier(Modifier::DIM),
        };
    }
    match severity {
        NotificationSeverity::Info => Style::default().fg(Color::Cyan),
        NotificationSeverity::Warning => Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD),
        NotificationSeverity::Error => Style::default()
            .fg(Color::Red)
            .add_modifier(Modifier::BOLD),
        NotificationSeverity::Success => Style::default().fg(Color::Green),
    }
}

fn border_style_for(severity: &NotificationSeverity, theme: &Theme) -> Style {
    notification_style(severity, theme)
}

/// Render a toast notification into a [`Frame`].
///
/// If the notification is expired, nothing is drawn.
/// `area` defines where the toast appears — typically a small floating rect
/// near the top-right of the screen.
pub fn render_notification(
    notification: &Notification,
    area: Rect,
    frame: &mut Frame,
    theme: &Theme,
) {
    if notification.is_expired() {
        return;
    }

    let prefix = notification_prefix(&notification.severity);
    let style = notification_style(&notification.severity, theme);
    let border_style = border_style_for(&notification.severity, theme);

    let ttl_remaining = notification
        .ttl
        .checked_sub(notification.created_at.elapsed())
        .unwrap_or_default();
    let ttl_hint = format!(" ({}s)", ttl_remaining.as_secs());

    let line = Line::from(vec![
        Span::styled(prefix.to_string(), style),
        Span::raw(" "),
        Span::raw(notification.message.clone()),
        Span::styled(ttl_hint, Style::default().add_modifier(Modifier::DIM)),
    ]);

    // Clear the background area first (so it floats over content below)
    frame.render_widget(Clear, area);
    let paragraph = Paragraph::new(line)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(border_style),
        )
        .style(Style::default());
    frame.render_widget(paragraph, area);
}

/// Render a notification into a [`Buffer`] (useful for testing).
///
/// Returns `false` if the notification was expired (nothing drawn).
pub fn render_notification_buffer(
    notification: &Notification,
    area: Rect,
    buf: &mut Buffer,
    theme: &Theme,
) -> bool {
    if notification.is_expired() {
        return false;
    }

    let prefix = notification_prefix(&notification.severity);
    let style = notification_style(&notification.severity, theme);

    let line = Line::from(vec![
        Span::styled(prefix.to_string(), style),
        Span::raw(" "),
        Span::raw(notification.message.clone()),
    ]);

    let paragraph = Paragraph::new(line).block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(border_style_for(&notification.severity, theme)),
    );
    Widget::render(paragraph, area, buf);
    true
}

/// Render a list of active (non-expired) notifications stacked vertically.
///
/// Notifications are rendered newest-first (last in list = topmost toast).
/// Each toast occupies 3 rows (1 border top + 1 content + 1 border bottom).
pub fn render_notification_stack(
    notifications: &[Notification],
    base_area: Rect,
    frame: &mut Frame,
    theme: &Theme,
) {
    let toast_height: u16 = 3;
    let active: Vec<&Notification> = notifications
        .iter()
        .filter(|n| !n.is_expired())
        .collect();

    for (i, n) in active.iter().rev().enumerate() {
        let y_offset = i as u16 * toast_height;
        if y_offset + toast_height > base_area.height {
            break;
        }
        let toast_area = Rect::new(
            base_area.x,
            base_area.y + y_offset,
            base_area.width,
            toast_height,
        );
        render_notification(n, toast_area, frame, theme);
    }
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
    use crate::models::notification::{Notification, NotificationSeverity};
    use crate::ui::theme::{ColorSupport, Theme};
    use ratatui::buffer::Buffer;
    use ratatui::layout::Rect;
    use std::time::Duration;

    fn make_notification(severity: NotificationSeverity, ttl_secs: u64) -> Notification {
        Notification::new(
            "Test notification message".to_string(),
            severity,
            Duration::from_secs(ttl_secs),
        )
    }

    #[test]
    fn notification_renders_info_prefix() {
        let n = make_notification(NotificationSeverity::Info, 10);
        let theme = Theme::with_color_support(ColorSupport::None);
        let area = Rect::new(0, 0, 60, 3);
        let mut buf = Buffer::empty(area);
        let rendered = render_notification_buffer(&n, area, &mut buf, &theme);
        assert!(rendered, "non-expired notification should render");

        let content = buf_content(&buf);
        assert!(content.contains("[INFO]"), "expected [INFO] prefix");
    }

    #[test]
    fn notification_renders_warn_prefix() {
        let n = make_notification(NotificationSeverity::Warning, 10);
        let theme = Theme::with_color_support(ColorSupport::None);
        let area = Rect::new(0, 0, 60, 3);
        let mut buf = Buffer::empty(area);
        render_notification_buffer(&n, area, &mut buf, &theme);

        let content = buf_content(&buf);
        assert!(content.contains("[WARN]"), "expected [WARN] prefix");
    }

    #[test]
    fn notification_renders_error_prefix() {
        let n = make_notification(NotificationSeverity::Error, 10);
        let theme = Theme::with_color_support(ColorSupport::None);
        let area = Rect::new(0, 0, 60, 3);
        let mut buf = Buffer::empty(area);
        render_notification_buffer(&n, area, &mut buf, &theme);

        let content = buf_content(&buf);
        assert!(content.contains("[ERR]"), "expected [ERR] prefix");
    }

    #[test]
    fn notification_renders_success_prefix() {
        let n = make_notification(NotificationSeverity::Success, 10);
        let theme = Theme::with_color_support(ColorSupport::None);
        let area = Rect::new(0, 0, 60, 3);
        let mut buf = Buffer::empty(area);
        render_notification_buffer(&n, area, &mut buf, &theme);

        let content = buf_content(&buf);
        assert!(content.contains("[OK]"), "expected [OK] prefix");
    }

    #[test]
    fn notification_shows_message_text() {
        let n = Notification::new(
            "Catalog reloaded successfully".to_string(),
            NotificationSeverity::Success,
            Duration::from_secs(5),
        );
        let theme = Theme::with_color_support(ColorSupport::None);
        let area = Rect::new(0, 0, 80, 3);
        let mut buf = Buffer::empty(area);
        render_notification_buffer(&n, area, &mut buf, &theme);

        let content = buf_content(&buf);
        assert!(
            content.contains("Catalog reloaded"),
            "notification message should appear"
        );
    }

    #[test]
    fn expired_notification_renders_nothing() {
        // Zero-TTL notification is expired immediately (elapsed >= 0 == TTL).
        let n = Notification::new(
            "Gone".to_string(),
            NotificationSeverity::Info,
            Duration::ZERO,
        );
        assert!(n.is_expired(), "zero-TTL notification must be expired");

        let theme = Theme::with_color_support(ColorSupport::None);
        let area = Rect::new(0, 0, 60, 3);
        let mut buf = Buffer::empty(area);
        let rendered = render_notification_buffer(&n, area, &mut buf, &theme);
        assert!(!rendered, "expired notification must not render");

        // Buffer should be entirely blank
        let content = buf_content(&buf);
        assert!(
            content.chars().all(|c| c == ' '),
            "expired notification must leave buffer untouched"
        );
    }

    #[test]
    fn notification_prefixes_all_nonempty() {
        for sev in [
            NotificationSeverity::Info,
            NotificationSeverity::Warning,
            NotificationSeverity::Error,
            NotificationSeverity::Success,
        ] {
            assert!(
                !notification_prefix(&sev).trim().is_empty(),
                "prefix for {sev:?} must be non-empty"
            );
        }
    }
}
