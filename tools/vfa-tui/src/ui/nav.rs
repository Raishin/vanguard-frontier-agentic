use ratatui::widgets::ListState;

/// All possible views in the TUI.
#[derive(Debug, Clone, PartialEq)]
pub enum View {
    AgentList,
    AgentDetail(String),
    SkillList,
    SkillDetail(String),
    RoleList,
    RoleDetail(String),
    ProviderList,
    ProviderAgents(String),
    McpList,
    McpDetail(String),
    RuleList,
    RuleDetail(String),
    ValidationList,
    ValidationOutput(String),
    ExportBuilder,
    ExportConfirm,
    ExportOutput,
    IntegrityOverview,
    IntegrityDetail(String),
}

/// Type alias for sidebar section entries.
pub type SidebarEntry = (&'static str, fn() -> View);

/// Sidebar section definitions mapping labels to their root views.
pub const SIDEBAR_SECTIONS: &[SidebarEntry] = &[
    ("Agents", || View::AgentList),
    ("Skills", || View::SkillList),
    ("Roles", || View::RoleList),
    ("Providers", || View::ProviderList),
    ("MCP References", || View::McpList),
    ("Rules", || View::RuleList),
    ("Validation", || View::ValidationList),
    ("Export", || View::ExportBuilder),
    ("Integrity", || View::IntegrityOverview),
];

/// Navigation state machine managing views, history, and list selections.
pub struct NavigationState {
    pub current_view: View,
    pub history: Vec<View>,
    pub sidebar_index: usize,
    pub list_state: ListState,
    pub detail_scroll: u16,
}

impl NavigationState {
    pub fn new() -> Self {
        let mut list_state = ListState::default();
        list_state.select(Some(0));
        Self {
            current_view: View::AgentList,
            history: Vec::new(),
            sidebar_index: 0,
            list_state,
            detail_scroll: 0,
        }
    }

    /// Push a new view onto the history stack and navigate to it.
    pub fn push_view(&mut self, view: View) {
        self.history.push(self.current_view.clone());
        self.current_view = view;
        self.list_state.select(Some(0));
        self.detail_scroll = 0;
    }

    /// Pop the previous view from history. Returns false if at root.
    pub fn pop_view(&mut self) -> bool {
        if let Some(prev) = self.history.pop() {
            self.current_view = prev;
            self.list_state.select(Some(0));
            self.detail_scroll = 0;
            true
        } else {
            false
        }
    }

    /// Set sidebar index and navigate to the corresponding section view.
    pub fn set_sidebar_index(&mut self, idx: usize) {
        if idx < SIDEBAR_SECTIONS.len() {
            self.sidebar_index = idx;
            let view_fn = SIDEBAR_SECTIONS[idx].1;
            let new_view = view_fn();
            self.history.clear();
            self.current_view = new_view;
            self.list_state.select(Some(0));
            self.detail_scroll = 0;
        }
    }

    /// Move selection down, stopping at the boundary.
    pub fn select_next(&mut self, max: usize) {
        if max == 0 {
            return;
        }
        let current = self.selected_index();
        if current < max.saturating_sub(1) {
            self.list_state.select(Some(current + 1));
        }
    }

    /// Move selection up, stopping at 0.
    pub fn select_prev(&mut self) {
        let current = self.selected_index();
        if current > 0 {
            self.list_state.select(Some(current - 1));
        }
    }

    /// Jump to the first item.
    pub fn select_first(&mut self) {
        self.list_state.select(Some(0));
    }

    /// Jump to the last item.
    pub fn select_last(&mut self, max: usize) {
        if max > 0 {
            self.list_state.select(Some(max - 1));
        }
    }

    /// Get the currently selected index.
    pub fn selected_index(&self) -> usize {
        self.list_state.selected().unwrap_or(0)
    }
}

impl Default for NavigationState {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_starts_at_agent_list() {
        let nav = NavigationState::new();
        assert_eq!(nav.current_view, View::AgentList);
        assert!(nav.history.is_empty());
        assert_eq!(nav.sidebar_index, 0);
        assert_eq!(nav.selected_index(), 0);
    }

    #[test]
    fn push_view_adds_to_history() {
        let mut nav = NavigationState::new();
        nav.push_view(View::AgentDetail("test-agent".to_string()));
        assert_eq!(
            nav.current_view,
            View::AgentDetail("test-agent".to_string())
        );
        assert_eq!(nav.history.len(), 1);
        assert_eq!(nav.history[0], View::AgentList);
    }

    #[test]
    fn pop_view_returns_to_previous() {
        let mut nav = NavigationState::new();
        nav.push_view(View::AgentDetail("test".to_string()));
        assert!(nav.pop_view());
        assert_eq!(nav.current_view, View::AgentList);
        assert!(nav.history.is_empty());
    }

    #[test]
    fn pop_view_at_root_returns_false() {
        let mut nav = NavigationState::new();
        assert!(!nav.pop_view());
        assert_eq!(nav.current_view, View::AgentList);
    }

    #[test]
    fn set_sidebar_index_changes_view() {
        let mut nav = NavigationState::new();
        nav.set_sidebar_index(1);
        assert_eq!(nav.sidebar_index, 1);
        assert_eq!(nav.current_view, View::SkillList);
        assert!(nav.history.is_empty());
    }

    #[test]
    fn set_sidebar_index_clears_history() {
        let mut nav = NavigationState::new();
        nav.push_view(View::AgentDetail("x".to_string()));
        nav.set_sidebar_index(2);
        assert!(nav.history.is_empty());
        assert_eq!(nav.current_view, View::RoleList);
    }

    #[test]
    fn select_next_stops_at_boundary() {
        let mut nav = NavigationState::new();
        nav.select_next(3);
        assert_eq!(nav.selected_index(), 1);
        nav.select_next(3);
        assert_eq!(nav.selected_index(), 2);
        nav.select_next(3);
        assert_eq!(nav.selected_index(), 2); // stays at boundary
    }

    #[test]
    fn select_prev_stops_at_zero() {
        let mut nav = NavigationState::new();
        nav.select_prev();
        assert_eq!(nav.selected_index(), 0); // stays at 0
    }

    #[test]
    fn select_first_and_last() {
        let mut nav = NavigationState::new();
        nav.select_next(10);
        nav.select_next(10);
        nav.select_next(10);
        nav.select_last(10);
        assert_eq!(nav.selected_index(), 9);
        nav.select_first();
        assert_eq!(nav.selected_index(), 0);
    }

    #[test]
    fn select_next_empty_list_is_safe() {
        let mut nav = NavigationState::new();
        nav.select_next(0);
        assert_eq!(nav.selected_index(), 0);
    }

    #[test]
    fn select_last_empty_list_is_safe() {
        let mut nav = NavigationState::new();
        nav.select_last(0);
        assert_eq!(nav.selected_index(), 0);
    }

    #[test]
    fn sidebar_sections_count() {
        assert_eq!(SIDEBAR_SECTIONS.len(), 9);
    }

    #[test]
    fn all_sidebar_views_are_correct() {
        let expected_views = vec![
            View::AgentList,
            View::SkillList,
            View::RoleList,
            View::ProviderList,
            View::McpList,
            View::RuleList,
            View::ValidationList,
            View::ExportBuilder,
            View::IntegrityOverview,
        ];
        for (i, expected) in expected_views.iter().enumerate() {
            let view_fn = SIDEBAR_SECTIONS[i].1;
            assert_eq!(&view_fn(), expected);
        }
    }

    #[test]
    fn push_view_resets_selection() {
        let mut nav = NavigationState::new();
        nav.select_next(10);
        nav.select_next(10);
        assert_eq!(nav.selected_index(), 2);
        nav.push_view(View::SkillList);
        assert_eq!(nav.selected_index(), 0);
    }
}
