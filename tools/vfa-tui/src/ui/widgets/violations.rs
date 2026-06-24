/// Violations dashboard widget.
///
/// Req 15.1–15.4: violations grouped by Severity (Critical→Warning→Info)
///                then by workspace, ranked by compliance score ascending
///                (worst first).
/// Req 29.2: text severity prefixes [CRIT]/[WARN]/[INFO] ensure readability
///           without color.
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget},
    Frame,
};

use crate::models::policy::{PolicyViolation, Severity};
use crate::ui::theme::Theme;

/// Text prefix for each severity level (color-independent, Req 29.2).
pub fn severity_prefix(severity: &Severity) -> &'static str {
    match severity {
        Severity::Critical => "[CRIT]",
        Severity::Warning => "[WARN]",
        Severity::Info => "[INFO]",
    }
}

fn severity_style(severity: &Severity, theme: &Theme) -> Style {
    if theme.no_color {
        return match severity {
            Severity::Critical => Style::default().add_modifier(Modifier::BOLD),
            Severity::Warning => Style::default(),
            Severity::Info => Style::default().add_modifier(Modifier::DIM),
        };
    }
    match severity {
        Severity::Critical => Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
        Severity::Warning => Style::default().fg(Color::Yellow),
        Severity::Info => Style::default().fg(Color::Cyan),
    }
}

/// State for the violations viewport (scroll offset).
#[derive(Debug, Default, Clone)]
pub struct ViolationsState {
    /// First visible line index.
    pub row_offset: usize,
}

impl ViolationsState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn scroll_down(&mut self, n: usize) {
        self.row_offset = self.row_offset.saturating_add(n);
    }

    pub fn scroll_up(&mut self, n: usize) {
        self.row_offset = self.row_offset.saturating_sub(n);
    }
}

/// A (workspace, compliance_score) pair used for ranking.
#[derive(Debug, Clone)]
pub struct WorkspaceScore {
    pub workspace: String,
    pub compliance_score: f64,
}

/// Build the rendered lines for the violations dashboard.
///
/// `violations` — all violations to display.
/// `workspace_scores` — per-workspace compliance scores (worst first when sorted ascending).
pub fn build_violations_lines<'a>(
    violations: &'a [PolicyViolation],
    workspace_scores: &'a [WorkspaceScore],
    state: &ViolationsState,
    theme: &Theme,
) -> Vec<Line<'a>> {
    let mut lines: Vec<Line<'a>> = Vec::new();

    // ── header ──────────────────────────────────────────────────────────────
    lines.push(Line::from(vec![Span::styled(
        "Policy Violations  [CRIT]=Critical [WARN]=Warning [INFO]=Info",
        Style::default().add_modifier(Modifier::BOLD),
    )]));
    lines.push(Line::from(vec![Span::raw("")]));

    if violations.is_empty() {
        lines.push(Line::from(vec![Span::styled(
            "  No policy violations detected.",
            Style::default().fg(Color::Green),
        )]));
        return lines.into_iter().skip(state.row_offset).collect();
    }

    // ── workspace compliance score summary (ascending = worst first) ────────
    if !workspace_scores.is_empty() {
        let mut sorted_scores = workspace_scores.to_vec();
        sorted_scores.sort_by(|a, b| {
            a.compliance_score
                .partial_cmp(&b.compliance_score)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then(a.workspace.cmp(&b.workspace))
        });

        lines.push(Line::from(vec![Span::styled(
            "Workspace Compliance (worst first):",
            Style::default().add_modifier(Modifier::BOLD),
        )]));
        for ws in &sorted_scores {
            let score_str = format!("{:.1}%", ws.compliance_score);
            let score_style = if ws.compliance_score < 50.0 {
                if theme.no_color {
                    Style::default().add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(Color::Red)
                }
            } else if ws.compliance_score < 80.0 {
                if theme.no_color {
                    Style::default()
                } else {
                    Style::default().fg(Color::Yellow)
                }
            } else if theme.no_color {
                Style::default()
            } else {
                Style::default().fg(Color::Green)
            };
            lines.push(Line::from(vec![
                Span::raw("  "),
                Span::raw(ws.workspace.clone()),
                Span::raw(": "),
                Span::styled(score_str, score_style),
            ]));
        }
        lines.push(Line::from(vec![Span::raw("")]));
    }

    // ── violations grouped by severity then workspace ─────────────────────
    for sev in [Severity::Critical, Severity::Warning, Severity::Info] {
        let sev_violations: Vec<&PolicyViolation> = violations
            .iter()
            .filter(|v| v.rule.severity == sev)
            .collect();

        if sev_violations.is_empty() {
            continue;
        }

        let prefix = severity_prefix(&sev);
        let sev_style = severity_style(&sev, theme);

        lines.push(Line::from(vec![
            Span::styled(prefix, sev_style),
            Span::styled(
                format!(" {} violations", sev_violations.len()),
                Style::default().add_modifier(Modifier::BOLD),
            ),
        ]));

        // Group by workspace and sort workspaces by compliance score (ascending)
        let mut workspaces_seen: Vec<&str> = sev_violations
            .iter()
            .map(|v| v.workspace.as_str())
            .collect();
        workspaces_seen.sort_unstable();
        workspaces_seen.dedup();

        // Sort workspace order by score if available (worst first)
        let score_map: std::collections::HashMap<&str, f64> = workspace_scores
            .iter()
            .map(|ws| (ws.workspace.as_str(), ws.compliance_score))
            .collect();
        workspaces_seen.sort_by(|a, b| {
            let sa = score_map.get(a).copied().unwrap_or(100.0);
            let sb = score_map.get(b).copied().unwrap_or(100.0);
            sa.partial_cmp(&sb)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then(a.cmp(b))
        });

        for ws in &workspaces_seen {
            let ws_viols: Vec<&&PolicyViolation> = sev_violations
                .iter()
                .filter(|v| v.workspace == *ws)
                .collect();

            lines.push(Line::from(vec![
                Span::raw("  "),
                Span::styled(
                    ws.to_string(),
                    Style::default().add_modifier(Modifier::BOLD),
                ),
                Span::raw(format!(
                    " ({} violation{})",
                    ws_viols.len(),
                    if ws_viols.len() == 1 { "" } else { "s" }
                )),
            ]));

            for v in &ws_viols {
                lines.push(Line::from(vec![
                    Span::raw("    "),
                    Span::styled(prefix, sev_style),
                    Span::raw(" "),
                    Span::raw(v.rule.id.clone()),
                    Span::raw(": "),
                    Span::raw(v.details.clone()),
                ]));
                if !v.remediation.is_empty() {
                    lines.push(Line::from(vec![
                        Span::raw("      → "),
                        Span::styled(
                            v.remediation.clone(),
                            Style::default().add_modifier(Modifier::DIM),
                        ),
                    ]));
                }
            }
        }

        lines.push(Line::from(vec![Span::raw("")]));
    }

    lines.into_iter().skip(state.row_offset).collect()
}

