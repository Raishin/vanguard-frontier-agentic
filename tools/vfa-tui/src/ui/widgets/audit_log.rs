/// Audit log viewer widget — scrollable view of audit log entries.
///
/// Displays: timestamp, event_type, subject, details (truncated).
/// Supports a viewport offset for scrolling through large audit logs.
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget},
    Frame,
};

use crate::models::audit::{AuditEntry, AuditEventType};
use crate::ui::theme::Theme;

/// Text label for each audit event type (color-independent).
pub fn event_type_label(event_type: &AuditEventType) -> &'static str {
    match event_type {
        AuditEventType::PolicyEvaluation => "[EVAL]",
        AuditEventType::Promotion => "[PROMO]",
        AuditEventType::InstallationDetected => "[INST]",
        AuditEventType::DriftDetected => "[DRIFT]",
        AuditEventType::ViolationResolved => "[RESOL]",
        AuditEventType::OperatorAction => "[OPS]",
        AuditEventType::GateExecution => "[GATE]",
        AuditEventType::ConfigChange => "[CFG]",
    }
}

/// State for the audit log viewport (scroll offset).
#[derive(Debug, Default, Clone)]
pub struct AuditLogState {
    /// First visible entry index.
    pub row_offset: usize,
}

impl AuditLogState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn scroll_down(&mut self, n: usize) {
        self.row_offset = self.row_offset.saturating_add(n);
    }

    pub fn scroll_up(&mut self, n: usize) {
        self.row_offset = self.row_offset.saturating_sub(n);
    }

    /// Clamp offset so it cannot exceed the number of entries.
    pub fn clamp(&mut self, entry_count: usize) {
        if entry_count == 0 {
            self.row_offset = 0;
        } else if self.row_offset >= entry_count {
            self.row_offset = entry_count - 1;
        }
    }
}

/// Maximum length of the `details` column shown inline.
const DETAILS_MAX: usize = 40;

fn truncate(s: &str, max: usize) -> String {
    if s.len() <= max {
        s.to_string()
    } else {
        format!("{}…", &s[..max.saturating_sub(1)])
    }
}

/// Build the rendered lines for the audit log viewer.
pub fn build_audit_lines<'a>(
    entries: &'a [AuditEntry],
    state: &AuditLogState,
    theme: &Theme,
) -> Vec<Line<'a>> {
    let mut lines: Vec<Line<'a>> = Vec::new();

    // ── header ──────────────────────────────────────────────────────────────
    lines.push(Line::from(vec![Span::styled(
        format!(
            "{:<24} {:<8} {:<30} {}",
            "Timestamp", "Type", "Subject", "Details"
        ),
        Style::default().add_modifier(Modifier::BOLD),
    )]));
    lines.push(Line::from(vec![Span::styled(
        "─".repeat(100),
        Style::default().add_modifier(Modifier::DIM),
    )]));

    if entries.is_empty() {
        lines.push(Line::from(vec![Span::raw("  (no audit log entries)")]));
        return lines.into_iter().skip(state.row_offset).collect();
    }

    for entry in entries.iter().skip(state.row_offset) {
        let ts = truncate(&entry.timestamp, 23);
        let type_label = event_type_label(&entry.event_type);
        let subject = truncate(&entry.subject, 29);
        let details_str = match &entry.details {
            serde_json::Value::String(s) => truncate(s, DETAILS_MAX),
            serde_json::Value::Object(m) => {
                // Show first key=value pair
                let first = m.iter().next().map(|(k, v)| {
                    let vs = match v {
                        serde_json::Value::String(s) => s.clone(),
                        other => other.to_string(),
                    };
                    format!("{k}={vs}")
                });
                truncate(first.as_deref().unwrap_or("{}"), DETAILS_MAX)
            }
            other => truncate(&other.to_string(), DETAILS_MAX),
        };

        let operator_hint = if entry.operator != "system" {
            format!(" [op:{}]", entry.operator)
        } else {
            String::new()
        };

        lines.push(Line::from(vec![
            Span::raw(format!("{ts:<24}")),
            Span::styled(
                format!("{type_label:<8}"),
                Style::default().add_modifier(Modifier::BOLD),
            ),
            Span::raw(format!("{subject:<30}")),
            Span::raw(details_str),
            Span::styled(
                operator_hint,
                Style::default().add_modifier(Modifier::DIM),
            ),
        ]));
    }

    lines
}

/// Render the audit log viewer into a [`Frame`].
pub fn render_audit_log(
    entries: &[AuditEntry],
    state: &AuditLogState,
    area: Rect,
    frame: &mut Frame,
    theme: &Theme,
) {
    let lines = build_audit_lines(entries, state, theme);
    let paragraph = Paragraph::new(lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!("Audit Log ({} entries)", entries.len()))
                .border_style(theme.border_style()),
        )
        .scroll((0, 0));
    frame.render_widget(paragraph, area);
}

