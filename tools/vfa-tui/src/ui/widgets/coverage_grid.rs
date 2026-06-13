/// Coverage grid widget — renders the coverage matrix as a scrollable,
/// color-coded grid where rows = catalog assets, columns = workspaces.
///
/// Req 3.2: color-coded cells (green=Installed/Current, yellow=Outdated,
///           red=Drifted, gray=NotInstalled).
/// Req 29.2: text glyphs alongside color so the widget is readable without color.
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget},
    Frame,
};

use crate::models::coverage::{AssetType, CellStatus, CoverageMatrix, CoverageRow};
use crate::ui::theme::Theme;

/// Text glyph for each cell status (color-independent, Req 29.2).
pub fn cell_glyph(status: &CellStatus) -> &'static str {
    match status {
        CellStatus::Installed => "[OK]",
        CellStatus::Outdated => "[OLD]",
        CellStatus::Drifted => "[DRF]",
        CellStatus::NotInstalled => "[--]",
    }
}

/// Color for each cell status (falls back gracefully in no-color mode).
fn cell_color(status: &CellStatus, theme: &Theme) -> Style {
    if theme.no_color {
        return Style::default();
    }
    match status {
        CellStatus::Installed => Style::default().fg(Color::Green),
        CellStatus::Outdated => Style::default().fg(Color::Yellow),
        CellStatus::Drifted => Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
        CellStatus::NotInstalled => Style::default().fg(Color::DarkGray),
    }
}

/// Active filter state for the coverage grid.
#[derive(Debug, Default, Clone)]
pub struct CoverageGridFilter {
    /// Restrict to a specific asset type (`None` = all types).
    pub asset_type: Option<AssetType>,
    /// Restrict to a provider string (`None` = all providers).
    pub provider: Option<String>,
    /// Restrict columns to workspaces whose name contains this substring.
    pub workspace: Option<String>,
}

impl CoverageGridFilter {
    pub fn new() -> Self {
        Self::default()
    }

    fn row_passes(&self, row: &CoverageRow) -> bool {
        if let Some(ref at) = self.asset_type {
            if &row.asset_type != at {
                return false;
            }
        }
        if let Some(ref prov) = self.provider {
            let row_prov = serde_json::to_value(&row.provider)
                .ok()
                .and_then(|v| v.as_str().map(|s| s.to_string()))
                .unwrap_or_default();
            if !row_prov.contains(prov.as_str()) {
                return false;
            }
        }
        true
    }

    fn col_passes(&self, workspace: &str) -> bool {
        if let Some(ref ws) = self.workspace {
            return workspace.contains(ws.as_str());
        }
        true
    }
}

/// State for the coverage grid viewport (scroll offsets).
#[derive(Debug, Default, Clone)]
pub struct CoverageGridState {
    /// First visible row index (asset dimension).
    pub row_offset: usize,
    /// First visible column index (workspace dimension).
    pub col_offset: usize,
}

impl CoverageGridState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn scroll_down(&mut self, n: usize) {
        self.row_offset = self.row_offset.saturating_add(n);
    }

    pub fn scroll_up(&mut self, n: usize) {
        self.row_offset = self.row_offset.saturating_sub(n);
    }

    pub fn scroll_right(&mut self, n: usize) {
        self.col_offset = self.col_offset.saturating_add(n);
    }

    pub fn scroll_left(&mut self, n: usize) {
        self.col_offset = self.col_offset.saturating_sub(n);
    }
}

/// Render the coverage grid into a [`Frame`].
///
/// The grid header shows workspace names (column headers), and each
/// subsequent line shows one asset row with a cell glyph per workspace.
pub fn render_coverage_grid(
    matrix: &CoverageMatrix,
    state: &CoverageGridState,
    filter: &CoverageGridFilter,
    area: Rect,
    frame: &mut Frame,
    theme: &Theme,
) {
    let lines = build_grid_lines(matrix, state, filter, theme, area.width as usize);
    let paragraph = Paragraph::new(lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Coverage Matrix  [OK]=Installed [OLD]=Outdated [DRF]=Drifted [--]=Missing")
                .border_style(theme.border_style()),
        )
        .scroll((0, 0));
    frame.render_widget(paragraph, area);
}