/// Render the violations dashboard into a [`Frame`].
pub fn render_violations(
    violations: &[PolicyViolation],
    workspace_scores: &[WorkspaceScore],
    state: &ViolationsState,
    area: Rect,
    frame: &mut Frame,
    theme: &Theme,
) {
    let lines = build_violations_lines(violations, workspace_scores, state, theme);
    let paragraph = Paragraph::new(lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Policy Violations")
                .border_style(theme.border_style()),
        )
        .scroll((0, 0));
    frame.render_widget(paragraph, area);
}

/// Render the violations dashboard into a [`Buffer`] (useful for testing).
pub fn render_violations_buffer(
    violations: &[PolicyViolation],
    workspace_scores: &[WorkspaceScore],
    state: &ViolationsState,
    area: Rect,
    buf: &mut Buffer,
    theme: &Theme,
) {
    let lines = build_violations_lines(violations, workspace_scores, state, theme);
    let paragraph = Paragraph::new(lines).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Policy Violations")
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
    use crate::models::policy::{
        PolicyRule, PolicyRuleType, PolicyScope, PolicyViolation, Severity,
    };
    use crate::ui::theme::{ColorSupport, Theme};
    use ratatui::backend::TestBackend;
    use ratatui::buffer::Buffer;
    use ratatui::layout::Rect;
    use ratatui::Terminal;

    fn make_rule(id: &str, severity: Severity) -> PolicyRule {
        PolicyRule {
            id: id.to_string(),
            rule_type: PolicyRuleType::RequireAsset {
                asset_id: "some-asset".to_string(),
            },
            severity,
            scope: PolicyScope::All,
            description: "Test rule".to_string(),
        }
    }

    fn make_violation(rule_id: &str, severity: Severity, workspace: &str) -> PolicyViolation {
        PolicyViolation {
            rule: make_rule(rule_id, severity),
            workspace: workspace.to_string(),
            asset_id: Some("some-asset".to_string()),
            first_detected: "2025-01-01T00:00:00.000Z".to_string(),
            details: format!("Asset missing in {workspace}"),
            remediation: "Install the asset.".to_string(),
        }
    }

    fn make_scores(workspaces: &[(&str, f64)]) -> Vec<WorkspaceScore> {
        workspaces
            .iter()
            .map(|(ws, score)| WorkspaceScore {
                workspace: ws.to_string(),
                compliance_score: *score,
            })
            .collect()
    }

    #[test]
    fn violations_shows_critical_prefix() {
        let violations = vec![make_violation("req-scanner", Severity::Critical, "prod")];
        let scores = make_scores(&[("prod", 50.0)]);
        let state = ViolationsState::new();
        let theme = Theme::with_color_support(ColorSupport::None);
        let area = Rect::new(0, 0, 100, 30);
        let mut buf = Buffer::empty(area);
        render_violations_buffer(&violations, &scores, &state, area, &mut buf, &theme);

        let content = buf_content(&buf);
        assert!(
            content.contains("[CRIT]"),
            "expected [CRIT] prefix for Critical severity"
        );
    }

    #[test]
    fn violations_shows_warn_prefix() {
        let violations = vec![make_violation("req-role", Severity::Warning, "staging")];
        let scores = make_scores(&[("staging", 75.0)]);
        let state = ViolationsState::new();
        let theme = Theme::with_color_support(ColorSupport::None);
        let area = Rect::new(0, 0, 100, 30);
        let mut buf = Buffer::empty(area);
        render_violations_buffer(&violations, &scores, &state, area, &mut buf, &theme);

        let content = buf_content(&buf);
        assert!(
            content.contains("[WARN]"),
            "expected [WARN] prefix for Warning severity"
        );
    }

    #[test]
    fn violations_shows_info_prefix() {
        let violations = vec![make_violation("info-rule", Severity::Info, "dev")];
        let scores = make_scores(&[("dev", 90.0)]);
        let state = ViolationsState::new();
        let theme = Theme::with_color_support(ColorSupport::None);
        let area = Rect::new(0, 0, 100, 30);
        let mut buf = Buffer::empty(area);
        render_violations_buffer(&violations, &scores, &state, area, &mut buf, &theme);

        let content = buf_content(&buf);
        assert!(
            content.contains("[INFO]"),
            "expected [INFO] prefix for Info severity"
        );
    }

    #[test]
    fn violations_critical_appears_before_warning() {
        let violations = vec![
            make_violation("warn-rule", Severity::Warning, "prod"),
            make_violation("crit-rule", Severity::Critical, "prod"),
        ];
        let scores = make_scores(&[("prod", 40.0)]);
        let state = ViolationsState::new();
        let theme = Theme::with_color_support(ColorSupport::None);
        let area = Rect::new(0, 0, 100, 40);
        let mut buf = Buffer::empty(area);
        render_violations_buffer(&violations, &scores, &state, area, &mut buf, &theme);

        let content = buf_content(&buf);
        let crit_pos = content.find("[CRIT]").expect("[CRIT] must appear");
        let warn_pos = content.find("[WARN]").expect("[WARN] must appear");
        assert!(
            crit_pos < warn_pos,
            "Critical section must appear before Warning section"
        );
    }

    #[test]
    fn violations_worst_workspace_first() {
        let violations = vec![
            make_violation("rule1", Severity::Critical, "good-ws"),
            make_violation("rule2", Severity::Critical, "bad-ws"),
        ];
        let scores = make_scores(&[("good-ws", 80.0), ("bad-ws", 20.0)]);
        let state = ViolationsState::new();
        let theme = Theme::with_color_support(ColorSupport::None);
        let area = Rect::new(0, 0, 100, 40);
        let mut buf = Buffer::empty(area);
        render_violations_buffer(&violations, &scores, &state, area, &mut buf, &theme);

        let content = buf_content(&buf);
        let bad_pos = content.find("bad-ws").expect("bad-ws must appear");
        let good_pos = content.find("good-ws").expect("good-ws must appear");
        // bad-ws has lower score → should appear first in the score table
        assert!(
            bad_pos < good_pos,
            "workspace with lower score should appear first"
        );
    }

    #[test]
    fn violations_empty_shows_no_violations_message() {
        let violations: Vec<PolicyViolation> = vec![];
        let scores: Vec<WorkspaceScore> = vec![];
        let state = ViolationsState::new();
        let theme = Theme::with_color_support(ColorSupport::None);
        let area = Rect::new(0, 0, 80, 20);
        let mut buf = Buffer::empty(area);
        render_violations_buffer(&violations, &scores, &state, area, &mut buf, &theme);

        let content = buf_content(&buf);
        assert!(
            content.contains("No policy violations"),
            "empty list should show 'No policy violations' message"
        );
    }

    #[test]
    fn severity_prefixes_all_nonempty() {
        for s in [Severity::Critical, Severity::Warning, Severity::Info] {
            assert!(
                !severity_prefix(&s).trim().is_empty(),
                "prefix for {s:?} must be non-empty"
            );
        }
    }

    #[test]
    fn violations_via_test_backend() {
        let violations = vec![make_violation("req-scanner", Severity::Critical, "prod")];
        let scores = make_scores(&[("prod", 50.0)]);
        let state = ViolationsState::new();
        let theme = Theme::with_color_support(ColorSupport::None);

        let backend = TestBackend::new(100, 30);
        let mut terminal = Terminal::new(backend).unwrap();
        terminal
            .draw(|frame| {
                render_violations(&violations, &scores, &state, frame.area(), frame, &theme);
            })
            .unwrap();

        let buf = terminal.backend().buffer().clone();
        let content = buf_content(&buf);
        assert!(content.contains("[CRIT]"));
    }
}