/// Render the audit log viewer into a [`Buffer`] (useful for testing).
pub fn render_audit_log_buffer(
    entries: &[AuditEntry],
    state: &AuditLogState,
    area: Rect,
    buf: &mut Buffer,
    theme: &Theme,
) {
    let lines = build_audit_lines(entries, state, theme);
    let paragraph = Paragraph::new(lines).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Audit Log")
            .border_style(theme.border_style()),
    );
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
    use crate::models::audit::{AuditEntry, AuditEventType};
    use crate::ui::theme::{ColorSupport, Theme};
    use ratatui::backend::TestBackend;
    use ratatui::buffer::Buffer;
    use ratatui::layout::Rect;
    use ratatui::Terminal;

    fn make_entry(id: i64, event_type: AuditEventType, subject: &str) -> AuditEntry {
        AuditEntry {
            id,
            timestamp: format!("2025-01-0{}T00:00:00.000Z", id),
            event_type,
            subject: subject.to_string(),
            details: serde_json::json!({"rule": "require-scanner"}),
            operator: "system".to_string(),
            entry_hash: "abc123".to_string(),
            prev_hash: "".to_string(),
        }
    }

    #[test]
    fn audit_log_shows_timestamp() {
        let entries = vec![make_entry(1, AuditEventType::PolicyEvaluation, "prod")];
        let state = AuditLogState::new();
        let theme = Theme::with_color_support(ColorSupport::None);
        let area = Rect::new(0, 0, 120, 10);
        let mut buf = Buffer::empty(area);
        render_audit_log_buffer(&entries, &state, area, &mut buf, &theme);

        let content = buf_content(&buf);
        assert!(content.contains("2025-01-01"), "timestamp should appear");
    }

    #[test]
    fn audit_log_shows_event_type_label() {
        let entries = vec![make_entry(1, AuditEventType::GateExecution, "validate:lint")];
        let state = AuditLogState::new();
        let theme = Theme::with_color_support(ColorSupport::None);
        let area = Rect::new(0, 0, 120, 10);
        let mut buf = Buffer::empty(area);
        render_audit_log_buffer(&entries, &state, area, &mut buf, &theme);

        let content = buf_content(&buf);
        assert!(content.contains("[GATE]"), "event type label [GATE] should appear");
    }

    #[test]
    fn audit_log_shows_subject() {
        let entries = vec![make_entry(1, AuditEventType::DriftDetected, "aws-iam-scanner")];
        let state = AuditLogState::new();
        let theme = Theme::with_color_support(ColorSupport::None);
        let area = Rect::new(0, 0, 120, 10);
        let mut buf = Buffer::empty(area);
        render_audit_log_buffer(&entries, &state, area, &mut buf, &theme);

        let content = buf_content(&buf);
        assert!(content.contains("aws-iam-scanner"), "subject should appear");
    }

    #[test]
    fn audit_log_empty_shows_no_entries_message() {
        let entries: Vec<AuditEntry> = vec![];
        let state = AuditLogState::new();
        let theme = Theme::with_color_support(ColorSupport::None);
        let area = Rect::new(0, 0, 80, 10);
        let mut buf = Buffer::empty(area);
        render_audit_log_buffer(&entries, &state, area, &mut buf, &theme);

        let content = buf_content(&buf);
        assert!(content.contains("no audit log"), "empty log should show hint");
    }

    #[test]
    fn audit_log_scroll_hides_first_entry() {
        let entries = vec![
            make_entry(1, AuditEventType::PolicyEvaluation, "first-subject"),
            make_entry(2, AuditEventType::GateExecution, "second-subject"),
        ];
        let mut state = AuditLogState::new();
        let theme = Theme::with_color_support(ColorSupport::None);
        let area = Rect::new(0, 0, 120, 10);

        // No offset — both visible
        let mut buf = Buffer::empty(area);
        render_audit_log_buffer(&entries, &state, area, &mut buf, &theme);
        let content = buf_content(&buf);
        assert!(content.contains("first-subject"), "first entry should be visible initially");

        // Scroll past the header + first entry
        state.scroll_down(3);
        let mut buf2 = Buffer::empty(area);
        render_audit_log_buffer(&entries, &state, area, &mut buf2, &theme);
        let content2 = buf_content(&buf2);
        // After scrolling 3 lines: header + separator line + first entry are gone
        assert!(
            !content2.contains("first-subject"),
            "after scroll, first entry should not be visible"
        );
    }

    #[test]
    fn event_type_labels_all_nonempty() {
        for et in [
            AuditEventType::PolicyEvaluation,
            AuditEventType::Promotion,
            AuditEventType::InstallationDetected,
            AuditEventType::DriftDetected,
            AuditEventType::ViolationResolved,
            AuditEventType::OperatorAction,
            AuditEventType::GateExecution,
            AuditEventType::ConfigChange,
        ] {
            assert!(
                !event_type_label(&et).trim().is_empty(),
                "label for {et:?} must be non-empty"
            );
        }
    }

    #[test]
    fn audit_log_via_test_backend() {
        let entries = vec![make_entry(1, AuditEventType::PolicyEvaluation, "prod")];
        let state = AuditLogState::new();
        let theme = Theme::with_color_support(ColorSupport::None);

        let backend = TestBackend::new(120, 10);
        let mut terminal = Terminal::new(backend).unwrap();
        terminal
            .draw(|frame| {
                render_audit_log(&entries, &state, frame.area(), frame, &theme);
            })
            .unwrap();

        let buf = terminal.backend().buffer().clone();
        let content = buf_content(&buf);
        assert!(content.contains("[EVAL]"));
    }
}
