/// DAG view widget — renders the gate DAG as execution layers with status labels.
///
/// Req 2.6: show gate relationships, execution status (pending/running/passed/
///          failed/skipped), and timing for the current run.
/// Req 29.2: text labels [PEND]/[RUN]/[PASS]/[FAIL]/[SKIP]/[TIME] ensure
///           readability without color.
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget},
    Frame,
};

use crate::models::gate::{DagGateStatus, GateDAG, GateResult};
use crate::ui::theme::Theme;

/// Text label for each gate status (color-independent, Req 29.2).
pub fn status_label(status: &DagGateStatus) -> &'static str {
    match status {
        DagGateStatus::Pending => "[PEND]",
        DagGateStatus::Running => "[RUN] ",
        DagGateStatus::Passed => "[PASS]",
        DagGateStatus::Failed => "[FAIL]",
        DagGateStatus::Skipped => "[SKIP]",
        DagGateStatus::TimedOut => "[TIME]",
    }
}

/// Style for each gate status.
fn status_style(status: &DagGateStatus, theme: &Theme) -> Style {
    if theme.no_color {
        return match status {
            DagGateStatus::Passed => Style::default(),
            DagGateStatus::Failed | DagGateStatus::TimedOut => {
                Style::default().add_modifier(Modifier::BOLD)
            }
            DagGateStatus::Running => Style::default().add_modifier(Modifier::BOLD),
            _ => Style::default().add_modifier(Modifier::DIM),
        };
    }
    match status {
        DagGateStatus::Pending => Style::default().fg(Color::DarkGray),
        DagGateStatus::Running => Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD),
        DagGateStatus::Passed => Style::default().fg(Color::Green),
        DagGateStatus::Failed => Style::default()
            .fg(Color::Red)
            .add_modifier(Modifier::BOLD),
        DagGateStatus::Skipped => Style::default().fg(Color::DarkGray),
        DagGateStatus::TimedOut => Style::default()
            .fg(Color::Magenta)
            .add_modifier(Modifier::BOLD),
    }
}

/// State for the DAG view (scroll offset).
#[derive(Debug, Default, Clone)]
pub struct DagViewState {
    /// First visible line index.
    pub row_offset: usize,
}

impl DagViewState {
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

/// Build lines for the DAG view given the gate DAG and live results map.
///
/// Results are keyed by gate name.  Gates not yet in `results` are shown
/// with `Pending` status.
pub fn build_dag_lines<'a>(
    dag: &'a GateDAG,
    results: &'a [GateResult],
    state: &DagViewState,
    theme: &Theme,
) -> Vec<Line<'a>> {
    // Build a result lookup by gate name.
    let result_map: std::collections::HashMap<&str, &GateResult> =
        results.iter().map(|r| (r.name.as_str(), r)).collect();

    let mut lines: Vec<Line<'a>> = Vec::new();

    // ── header ──────────────────────────────────────────────────────────────
    lines.push(Line::from(vec![Span::styled(
        "Gate DAG  [PEND]=Pending [RUN]=Running [PASS]=Passed [FAIL]=Failed [SKIP]=Skipped [TIME]=TimedOut",
        Style::default().add_modifier(Modifier::BOLD),
    )]));
    lines.push(Line::from(vec![Span::raw("")]));

    // ── execution layers ──────────────────────────────────────────────────
    for (layer_idx, layer) in dag.execution_order.iter().enumerate() {
        lines.push(Line::from(vec![Span::styled(
            format!("Layer {}:", layer_idx + 1),
            Style::default().add_modifier(Modifier::BOLD),
        )]));

        for gate_name in layer {
            let (status, duration_ms, skip_reason) = match result_map.get(gate_name.as_str()) {
                Some(r) => {
                    let ms = r.duration.as_millis();
                    (r.status.clone(), Some(ms), r.skip_reason.clone())
                }
                None => (DagGateStatus::Pending, None, None),
            };

            let label = status_label(&status);
            let style = status_style(&status, theme);

            let mut spans: Vec<Span<'a>> = Vec::new();
            spans.push(Span::raw("  "));
            spans.push(Span::styled(label.to_string(), style));
            spans.push(Span::raw(" "));
            spans.push(Span::raw(gate_name.clone()));

            if let Some(ms) = duration_ms {
                if ms > 0 || status != DagGateStatus::Pending {
                    spans.push(Span::styled(
                        format!(" ({ms}ms)"),
                        Style::default().add_modifier(Modifier::DIM),
                    ));
                }
            }

            if let Some(reason) = skip_reason {
                spans.push(Span::styled(
                    format!(" — {reason}"),
                    Style::default().add_modifier(Modifier::DIM),
                ));
            }

            // Show dependencies as a small annotation
            let gate_def = dag.gates.iter().find(|g| &g.name == gate_name);
            if let Some(def) = gate_def {
                if !def.dependencies.is_empty() {
                    let deps = def.dependencies.join(", ");
                    spans.push(Span::styled(
                        format!("  [needs: {deps}]"),
                        Style::default().add_modifier(Modifier::DIM),
                    ));
                }
            }

            lines.push(Line::from(spans));
        }
        lines.push(Line::from(vec![Span::raw("")]));
    }

    if dag.execution_order.is_empty() {
        lines.push(Line::from(vec![Span::raw(
            "  (no gates configured — add a gates.toml or validate:* scripts to package.json)",
        )]));
    }

    // Apply row offset
    lines.into_iter().skip(state.row_offset).collect()
}