/// Pure helper — build the rendered lines for a coverage grid.
///
/// Exposed so tests can call it directly on a `Buffer`.
pub fn build_grid_lines<'a>(
    matrix: &'a CoverageMatrix,
    state: &CoverageGridState,
    filter: &CoverageGridFilter,
    theme: &Theme,
    max_width: usize,
) -> Vec<Line<'a>> {
    // Filtered rows and columns
    let rows: Vec<&CoverageRow> = matrix
        .rows
        .iter()
        .filter(|r| filter.row_passes(r))
        .collect();
    let cols: Vec<&String> = matrix
        .columns
        .iter()
        .filter(|c| filter.col_passes(c))
        .collect();

    // Skip rows/cols before viewport offset
    let visible_rows = rows.into_iter().skip(state.row_offset);
    let visible_cols: Vec<&String> = cols.into_iter().skip(state.col_offset).collect();

    // Cell width: "[DRF]" is 5 chars; we pad to CELL_W chars total
    const CELL_W: usize = 6;
    const NAME_W: usize = 22;

    let mut lines: Vec<Line<'a>> = Vec::new();

    // ── header line ──────────────────────────────────────────────────────────
    let mut header_spans: Vec<Span<'a>> = vec![Span::styled(
        format!("{:<width$}", "Asset", width = NAME_W),
        Style::default().add_modifier(Modifier::BOLD),
    )];
    let mut col_count = 0;
    for ws in &visible_cols {
        let used = NAME_W + col_count * CELL_W;
        if used + CELL_W > max_width {
            break;
        }
        let label = if ws.len() >= CELL_W {
            ws[..CELL_W - 1].to_string()
        } else {
            format!("{:<width$}", &ws[..], width = CELL_W)
        };
        header_spans.push(Span::styled(
            label,
            Style::default().add_modifier(Modifier::BOLD),
        ));
        col_count += 1;
    }
    let visible_cols_truncated = &visible_cols[..col_count.min(visible_cols.len())];
    lines.push(Line::from(header_spans));

    // ── data rows ─────────────────────────────────────────────────────────────
    for row in visible_rows {
        let asset_label = if row.asset_name.len() >= NAME_W {
            format!("{:.width$}", row.asset_name, width = NAME_W - 1)
        } else {
            format!("{:<width$}", row.asset_name, width = NAME_W)
        };

        let mut spans: Vec<Span<'a>> = vec![Span::raw(asset_label)];
        for ws in visible_cols_truncated {
            let key = (row.asset_id.clone(), ws.as_str().to_string());
            let cell = matrix.cells.get(&key);
            let (glyph, style) = match cell {
                Some(c) => (cell_glyph(&c.status), cell_color(&c.status, theme)),
                None => ("[--]", Style::default().fg(Color::DarkGray)),
            };
            spans.push(Span::styled(format!("{:<width$}", glyph, width = CELL_W), style));
        }
        lines.push(Line::from(spans));
    }

    if lines.len() == 1 {
        lines.push(Line::from(vec![Span::raw("  (no assets match current filter)")]));
    }

    lines
}

