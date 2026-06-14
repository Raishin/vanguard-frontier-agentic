/// Dependency graph ASCII tree widget.
///
/// Req 5.4: ASCII art dependency tree with expandable/collapsible nodes,
///          upstream/downstream highlight.
///
/// Leverages `DependencyGraph::render_ascii_tree` for content generation
/// and renders it into a scrollable panel.
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget},
    Frame,
};

use crate::federation::dep_graph::DependencyGraph;
use crate::ui::theme::Theme;

/// State for the dependency graph viewer.
#[derive(Debug, Clone)]
pub struct DepGraphState {
    /// Node currently focused (highlight upstream/downstream).
    pub focus_id: Option<String>,
    /// Maximum render depth for the ASCII tree.
    pub max_depth: usize,
    /// Scroll offset (first visible line).
    pub row_offset: usize,
    /// Set of node IDs that are "collapsed" (children hidden).
    pub collapsed: std::collections::HashSet<String>,
}

impl Default for DepGraphState {
    fn default() -> Self {
        Self {
            focus_id: None,
            max_depth: 5,
            row_offset: 0,
            collapsed: std::collections::HashSet::new(),
        }
    }
}

impl DepGraphState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn scroll_down(&mut self, n: usize) {
        self.row_offset = self.row_offset.saturating_add(n);
    }

    pub fn scroll_up(&mut self, n: usize) {
        self.row_offset = self.row_offset.saturating_sub(n);
    }

    pub fn toggle_collapse(&mut self, node_id: &str) {
        if self.collapsed.contains(node_id) {
            self.collapsed.remove(node_id);
        } else {
            self.collapsed.insert(node_id.to_string());
        }
    }

    pub fn set_focus(&mut self, node_id: Option<String>) {
        self.focus_id = node_id;
        self.row_offset = 0;
    }
}

/// Build the rendered lines for the dependency graph viewer.
///
/// If `state.focus_id` is `Some(id)`, renders the tree rooted at that ID and
/// highlights upstream/downstream nodes.
/// Otherwise renders an overview listing all root-level nodes (nodes with no
/// upstream parents).
pub fn build_dep_graph_lines<'a>(
    graph: &'a DependencyGraph,
    state: &DepGraphState,
    theme: &Theme,
) -> Vec<Line<'a>> {
    let mut lines: Vec<Line<'a>> = Vec::new();

    // Determine upstream/downstream sets for highlighting
    let (upstream_ids, downstream_ids) = if let Some(ref focus) = state.focus_id {
        let up: std::collections::HashSet<String> = graph
            .upstream_ids(focus)
            .into_iter()
            .collect();
        let down: std::collections::HashSet<String> = graph
            .downstream_ids(focus)
            .into_iter()
            .collect();
        (up, down)
    } else {
        (
            std::collections::HashSet::new(),
            std::collections::HashSet::new(),
        )
    };

    // ── header ──────────────────────────────────────────────────────────────
    let header_text = if let Some(ref focus) = state.focus_id {
        format!(
            "Dep Graph: {} (depth ≤{})  ↑=upstream ↓=downstream",
            focus, state.max_depth
        )
    } else {
        format!(
            "Dependency Graph ({} nodes, {} edges)",
            graph.nodes.len(),
            graph.edges.len()
        )
    };

    lines.push(Line::from(vec![Span::styled(
        header_text,
        Style::default().add_modifier(Modifier::BOLD),
    )]));
    lines.push(Line::from(vec![Span::raw("")]));

    if graph.nodes.is_empty() {
        lines.push(Line::from(vec![Span::raw("  (no nodes in graph)")]));
        return lines.into_iter().skip(state.row_offset).collect();
    }

    if let Some(ref focus_id) = state.focus_id {
        // Render ASCII tree rooted at focused node
        let tree_text = graph.render_ascii_tree(focus_id, state.max_depth);
        for raw_line in tree_text.lines() {
            // Highlight upstream/downstream nodes by scanning for known IDs
            let line = build_annotated_line(
                raw_line,
                &upstream_ids,
                &downstream_ids,
                theme,
            );
            lines.push(line);
        }

        // Legend
        lines.push(Line::from(vec![Span::raw("")]));
        if !upstream_ids.is_empty() {
            let up_list: Vec<&str> = upstream_ids.iter().map(|s| s.as_str()).collect();
            lines.push(Line::from(vec![
                Span::raw("  ↑ upstream: "),
                Span::styled(
                    up_list.join(", "),
                    upstream_style(theme),
                ),
            ]));
        }
        if !downstream_ids.is_empty() {
            let down_list: Vec<&str> = downstream_ids.iter().map(|s| s.as_str()).collect();
            lines.push(Line::from(vec![
                Span::raw("  ↓ downstream: "),
                Span::styled(
                    down_list.join(", "),
                    downstream_style(theme),
                ),
            ]));
        }
    } else {
        // Overview: list all nodes with their type and edge counts
        let mut sorted_ids: Vec<&str> = graph.nodes.keys().map(|s| s.as_str()).collect();
        sorted_ids.sort_unstable();

        for node_id in sorted_ids {
            let up_count = graph.upstream_ids(node_id).len();
            let down_count = graph.downstream_ids(node_id).len();
            let collapsed_marker = if state.collapsed.contains(node_id) {
                " [+]"
            } else {
                ""
            };
            lines.push(Line::from(vec![
                Span::raw("  "),
                Span::styled(
                    node_id.to_string(),
                    Style::default().add_modifier(Modifier::BOLD),
                ),
                Span::raw(format!(
                    "  ↑{up_count} ↓{down_count}{collapsed_marker}"
                )),
            ]));
        }
    }

    lines.into_iter().skip(state.row_offset).collect()
}

