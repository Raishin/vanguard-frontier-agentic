use std::collections::HashSet;
use std::path::PathBuf;
use std::time::Instant;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use ratatui::Frame;
use uuid::Uuid;

use crate::catalog::store::CatalogStore;
use crate::models::export::ExportSelection;
use crate::models::gate::{extract_validation_gates, ValidationGate};
use crate::search::fuzzy::SearchEngine;
use crate::ui::layout::compute_layout;
use crate::ui::nav::{NavigationState, View, SIDEBAR_SECTIONS};
use crate::ui::theme::Theme;
use crate::ui::widgets::{detail, help_bar, list_view, output, search, status_bar};

/// State for the export command builder UI.
pub struct ExportBuilderState {
    pub platform: String,
    pub selection: ExportSelection,
    pub target_repo: String,
    pub dry_run: bool,
    pub force: bool,
    pub no_skills: bool,
    pub focused_field: usize,
}

impl ExportBuilderState {
    pub fn new() -> Self {
        Self {
            platform: "kiro".to_string(),
            selection: ExportSelection::All,
            target_repo: String::new(),
            dry_run: true,
            force: false,
            no_skills: false,
            focused_field: 0,
        }
    }
}

impl Default for ExportBuilderState {
    fn default() -> Self {
        Self::new()
    }
}

/// Main application state.
pub struct App {
    pub nav: NavigationState,
    pub catalog: CatalogStore,
    pub search_engine: SearchEngine,
    pub search_query: String,
    pub search_active: bool,
    pub filtered_indices: Vec<usize>,
    pub subprocess_output: Vec<output::OutputLine>,
    pub validation_gates: Vec<ValidationGate>,
    pub export_state: ExportBuilderState,
    pub status_message: Option<(String, Instant)>,
    pub session_id: Uuid,
    pub should_quit: bool,
    pub no_color: bool,
    pub workspace_root: PathBuf,
}

impl App {
    pub fn new(
        catalog: CatalogStore,
        workspace_root: PathBuf,
        session_id: Uuid,
        no_color: bool,
    ) -> Self {
        let validation_gates = extract_validation_gates(&workspace_root);
        let mut search_engine = SearchEngine::new();
        let filtered_indices = search_engine.search_agents("", &catalog.agents, None, None);
        Self {
            nav: NavigationState::new(),
            catalog,
            search_engine,
            search_query: String::new(),
            search_active: false,
            filtered_indices,
            subprocess_output: Vec::new(),
            validation_gates,
            export_state: ExportBuilderState::new(),
            status_message: None,
            session_id,
            should_quit: false,
            no_color,
            workspace_root,
        }
    }