/// Render coverage grid directly into a `Buffer` (useful for testing).
pub fn render_coverage_grid_buffer(
    matrix: &CoverageMatrix,
    state: &CoverageGridState,
    filter: &CoverageGridFilter,
    area: Rect,
    buf: &mut Buffer,
    theme: &Theme,
) {
    let lines = build_grid_lines(matrix, state, filter, theme, area.width as usize);
    let paragraph = Paragraph::new(lines).block(
        Block::default()
            .borders(Borders::ALL)
            .title("Coverage Matrix")
            .border_style(theme.border_style()),
    );
    Widget::render(paragraph, area, buf);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::coverage::{AssetType, CellStatus, CoverageCell, CoverageMatrix, CoverageRow};
    use crate::models::provider::Provider;
    use crate::ui::theme::{ColorSupport, Theme};
    use ratatui::backend::TestBackend;
    use ratatui::buffer::Buffer;
    use ratatui::layout::Rect;
    use ratatui::Terminal;
    use std::collections::HashMap;

    fn make_matrix() -> CoverageMatrix {
        let rows = vec![
            CoverageRow {
                asset_id: "agent-alpha".to_string(),
                asset_type: AssetType::Agent,
                asset_name: "Alpha Agent".to_string(),
                provider: Provider::Aws,
            },
            CoverageRow {
                asset_id: "skill-beta".to_string(),
                asset_type: AssetType::Skill,
                asset_name: "Beta Skill".to_string(),
                provider: Provider::Aws,
            },
        ];
        let columns = vec!["prod".to_string(), "staging".to_string()];
        let mut cells = HashMap::new();
        cells.insert(
            ("agent-alpha".to_string(), "prod".to_string()),
            CoverageCell {
                status: CellStatus::Installed,
                installed_version: Some("1.0.0".to_string()),
                canonical_version: "1.0.0".to_string(),
                installed_hash: Some("abc".to_string()),
                canonical_hash: Some("abc".to_string()),
            },
        );
        cells.insert(
            ("agent-alpha".to_string(), "staging".to_string()),
            CoverageCell {
                status: CellStatus::Outdated,
                installed_version: Some("0.9.0".to_string()),
                canonical_version: "1.0.0".to_string(),
                installed_hash: Some("xyz".to_string()),
                canonical_hash: Some("abc".to_string()),
            },
        );
        cells.insert(
            ("skill-beta".to_string(), "prod".to_string()),
            CoverageCell {
                status: CellStatus::Drifted,
                installed_version: Some("1.0.0".to_string()),
                canonical_version: "1.0.0".to_string(),
                installed_hash: Some("WRONG".to_string()),
                canonical_hash: Some("abc".to_string()),
            },
        );
        // skill-beta / staging → not installed (no entry in map)
        let mut workspace_scores = HashMap::new();
        workspace_scores.insert("prod".to_string(), 50.0);
        workspace_scores.insert("staging".to_string(), 0.0);

        CoverageMatrix {
            rows,
            columns,
            cells,
            workspace_scores,
        }
    }

    #[test]
    fn coverage_grid_renders_ok_glyph() {
        let matrix = make_matrix();
        let state = CoverageGridState::new();
        let filter = CoverageGridFilter::new();
        let theme = Theme::with_color_support(ColorSupport::None);
        let area = Rect::new(0, 0, 80, 20);
        let mut buf = Buffer::empty(area);
        render_coverage_grid_buffer(&matrix, &state, &filter, area, &mut buf, &theme);

        let content: String = buf
            .content
            .iter()
            .map(|c| c.symbol().chars().next().unwrap_or(' '))
            .collect();
        assert!(content.contains("[OK]"), "expected [OK] glyph for Installed");
    }

    #[test]
    fn coverage_grid_renders_old_glyph() {
        let matrix = make_matrix();
        let state = CoverageGridState::new();
        let filter = CoverageGridFilter::new();
        let theme = Theme::with_color_support(ColorSupport::None);
        let area = Rect::new(0, 0, 80, 20);
        let mut buf = Buffer::empty(area);
        render_coverage_grid_buffer(&matrix, &state, &filter, area, &mut buf, &theme);

        let content: String = buf
            .content
            .iter()
            .map(|c| c.symbol().chars().next().unwrap_or(' '))
            .collect();
        assert!(content.contains("[OLD]"), "expected [OLD] glyph for Outdated");
    }

    #[test]
    fn coverage_grid_renders_drf_glyph() {
        let matrix = make_matrix();
        let state = CoverageGridState::new();
        let filter = CoverageGridFilter::new();
        let theme = Theme::with_color_support(ColorSupport::None);
        let area = Rect::new(0, 0, 80, 20);
        let mut buf = Buffer::empty(area);
        render_coverage_grid_buffer(&matrix, &state, &filter, area, &mut buf, &theme);

        let content: String = buf
            .content
            .iter()
            .map(|c| c.symbol().chars().next().unwrap_or(' '))
            .collect();
        assert!(content.contains("[DRF]"), "expected [DRF] glyph for Drifted");
    }

    #[test]
    fn coverage_grid_renders_missing_glyph_for_not_installed() {
        let matrix = make_matrix();
        let state = CoverageGridState::new();
        let filter = CoverageGridFilter::new();
        let theme = Theme::with_color_support(ColorSupport::None);
        let area = Rect::new(0, 0, 80, 20);
        let mut buf = Buffer::empty(area);
        render_coverage_grid_buffer(&matrix, &state, &filter, area, &mut buf, &theme);

        let content: String = buf
            .content
            .iter()
            .map(|c| c.symbol().chars().next().unwrap_or(' '))
            .collect();
        assert!(content.contains("[--]"), "expected [--] glyph for NotInstalled/missing");
    }

    #[test]
    fn coverage_grid_filter_by_asset_type() {
        let matrix = make_matrix();
        let state = CoverageGridState::new();
        let filter = CoverageGridFilter {
            asset_type: Some(AssetType::Skill),
            ..Default::default()
        };
        let theme = Theme::with_color_support(ColorSupport::None);
        let area = Rect::new(0, 0, 80, 20);
        let mut buf = Buffer::empty(area);
        render_coverage_grid_buffer(&matrix, &state, &filter, area, &mut buf, &theme);

        let content: String = buf
            .content
            .iter()
            .map(|c| c.symbol().chars().next().unwrap_or(' '))
            .collect();
        // Only Beta Skill row should appear; Alpha Agent row should be absent
        assert!(content.contains("Beta Skill"), "skill row should appear when filtering by Skill");
        assert!(!content.contains("Alpha Agent"), "agent row should be filtered out");
    }

    #[test]
    fn cell_glyph_all_variants_are_nonempty() {
        for status in [
            CellStatus::Installed,
            CellStatus::Outdated,
            CellStatus::Drifted,
            CellStatus::NotInstalled,
        ] {
            let g = cell_glyph(&status);
            assert!(!g.is_empty(), "glyph for {:?} must not be empty", status);
        }
    }

    #[test]
    fn coverage_grid_via_test_backend() {
        let matrix = make_matrix();
        let state = CoverageGridState::new();
        let filter = CoverageGridFilter::new();
        let theme = Theme::with_color_support(ColorSupport::None);

        let backend = TestBackend::new(80, 20);
        let mut terminal = Terminal::new(backend).unwrap();
        terminal
            .draw(|frame| {
                render_coverage_grid(&matrix, &state, &filter, frame.area(), frame, &theme);
            })
            .unwrap();

        let buf = terminal.backend().buffer().clone();
        let content: String = buf
            .content
            .iter()
            .map(|c| c.symbol().chars().next().unwrap_or(' '))
            .collect();
        assert!(content.contains("[OK]"));
    }
}