fn upstream_style(theme: &Theme) -> Style {
    if theme.no_color {
        Style::default().add_modifier(Modifier::UNDERLINED)
    } else {
        Style::default().fg(Color::Cyan)
    }
}

fn downstream_style(theme: &Theme) -> Style {
    if theme.no_color {
        Style::default().add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::Yellow)
    }
}

/// Annotate a single ASCII tree line with upstream/downstream highlight spans.
fn build_annotated_line<'a>(
    raw_line: &str,
    upstream_ids: &std::collections::HashSet<String>,
    downstream_ids: &std::collections::HashSet<String>,
    theme: &Theme,
) -> Line<'a> {
    // Find the node ID embedded in the line (strip leading whitespace/connectors)
    let trimmed = raw_line.trim_start_matches(' ');
    // Remove any [cycle] suffix for matching
    let id_candidate = trimmed.trim_end_matches(" [cycle]");

    let style = if upstream_ids.contains(id_candidate) {
        upstream_style(theme)
    } else if downstream_ids.contains(id_candidate) {
        downstream_style(theme)
    } else {
        Style::default()
    };

    Line::from(vec![Span::styled(raw_line.to_string(), style)])
}

/// Render the dependency graph into a [`Frame`].
pub fn render_dep_graph(
    graph: &DependencyGraph,
    state: &DepGraphState,
    area: Rect,
    frame: &mut Frame,
    theme: &Theme,
) {
    let lines = build_dep_graph_lines(graph, state, theme);
    let paragraph = Paragraph::new(lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Dependencies")
                .border_style(theme.border_style()),
        )
        .scroll((0, 0));
    frame.render_widget(paragraph, area);
}