/// Render the DAG view into a [`Frame`].
pub fn render_dag_view(
    dag: &GateDAG,
    results: &[GateResult],
    state: &DagViewState,
    area: Rect,
    frame: &mut Frame,
    theme: &Theme,
) {
    let lines = build_dag_lines(dag, results, state, theme);
    let paragraph = Paragraph::new(lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Validation Gate DAG")
                .border_style(theme.border_style()),
        )
        .scroll((0, 0));
    frame.render_widget(paragraph, area);
}

/// Render the DAG view into a [`Buffer`] (useful for testing).
pub fn render_dag_view_buffer(
    dag: &GateDAG,
    results: &[GateResult],
    state: &DagViewState,
    area: Rect,
    buf: &mut Buffer,
    theme: &Theme,
) {
    let lines = build_dag_lines(dag, results, state, theme);
    let paragraph = Paragraph::new(lines).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Validation Gate DAG")
            .border_style(theme.border_style()),
    );
    Widget::render(paragraph, area, buf);
}

// ── helpers ──────────────────────────────────────────────────────────────────

/// Collect all characters from a buffer cell row as a string.
#[cfg(test)]
fn buf_row_str(buf: &Buffer, row: u16) -> String {
    let w = buf.area.width;
    (0..w)
        .map(|col| {
            buf.cell((col, row))
                .map(|c| c.symbol().chars().next().unwrap_or(' '))
                .unwrap_or(' ')
        })
        .collect()
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
    use crate::models::gate::{DagGateStatus, GateDAG, GateDefinition, GateResult};
    use crate::ui::theme::{ColorSupport, Theme};
    use ratatui::backend::TestBackend;
    use ratatui::buffer::Buffer;
    use ratatui::layout::Rect;
    use ratatui::Terminal;
    use std::collections::HashMap;
    use std::time::Duration;

    fn make_dag() -> GateDAG {
        let gates = vec![
            GateDefinition {
                name: "lint".to_string(),
                command: "npm".to_string(),
                args: vec!["run".to_string(), "validate:lint".to_string()],
                dependencies: vec![],
                timeout: Duration::from_secs(60),
                description: "Run linter".to_string(),
            },
            GateDefinition {
                name: "types".to_string(),
                command: "npm".to_string(),
                args: vec!["run".to_string(), "validate:types".to_string()],
                dependencies: vec!["lint".to_string()],
                timeout: Duration::from_secs(120),
                description: "Type check".to_string(),
            },
        ];
        let mut adjacency = HashMap::new();
        adjacency.insert("lint".to_string(), vec!["types".to_string()]);
        GateDAG {
            gates,
            adjacency,
            execution_order: vec![
                vec!["lint".to_string()],
                vec!["types".to_string()],
            ],
        }
    }

    fn make_results() -> Vec<GateResult> {
        vec![
            GateResult {
                name: "lint".to_string(),
                status: DagGateStatus::Passed,
                exit_code: Some(0),
                duration: Duration::from_millis(1500),
                timestamp: "2025-01-01T00:00:00.000Z".to_string(),
                output: "All clear.".to_string(),
                skip_reason: None,
            },
            GateResult {
                name: "types".to_string(),
                status: DagGateStatus::Failed,
                exit_code: Some(1),
                duration: Duration::from_millis(800),
                timestamp: "2025-01-01T00:00:02.000Z".to_string(),
                output: "Type error in foo.ts".to_string(),
                skip_reason: None,
            },
        ]
    }

    #[test]
    fn dag_view_shows_pass_label() {
        let dag = make_dag();
        let results = make_results();
        let state = DagViewState::new();
        let theme = Theme::with_color_support(ColorSupport::None);
        let area = Rect::new(0, 0, 100, 20);
        let mut buf = Buffer::empty(area);
        render_dag_view_buffer(&dag, &results, &state, area, &mut buf, &theme);

        let content = buf_content(&buf);
        assert!(content.contains("[PASS]"), "expected [PASS] label");
    }

    #[test]
    fn dag_view_shows_fail_label() {
        let dag = make_dag();
        let results = make_results();
        let state = DagViewState::new();
        let theme = Theme::with_color_support(ColorSupport::None);
        let area = Rect::new(0, 0, 100, 20);
        let mut buf = Buffer::empty(area);
        render_dag_view_buffer(&dag, &results, &state, area, &mut buf, &theme);

        let content = buf_content(&buf);
        assert!(content.contains("[FAIL]"), "expected [FAIL] label");
    }

    #[test]
    fn dag_view_pending_gate_shows_pend() {
        let dag = make_dag();
        let results: Vec<GateResult> = vec![]; // no results → pending
        let state = DagViewState::new();
        let theme = Theme::with_color_support(ColorSupport::None);
        let area = Rect::new(0, 0, 100, 20);
        let mut buf = Buffer::empty(area);
        render_dag_view_buffer(&dag, &results, &state, area, &mut buf, &theme);

        let content = buf_content(&buf);
        assert!(content.contains("[PEND]"), "expected [PEND] label for unstarted gate");
    }

    #[test]
    fn dag_view_skipped_gate_shows_skip() {
        let dag = make_dag();
        let results = vec![
            GateResult {
                name: "lint".to_string(),
                status: DagGateStatus::Failed,
                exit_code: Some(1),
                duration: Duration::from_millis(500),
                timestamp: "2025-01-01T00:00:00.000Z".to_string(),
                output: String::new(),
                skip_reason: None,
            },
            GateResult {
                name: "types".to_string(),
                status: DagGateStatus::Skipped,
                exit_code: None,
                duration: Duration::ZERO,
                timestamp: "2025-01-01T00:00:01.000Z".to_string(),
                output: String::new(),
                skip_reason: Some("dependency failed: lint".to_string()),
            },
        ];
        let state = DagViewState::new();
        let theme = Theme::with_color_support(ColorSupport::None);
        let area = Rect::new(0, 0, 120, 20);
        let mut buf = Buffer::empty(area);
        render_dag_view_buffer(&dag, &results, &state, area, &mut buf, &theme);

        let content = buf_content(&buf);
        assert!(content.contains("[SKIP]"), "expected [SKIP] label for skipped gate");
    }

    #[test]
    fn dag_view_shows_gate_name() {
        let dag = make_dag();
        let results = make_results();
        let state = DagViewState::new();
        let theme = Theme::with_color_support(ColorSupport::None);
        let area = Rect::new(0, 0, 100, 20);
        let mut buf = Buffer::empty(area);
        render_dag_view_buffer(&dag, &results, &state, area, &mut buf, &theme);

        let content = buf_content(&buf);
        assert!(content.contains("lint"), "gate name 'lint' should appear");
        assert!(content.contains("types"), "gate name 'types' should appear");
    }

    #[test]
    fn status_labels_all_nonempty() {
        for s in [
            DagGateStatus::Pending,
            DagGateStatus::Running,
            DagGateStatus::Passed,
            DagGateStatus::Failed,
            DagGateStatus::Skipped,
            DagGateStatus::TimedOut,
        ] {
            assert!(!status_label(&s).trim().is_empty(), "label for {s:?} must be non-empty");
        }
    }

    #[test]
    fn dag_view_via_test_backend() {
        let dag = make_dag();
        let results = make_results();
        let state = DagViewState::new();
        let theme = Theme::with_color_support(ColorSupport::None);

        let backend = TestBackend::new(100, 20);
        let mut terminal = Terminal::new(backend).unwrap();
        terminal
            .draw(|frame| {
                render_dag_view(&dag, &results, &state, frame.area(), frame, &theme);
            })
            .unwrap();

        let buf = terminal.backend().buffer().clone();
        let content = buf_content(&buf);
        assert!(content.contains("[PASS]"));
    }

    #[test]
    fn dag_view_empty_dag() {
        let dag = GateDAG {
            gates: vec![],
            adjacency: HashMap::new(),
            execution_order: vec![],
        };
        let results: Vec<GateResult> = vec![];
        let state = DagViewState::new();
        let theme = Theme::with_color_support(ColorSupport::None);
        let area = Rect::new(0, 0, 100, 10);
        let mut buf = Buffer::empty(area);
        // Should not panic on empty DAG
        render_dag_view_buffer(&dag, &results, &state, area, &mut buf, &theme);
        let content = buf_content(&buf);
        assert!(content.contains("no gates"), "empty DAG should show hint");
    }

    #[test]
    fn dag_view_scroll_offset_skips_lines() {
        let dag = make_dag();
        let results = make_results();
        let mut state = DagViewState::new();
        let theme = Theme::with_color_support(ColorSupport::None);
        let area = Rect::new(0, 0, 100, 20);

        // Without offset — header is visible
        let mut buf = Buffer::empty(area);
        render_dag_view_buffer(&dag, &results, &state, area, &mut buf, &theme);
        let row0 = buf_row_str(&buf, 1); // row 1 = inside border
        assert!(row0.contains("Gate DAG"));

        // With large offset — header scrolled away
        state.scroll_down(5);
        let mut buf2 = Buffer::empty(area);
        render_dag_view_buffer(&dag, &results, &state, area, &mut buf2, &theme);
        let row0b = buf_row_str(&buf2, 1);
        assert!(!row0b.contains("Gate DAG"), "scrolled: header should not be on row 1");
    }
}