    /// Handle a key event. Dispatches based on search mode and current view.
    pub fn handle_key_event(&mut self, key: KeyEvent) {
        if self.search_active {
            self.handle_search_key(key);
            return;
        }
        match key.code {
            KeyCode::Char('q') => self.should_quit = true,
            KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                self.should_quit = true;
            }
            KeyCode::Esc => {
                if !self.nav.pop_view() {
                    self.should_quit = true;
                }
                self.update_filtered();
            }
            KeyCode::Tab => {
                let next = (self.nav.sidebar_index + 1) % SIDEBAR_SECTIONS.len();
                self.nav.set_sidebar_index(next);
                self.search_query.clear();
                self.update_filtered();
            }
            KeyCode::BackTab => {
                let prev = if self.nav.sidebar_index == 0 {
                    SIDEBAR_SECTIONS.len() - 1
                } else {
                    self.nav.sidebar_index - 1
                };
                self.nav.set_sidebar_index(prev);
                self.search_query.clear();
                self.update_filtered();
            }
            KeyCode::Char('j') | KeyCode::Down => self.handle_down(),
            KeyCode::Char('k') | KeyCode::Up => self.handle_up(),
            KeyCode::Char('g') => self.nav.select_first(),
            KeyCode::Char('G') => {
                let max = self.current_list_len();
                self.nav.select_last(max);
            }
            KeyCode::Char('/') => {
                self.search_active = true;
                self.search_query.clear();
            }
            KeyCode::Enter => self.handle_enter(),
            _ => {}
        }
    }

    /// Handle key events during active search mode.
    fn handle_search_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Esc => {
                self.search_active = false;
            }
            KeyCode::Enter => {
                self.search_active = false;
                self.update_filtered();
            }
            KeyCode::Backspace => {
                self.search_query.pop();
                self.update_filtered();
            }
            KeyCode::Char(c) => {
                self.search_query.push(c);
                self.update_filtered();
            }
            _ => {}
        }
    }

    fn handle_down(&mut self) {
        match &self.nav.current_view {
            View::AgentDetail(_)
            | View::SkillDetail(_)
            | View::McpDetail(_)
            | View::RuleDetail(_)
            | View::IntegrityDetail(_)
            | View::RoleDetail(_) => {
                self.nav.detail_scroll = self.nav.detail_scroll.saturating_add(1);
            }
            _ => {
                let max = self.current_list_len();
                self.nav.select_next(max);
            }
        }
    }

    fn handle_up(&mut self) {
        match &self.nav.current_view {
            View::AgentDetail(_)
            | View::SkillDetail(_)
            | View::McpDetail(_)
            | View::RuleDetail(_)
            | View::IntegrityDetail(_)
            | View::RoleDetail(_) => {
                self.nav.detail_scroll = self.nav.detail_scroll.saturating_sub(1);
            }
            _ => self.nav.select_prev(),
        }
    }

    fn handle_enter(&mut self) {
        let idx = self.nav.selected_index();
        match self.nav.current_view.clone() {
            View::AgentList => {
                if let Some(&real_idx) = self.filtered_indices.get(idx) {
                    if let Some(agent) = self.catalog.agents.get(real_idx) {
                        self.nav.push_view(View::AgentDetail(agent.id.clone()));
                    }
                }
            }
            View::SkillList => {
                if let Some(skill) = self.catalog.skills.get(idx) {
                    self.nav.push_view(View::SkillDetail(skill.id.clone()));
                }
            }
            View::RoleList => {
                let roles: Vec<String> = self.get_role_list().iter().map(|r| r.0.clone()).collect();
                if let Some(role_id) = roles.get(idx) {
                    self.nav.push_view(View::RoleDetail(role_id.clone()));
                }
            }
            View::ProviderList => {
                let providers = self.get_provider_list();
                if let Some((provider, _)) = providers.get(idx) {
                    self.nav.push_view(View::ProviderAgents(provider.clone()));
                }
            }
            View::McpList => {
                if let Some(mcp) = self.catalog.mcp_refs.get(idx) {
                    self.nav.push_view(View::McpDetail(mcp.id.clone()));
                }
            }
            View::RuleList => {
                if let Some(rule) = self.catalog.rules.get(idx) {
                    self.nav.push_view(View::RuleDetail(rule.id.clone()));
                }
            }
            View::ProviderAgents(ref provider) => {
                let agents = self.catalog.agents_by_provider(provider);
                if let Some(agent) = agents.get(idx) {
                    self.nav.push_view(View::AgentDetail(agent.id.clone()));
                }
            }
            View::IntegrityOverview => {
                if let Some(integrity) = &self.catalog.integrity {
                    if let Some(tree) = integrity.trees.get(idx) {
                        self.nav.push_view(View::IntegrityDetail(tree.tree.clone()));
                    }
                }
            }
            _ => {}
        }
    }

    /// Tick: clear expired status messages.
    pub fn tick(&mut self) {
        if let Some((_, created)) = &self.status_message {
            if created.elapsed().as_secs() > 10 {
                self.status_message = None;
            }
        }
    }

    /// Render the full UI.
    pub fn render(&mut self, frame: &mut Frame) {
        let theme = Theme::new(self.no_color);
        let layout = compute_layout(frame.area());

        self.render_sidebar(&layout.sidebar, frame, &theme);
        self.render_main_content(&layout.main_content, frame, &theme);

        let (visible, total) = self.get_counts();
        let filter_str = if self.search_query.is_empty() {
            String::new()
        } else {
            self.search_query.clone()
        };
        let session_str = self.session_id.to_string();
        status_bar::render_status_bar(
            visible,
            total,
            &filter_str,
            &session_str,
            layout.status_bar,
            frame,
            &theme,
        );

        help_bar::render_help_bar(&self.nav.current_view, layout.help_bar, frame, &theme);
    }

    fn render_sidebar(&mut self, area: &ratatui::layout::Rect, frame: &mut Frame, theme: &Theme) {
        let items: Vec<String> = SIDEBAR_SECTIONS
            .iter()
            .map(|(s, _)| s.to_string())
            .collect();
        let mut sidebar_state = ratatui::widgets::ListState::default();
        sidebar_state.select(Some(self.nav.sidebar_index));
        list_view::render_list_view(&items, &mut sidebar_state, "Catalog", *area, frame, theme);
    }

    fn render_main_content(
        &mut self,
        area: &ratatui::layout::Rect,
        frame: &mut Frame,
        theme: &Theme,
    ) {
        match self.nav.current_view.clone() {
            View::AgentList => self.render_agent_list(*area, frame, theme),
            View::AgentDetail(ref id) => self.render_agent_detail_view(id, *area, frame, theme),
            View::SkillList => self.render_skill_list(*area, frame, theme),
            View::SkillDetail(ref id) => self.render_skill_detail_view(id, *area, frame, theme),
            View::RoleList => self.render_role_list(*area, frame, theme),
            View::RoleDetail(ref id) => self.render_role_detail_view(id, *area, frame, theme),
            View::ProviderList => self.render_provider_list(*area, frame, theme),
            View::ProviderAgents(ref p) => self.render_provider_agents(p, *area, frame, theme),
            View::McpList => self.render_mcp_list(*area, frame, theme),
            View::McpDetail(ref id) => self.render_mcp_detail_view(id, *area, frame, theme),
            View::RuleList => self.render_rule_list(*area, frame, theme),
            View::RuleDetail(ref id) => self.render_rule_detail_view(id, *area, frame, theme),
            View::ValidationList => self.render_validation_list(*area, frame, theme),
            View::ValidationOutput(ref name) => {
                let name = name.clone();
                self.render_validation_output(&name, *area, frame, theme);
            }
            View::ExportBuilder => self.render_export_builder(*area, frame, theme),
            View::ExportConfirm => self.render_export_confirm(*area, frame, theme),
            View::ExportOutput => self.render_export_output(*area, frame, theme),
            View::IntegrityOverview => self.render_integrity_view(*area, frame, theme),
            View::IntegrityDetail(ref tree) => {
                let tree = tree.clone();
                self.render_integrity_detail(&tree, *area, frame, theme);
            }
        }

        if self.search_active || !self.search_query.is_empty() {
            let search_area = ratatui::layout::Rect {
                x: area.x,
                y: area.y,
                width: area.width.min(40),
                height: 3,
            };
            search::render_search_input(
                &self.search_query,
                self.search_active,
                search_area,
                frame,
                theme,
            );
        }
    }

    fn render_agent_list(&mut self, area: ratatui::layout::Rect, frame: &mut Frame, theme: &Theme) {
        let items: Vec<String> = self
            .filtered_indices
            .iter()
            .filter_map(|&i| {
                self.catalog.agents.get(i).map(|a| {
                    format!(
                        "{} - {}",
                        a.id,
                        a.summary.chars().take(60).collect::<String>()
                    )
                })
            })
            .collect();
        list_view::render_list_view(
            &items,
            &mut self.nav.list_state,
            "Agents",
            area,
            frame,
            theme,
        );
    }

    fn render_agent_detail_view(
        &self,
        id: &str,
        area: ratatui::layout::Rect,
        frame: &mut Frame,
        theme: &Theme,
    ) {
        if let Some(agent) = self.catalog.agents.iter().find(|a| a.id == id) {
            detail::render_agent_detail(agent, area, frame, self.nav.detail_scroll, theme);
        }
    }

    fn render_skill_list(&mut self, area: ratatui::layout::Rect, frame: &mut Frame, theme: &Theme) {
        let items: Vec<String> = self
            .catalog
            .skills
            .iter()
            .map(|s| {
                format!(
                    "{} - {}",
                    s.id,
                    s.summary.chars().take(60).collect::<String>()
                )
            })
            .collect();
        list_view::render_list_view(
            &items,
            &mut self.nav.list_state,
            "Skills",
            area,
            frame,
            theme,
        );
    }

    fn render_skill_detail_view(
        &self,
        id: &str,
        area: ratatui::layout::Rect,
        frame: &mut Frame,
        theme: &Theme,
    ) {
        if let Some(skill) = self.catalog.skills.iter().find(|s| s.id == id) {
            let related = self.catalog.agents_with_skill(id);
            detail::render_skill_detail(
                skill,
                &related,
                area,
                frame,
                self.nav.detail_scroll,
                theme,
            );
        }
    }

    fn render_role_list(&mut self, area: ratatui::layout::Rect, frame: &mut Frame, theme: &Theme) {
        let roles = self.get_role_list();
        let items: Vec<String> = roles
            .iter()
            .map(|(id, label, count)| format!("{id} ({label}) - {count} agents"))
            .collect();
        list_view::render_list_view(
            &items,
            &mut self.nav.list_state,
            "Roles",
            area,
            frame,
            theme,
        );
    }

    fn render_role_detail_view(
        &self,
        id: &str,
        area: ratatui::layout::Rect,
        frame: &mut Frame,
        theme: &Theme,
    ) {
        let agents = self.catalog.agents_for_role(id);
        let items: Vec<ratatui::text::Line> = agents
            .iter()
            .map(|a| ratatui::text::Line::from(format!("  - {}", a.id)))
            .collect();
        let mut lines = vec![ratatui::text::Line::from(format!("Role: {id}"))];
        if let Some(role) = self.catalog.roles.get(id) {
            lines.push(ratatui::text::Line::from(format!("Label: {}", role.label)));
            lines.push(ratatui::text::Line::from(format!(
                "Description: {}",
                role.description
            )));
        }
        lines.push(ratatui::text::Line::from(format!(
            "Agents ({}): ",
            agents.len()
        )));
        lines.extend(items);
        let paragraph = ratatui::widgets::Paragraph::new(lines)
            .block(
                ratatui::widgets::Block::default()
                    .borders(ratatui::widgets::Borders::ALL)
                    .title("Role Detail")
                    .border_style(theme.border_style()),
            )
            .wrap(ratatui::widgets::Wrap { trim: false })
            .scroll((self.nav.detail_scroll, 0));
        frame.render_widget(paragraph, area);
    }

    fn render_provider_list(
        &mut self,
        area: ratatui::layout::Rect,
        frame: &mut Frame,
        theme: &Theme,
    ) {
        let providers = self.get_provider_list();
        let items: Vec<String> = providers
            .iter()
            .map(|(p, count)| format!("{p} ({count} agents)"))
            .collect();
        list_view::render_list_view(
            &items,
            &mut self.nav.list_state,
            "Providers",
            area,
            frame,
            theme,
        );
    }

    fn render_provider_agents(
        &mut self,
        provider: &str,
        area: ratatui::layout::Rect,
        frame: &mut Frame,
        theme: &Theme,
    ) {
        let agents = self.catalog.agents_by_provider(provider);
        let items: Vec<String> = agents
            .iter()
            .map(|a| {
                format!(
                    "{} - {}",
                    a.id,
                    a.summary.chars().take(60).collect::<String>()
                )
            })
            .collect();
        let title = format!("Agents: {provider}");
        list_view::render_list_view(&items, &mut self.nav.list_state, &title, area, frame, theme);
    }

    fn render_mcp_list(&mut self, area: ratatui::layout::Rect, frame: &mut Frame, theme: &Theme) {
        let items: Vec<String> = self
            .catalog
            .mcp_refs
            .iter()
            .map(|m| {
                format!(
                    "{} - {}",
                    m.id,
                    m.summary.chars().take(60).collect::<String>()
                )
            })
            .collect();
        list_view::render_list_view(
            &items,
            &mut self.nav.list_state,
            "MCP References",
            area,
            frame,
            theme,
        );
    }

    fn render_mcp_detail_view(
        &self,
        id: &str,
        area: ratatui::layout::Rect,
        frame: &mut Frame,
        theme: &Theme,
    ) {
        if let Some(mcp) = self.catalog.mcp_refs.iter().find(|m| m.id == id) {
            detail::render_mcp_detail(mcp, area, frame, self.nav.detail_scroll, theme);
        }
    }

    fn render_rule_list(&mut self, area: ratatui::layout::Rect, frame: &mut Frame, theme: &Theme) {
        let items: Vec<String> = self
            .catalog
            .rules
            .iter()
            .map(|r| {
                format!(
                    "{} - {}",
                    r.id,
                    r.summary.chars().take(60).collect::<String>()
                )
            })
            .collect();
        list_view::render_list_view(
            &items,
            &mut self.nav.list_state,
            "Rules",
            area,
            frame,
            theme,
        );
    }

    fn render_rule_detail_view(
        &self,
        id: &str,
        area: ratatui::layout::Rect,
        frame: &mut Frame,
        theme: &Theme,
    ) {
        if let Some(rule) = self.catalog.rules.iter().find(|r| r.id == id) {
            detail::render_rule_detail(rule, area, frame, self.nav.detail_scroll, theme);
        }
    }

    fn render_validation_list(
        &mut self,
        area: ratatui::layout::Rect,
        frame: &mut Frame,
        theme: &Theme,
    ) {
        let items: Vec<String> = self
            .validation_gates
            .iter()
            .map(|g| format!("{} [{:?}]", g.script_name, g.status))
            .collect();
        list_view::render_list_view(
            &items,
            &mut self.nav.list_state,
            "Validation Gates",
            area,
            frame,
            theme,
        );
    }

    fn render_validation_output(
        &self,
        _name: &str,
        area: ratatui::layout::Rect,
        frame: &mut Frame,
        theme: &Theme,
    ) {
        output::render_output(
            &self.subprocess_output,
            "Validation Output",
            area,
            frame,
            theme,
        );
    }

    fn render_export_builder(&self, area: ratatui::layout::Rect, frame: &mut Frame, theme: &Theme) {
        let selection_str = match &self.export_state.selection {
            ExportSelection::All => "All".to_string(),
            ExportSelection::Role(r) => format!("Role: {r}"),
            ExportSelection::Provider(p) => format!("Provider: {p}"),
            ExportSelection::Agents(ids) => format!("Agents: {}", ids.join(", ")),
        };
        let lines = vec![
            ratatui::text::Line::from(format!("Platform: {}", self.export_state.platform)),
            ratatui::text::Line::from(format!("Selection: {selection_str}")),
            ratatui::text::Line::from(format!("Target Repo: {}", self.export_state.target_repo)),
            ratatui::text::Line::from(format!("Dry Run: {}", self.export_state.dry_run)),
            ratatui::text::Line::from(format!("Force: {}", self.export_state.force)),
            ratatui::text::Line::from(format!("No Skills: {}", self.export_state.no_skills)),
            ratatui::text::Line::from(""),
            ratatui::text::Line::from("[Enter to confirm, Esc to cancel]"),
        ];
        let paragraph = ratatui::widgets::Paragraph::new(lines)
            .block(
                ratatui::widgets::Block::default()
                    .borders(ratatui::widgets::Borders::ALL)
                    .title("Export Builder")
                    .border_style(theme.border_style()),
            )
            .wrap(ratatui::widgets::Wrap { trim: false });
        frame.render_widget(paragraph, area);
    }

    fn render_export_confirm(&self, area: ratatui::layout::Rect, frame: &mut Frame, theme: &Theme) {
        let lines = vec![
            ratatui::text::Line::from("Confirm export execution?"),
            ratatui::text::Line::from(""),
            ratatui::text::Line::from("[Enter to execute, Esc to cancel]"),
        ];
        let paragraph = ratatui::widgets::Paragraph::new(lines)
            .block(
                ratatui::widgets::Block::default()
                    .borders(ratatui::widgets::Borders::ALL)
                    .title("Confirm Export")
                    .border_style(theme.border_style()),
            )
            .wrap(ratatui::widgets::Wrap { trim: false });
        frame.render_widget(paragraph, area);
    }

    fn render_export_output(&self, area: ratatui::layout::Rect, frame: &mut Frame, theme: &Theme) {
        output::render_output(&self.subprocess_output, "Export Output", area, frame, theme);
    }

    fn render_integrity_view(
        &mut self,
        area: ratatui::layout::Rect,
        frame: &mut Frame,
        theme: &Theme,
    ) {
        let items: Vec<String> = match &self.catalog.integrity {
            Some(integrity) => integrity
                .trees
                .iter()
                .map(|t| {
                    format!(
                        "{} ({} files, SHA: {}...)",
                        t.tree,
                        t.files.len(),
                        &t.aggregate_sha256[..8.min(t.aggregate_sha256.len())]
                    )
                })
                .collect(),
            None => vec!["No integrity data available".to_string()],
        };
        list_view::render_list_view(
            &items,
            &mut self.nav.list_state,
            "Asset Integrity",
            area,
            frame,
            theme,
        );
    }

    fn render_integrity_detail(
        &self,
        tree_name: &str,
        area: ratatui::layout::Rect,
        frame: &mut Frame,
        theme: &Theme,
    ) {
        let lines: Vec<ratatui::text::Line> = if let Some(integrity) = &self.catalog.integrity {
            if let Some(tree) = integrity.trees.iter().find(|t| t.tree == tree_name) {
                let mut l = vec![
                    ratatui::text::Line::from(format!("Tree: {}", tree.tree)),
                    ratatui::text::Line::from(format!("SHA-256: {}", tree.aggregate_sha256)),
                    ratatui::text::Line::from(format!("Files: {}", tree.files.len())),
                    ratatui::text::Line::from(""),
                ];
                for f in &tree.files {
                    l.push(ratatui::text::Line::from(format!(
                        "  {} ({} bytes)",
                        f.path, f.bytes
                    )));
                }
                l
            } else {
                vec![ratatui::text::Line::from("Tree not found")]
            }
        } else {
            vec![ratatui::text::Line::from("No integrity data")]
        };
        let paragraph = ratatui::widgets::Paragraph::new(lines)
            .block(
                ratatui::widgets::Block::default()
                    .borders(ratatui::widgets::Borders::ALL)
                    .title("Integrity Detail")
                    .border_style(theme.border_style()),
            )
            .wrap(ratatui::widgets::Wrap { trim: false })
            .scroll((self.nav.detail_scroll, 0));
        frame.render_widget(paragraph, area);
    }

    /// Update filtered indices based on current view and search query.
    fn update_filtered(&mut self) {
        if self.nav.current_view == View::AgentList {
            self.filtered_indices = self.search_engine.search_agents(
                &self.search_query,
                &self.catalog.agents,
                None,
                None,
            );
        }
    }

    /// Get the length of the current list for navigation.
    fn current_list_len(&self) -> usize {
        match &self.nav.current_view {
            View::AgentList => self.filtered_indices.len(),
            View::SkillList => self.catalog.skills.len(),
            View::RoleList => self.catalog.roles.len(),
            View::ProviderList => self.get_provider_list().len(),
            View::McpList => self.catalog.mcp_refs.len(),
            View::RuleList => self.catalog.rules.len(),
            View::ValidationList => self.validation_gates.len(),
            View::ProviderAgents(p) => self.catalog.agents_by_provider(p).len(),
            View::IntegrityOverview => self
                .catalog
                .integrity
                .as_ref()
                .map(|i| i.trees.len())
                .unwrap_or(0),
            _ => 0,
        }
    }

    fn get_counts(&self) -> (usize, usize) {
        match &self.nav.current_view {
            View::AgentList => (self.filtered_indices.len(), self.catalog.agents.len()),
            View::SkillList => (self.catalog.skills.len(), self.catalog.skills.len()),
            View::McpList => (self.catalog.mcp_refs.len(), self.catalog.mcp_refs.len()),
            View::RuleList => (self.catalog.rules.len(), self.catalog.rules.len()),
            _ => (0, 0),
        }
    }

    /// Get provider list with counts.
    pub fn get_provider_list(&self) -> Vec<(String, usize)> {
        let mut providers: HashSet<String> = HashSet::new();
        for agent in &self.catalog.agents {
            let p = serde_json::to_value(agent.provider)
                .ok()
                .and_then(|v| v.as_str().map(|s| s.to_string()))
                .unwrap_or_else(|| format!("{:?}", agent.provider));
            providers.insert(p);
        }
        let mut list: Vec<(String, usize)> = providers
            .into_iter()
            .map(|p| {
                let count = self.catalog.agents_by_provider(&p).len();
                (p, count)
            })
            .collect();
        list.sort_by(|a, b| a.0.cmp(&b.0));
        list
    }

    /// Get role list with labels and agent counts.
    pub fn get_role_list(&self) -> Vec<(String, String, usize)> {
        let mut list: Vec<(String, String, usize)> = self
            .catalog
            .roles
            .iter()
            .map(|(id, role)| (id.clone(), role.label.clone(), role.agents.len()))
            .collect();
        list.sort_by(|a, b| a.0.cmp(&b.0));
        list
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crossterm::event::{KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers};
    use std::path::Path;

    fn workspace_root() -> PathBuf {
        let manifest_dir = env!("CARGO_MANIFEST_DIR");
        Path::new(manifest_dir)
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .to_path_buf()
    }

    fn make_app() -> App {
        let ws = workspace_root();
        let catalog = CatalogStore::load(&ws);
        App::new(catalog, ws, Uuid::new_v4(), true)
    }

    fn key_event(code: KeyCode) -> KeyEvent {
        KeyEvent {
            code,
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        }
    }

    #[test]
    fn app_new_starts_at_agent_list() {
        let app = make_app();
        assert_eq!(app.nav.current_view, View::AgentList);
        assert!(!app.should_quit);
        assert!(!app.filtered_indices.is_empty());
    }

    #[test]
    fn app_quit_on_q() {
        let mut app = make_app();
        app.handle_key_event(key_event(KeyCode::Char('q')));
        assert!(app.should_quit);
    }

    #[test]
    fn app_quit_on_ctrl_c() {
        let mut app = make_app();
        let key = KeyEvent {
            code: KeyCode::Char('c'),
            modifiers: KeyModifiers::CONTROL,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        };
        app.handle_key_event(key);
        assert!(app.should_quit);
    }

    #[test]
    fn app_tab_switches_section() {
        let mut app = make_app();
        assert_eq!(app.nav.sidebar_index, 0);
        app.handle_key_event(key_event(KeyCode::Tab));
        assert_eq!(app.nav.sidebar_index, 1);
        assert_eq!(app.nav.current_view, View::SkillList);
    }

    #[test]
    fn app_backtab_wraps_around() {
        let mut app = make_app();
        let key = KeyEvent {
            code: KeyCode::BackTab,
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: KeyEventState::NONE,
        };
        app.handle_key_event(key);
        assert_eq!(app.nav.sidebar_index, SIDEBAR_SECTIONS.len() - 1);
    }

    #[test]
    fn app_j_navigates_down() {
        let mut app = make_app();
        assert_eq!(app.nav.selected_index(), 0);
        app.handle_key_event(key_event(KeyCode::Char('j')));
        assert_eq!(app.nav.selected_index(), 1);
    }

    #[test]
    fn app_k_navigates_up() {
        let mut app = make_app();
        app.handle_key_event(key_event(KeyCode::Char('j')));
        app.handle_key_event(key_event(KeyCode::Char('k')));
        assert_eq!(app.nav.selected_index(), 0);
    }

    #[test]
    fn app_enter_drills_into_agent_detail() {
        let mut app = make_app();
        app.handle_key_event(key_event(KeyCode::Enter));
        assert!(matches!(app.nav.current_view, View::AgentDetail(_)));
    }

    #[test]
    fn app_escape_goes_back() {
        let mut app = make_app();
        app.handle_key_event(key_event(KeyCode::Enter));
        assert!(matches!(app.nav.current_view, View::AgentDetail(_)));
        app.handle_key_event(key_event(KeyCode::Esc));
        assert_eq!(app.nav.current_view, View::AgentList);
    }

    #[test]
    fn app_search_activation() {
        let mut app = make_app();
        app.handle_key_event(key_event(KeyCode::Char('/')));
        assert!(app.search_active);
        assert!(app.search_query.is_empty());
    }

    #[test]
    fn app_search_typing() {
        let mut app = make_app();
        app.handle_key_event(key_event(KeyCode::Char('/')));
        app.handle_key_event(key_event(KeyCode::Char('a')));
        app.handle_key_event(key_event(KeyCode::Char('w')));
        app.handle_key_event(key_event(KeyCode::Char('s')));
        assert_eq!(app.search_query, "aws");
    }

    #[test]
    fn app_search_backspace() {
        let mut app = make_app();
        app.handle_key_event(key_event(KeyCode::Char('/')));
        app.handle_key_event(key_event(KeyCode::Char('x')));
        app.handle_key_event(key_event(KeyCode::Backspace));
        assert!(app.search_query.is_empty());
    }

    #[test]
    fn app_search_escape_deactivates() {
        let mut app = make_app();
        app.handle_key_event(key_event(KeyCode::Char('/')));
        app.handle_key_event(key_event(KeyCode::Esc));
        assert!(!app.search_active);
    }

    #[test]
    fn app_g_goes_to_top() {
        let mut app = make_app();
        app.handle_key_event(key_event(KeyCode::Char('j')));
        app.handle_key_event(key_event(KeyCode::Char('j')));
        app.handle_key_event(key_event(KeyCode::Char('g')));
        assert_eq!(app.nav.selected_index(), 0);
    }

    #[test]
    fn app_big_g_goes_to_bottom() {
        let mut app = make_app();
        app.handle_key_event(key_event(KeyCode::Char('G')));
        let max = app.current_list_len();
        assert_eq!(app.nav.selected_index(), max - 1);
    }

    #[test]
    fn app_empty_catalog_does_not_panic() {
        let catalog = CatalogStore {
            agents: Vec::new(),
            skills: Vec::new(),
            roles: std::collections::HashMap::new(),
            role_catalog_version: String::new(),
            role_catalog_description: String::new(),
            mcp_refs: Vec::new(),
            rules: Vec::new(),
            integrity: None,
            load_errors: Vec::new(),
        };
        let app = App::new(catalog, PathBuf::from("/tmp"), Uuid::new_v4(), true);
        assert!(app.filtered_indices.is_empty());
    }

    #[test]
    fn app_tick_clears_old_messages() {
        let mut app = make_app();
        app.status_message = Some((
            "test".to_string(),
            Instant::now() - std::time::Duration::from_secs(20),
        ));
        app.tick();
        assert!(app.status_message.is_none());
    }

    #[test]
    fn app_tick_keeps_recent_messages() {
        let mut app = make_app();
        app.status_message = Some(("test".to_string(), Instant::now()));
        app.tick();
        assert!(app.status_message.is_some());
    }

    #[test]
    fn app_get_provider_list_non_empty() {
        let app = make_app();
        let providers = app.get_provider_list();
        assert!(!providers.is_empty());
    }

    #[test]
    fn app_get_role_list_non_empty() {
        let app = make_app();
        let roles = app.get_role_list();
        assert!(!roles.is_empty());
    }
}