/// Render the dependency graph into a [`Buffer`] (useful for testing).
pub fn render_dep_graph_buffer(
    graph: &DependencyGraph,
    state: &DepGraphState,
    area: Rect,
    buf: &mut Buffer,
    theme: &Theme,
) {
    let lines = build_dep_graph_lines(graph, state, theme);
    let paragraph = Paragraph::new(lines).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Dependencies")
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
    use crate::federation::dep_graph::{DependencyEdge, DependencyGraph, DependencyNode, AssetType, EdgeType};
    use crate::ui::theme::{ColorSupport, Theme};
    use ratatui::backend::TestBackend;
    use ratatui::buffer::Buffer;
    use ratatui::layout::Rect;
    use ratatui::Terminal;
    use std::collections::HashMap;

    fn make_graph() -> DependencyGraph {
        let mut nodes = HashMap::new();
        nodes.insert(
            "agent-a".to_string(),
            DependencyNode {
                id: "agent-a".to_string(),
                asset_type: AssetType::Agent,
            },
        );
        nodes.insert(
            "skill-x".to_string(),
            DependencyNode {
                id: "skill-x".to_string(),
                asset_type: AssetType::Skill,
            },
        );
        nodes.insert(
            "role-r".to_string(),
            DependencyNode {
                id: "role-r".to_string(),
                asset_type: AssetType::Role,
            },
        );
        let edges = vec![
            DependencyEdge {
                from: "agent-a".to_string(),
                to: "skill-x".to_string(),
                edge_type: EdgeType::DependsOn,
            },
            DependencyEdge {
                from: "role-r".to_string(),
                to: "agent-a".to_string(),
                edge_type: EdgeType::Contains,
            },
        ];
        DependencyGraph { nodes, edges }
    }

    #[test]
    fn dep_graph_overview_shows_node_ids() {
        let graph = make_graph();
        let state = DepGraphState::new();
        let theme = Theme::with_color_support(ColorSupport::None);
        let area = Rect::new(0, 0, 80, 20);
        let mut buf = Buffer::empty(area);
        render_dep_graph_buffer(&graph, &state, area, &mut buf, &theme);

        let content = buf_content(&buf);
        assert!(content.contains("agent-a"), "agent-a should appear in overview");
        assert!(content.contains("skill-x"), "skill-x should appear in overview");
        assert!(content.contains("role-r"), "role-r should appear in overview");
    }

    #[test]
    fn dep_graph_focused_shows_tree() {
        let graph = make_graph();
        let mut state = DepGraphState::new();
        state.set_focus(Some("agent-a".to_string()));
        let theme = Theme::with_color_support(ColorSupport::None);
        let area = Rect::new(0, 0, 80, 20);
        let mut buf = Buffer::empty(area);
        render_dep_graph_buffer(&graph, &state, area, &mut buf, &theme);

        let content = buf_content(&buf);
        assert!(content.contains("agent-a"), "focused node should appear");
        assert!(content.contains("skill-x"), "child node should appear in tree");
    }

    #[test]
    fn dep_graph_upstream_downstream_annotation() {
        let graph = make_graph();
        let mut state = DepGraphState::new();
        state.set_focus(Some("agent-a".to_string()));
        let theme = Theme::with_color_support(ColorSupport::None);
        let area = Rect::new(0, 0, 100, 20);
        let mut buf = Buffer::empty(area);
        render_dep_graph_buffer(&graph, &state, area, &mut buf, &theme);

        let content = buf_content(&buf);
        // upstream of agent-a is skill-x, downstream is role-r
        assert!(content.contains("upstream"), "upstream annotation should appear");
        assert!(content.contains("skill-x"), "upstream node skill-x should appear");
    }

    #[test]
    fn dep_graph_empty_shows_no_nodes_message() {
        let graph = DependencyGraph {
            nodes: HashMap::new(),
            edges: vec![],
        };
        let state = DepGraphState::new();
        let theme = Theme::with_color_support(ColorSupport::None);
        let area = Rect::new(0, 0, 80, 10);
        let mut buf = Buffer::empty(area);
        render_dep_graph_buffer(&graph, &state, area, &mut buf, &theme);

        let content = buf_content(&buf);
        assert!(content.contains("no nodes"), "empty graph should show hint");
    }

    #[test]
    fn dep_graph_collapse_marker_shown() {
        let graph = make_graph();
        let mut state = DepGraphState::new();
        state.toggle_collapse("agent-a");
        let theme = Theme::with_color_support(ColorSupport::None);
        let area = Rect::new(0, 0, 80, 20);
        let mut buf = Buffer::empty(area);
        render_dep_graph_buffer(&graph, &state, area, &mut buf, &theme);

        let content = buf_content(&buf);
        assert!(content.contains("[+]"), "collapsed node should show [+] marker");
    }

    #[test]
    fn dep_graph_toggle_collapse_removes_marker() {
        let graph = make_graph();
        let mut state = DepGraphState::new();
        state.toggle_collapse("agent-a");
        state.toggle_collapse("agent-a"); // toggle back
        let theme = Theme::with_color_support(ColorSupport::None);
        let area = Rect::new(0, 0, 80, 20);
        let mut buf = Buffer::empty(area);
        render_dep_graph_buffer(&graph, &state, area, &mut buf, &theme);

        let content = buf_content(&buf);
        assert!(!content.contains("[+]"), "un-collapsed node should not show [+] marker");
    }

    #[test]
    fn dep_graph_via_test_backend() {
        let graph = make_graph();
        let state = DepGraphState::new();
        let theme = Theme::with_color_support(ColorSupport::None);

        let backend = TestBackend::new(80, 20);
        let mut terminal = Terminal::new(backend).unwrap();
        terminal
            .draw(|frame| {
                render_dep_graph(&graph, &state, frame.area(), frame, &theme);
            })
            .unwrap();

        let buf = terminal.backend().buffer().clone();
        let content = buf_content(&buf);
        assert!(content.contains("agent-a"));
    }

    #[test]
    fn dep_graph_scroll_hides_header() {
        let graph = make_graph();
        let mut state = DepGraphState::new();
        let theme = Theme::with_color_support(ColorSupport::None);
        let area = Rect::new(0, 0, 100, 20);

        let mut buf = Buffer::empty(area);
        render_dep_graph_buffer(&graph, &state, area, &mut buf, &theme);
        let content = buf_content(&buf);
        assert!(content.contains("Dependency Graph"), "header visible at offset 0");

        state.scroll_down(2);
        let mut buf2 = Buffer::empty(area);
        render_dep_graph_buffer(&graph, &state, area, &mut buf2, &theme);
        // After scrolling 2 lines the header is gone (it's lines 0 and 1)
        let row1 = (0..area.width)
            .map(|col| {
                buf2.cell((col, 1))
                    .map(|c| c.symbol().chars().next().unwrap_or(' '))
                    .unwrap_or(' ')
            })
            .collect::<String>();
        assert!(!row1.contains("Dep"), "header should be scrolled off");
    }
}
