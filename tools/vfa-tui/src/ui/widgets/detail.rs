use ratatui::{
    layout::Rect,
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};

use crate::models::{Agent, McpReference, Rule, Skill};

use super::super::theme::Theme;

fn format_option(opt: &Option<String>) -> String {
    match opt {
        Some(s) => s.clone(),
        None => "N/A".to_string(),
    }
}

fn format_vec(v: &[String]) -> String {
    if v.is_empty() {
        "N/A".to_string()
    } else {
        v.join(", ")
    }
}

fn detail_line(key: &str, value: &str, theme: &Theme) -> Line<'static> {
    Line::from(vec![
        Span::styled(format!("{key}: "), theme.detail_key()),
        Span::styled(value.to_string(), theme.detail_value()),
    ])
}

/// Render agent detail view with all fields.
pub fn render_agent_detail(
    agent: &Agent,
    area: Rect,
    frame: &mut Frame,
    scroll: u16,
    theme: &Theme,
) {
    let harnesses_str = agent
        .harnesses
        .iter()
        .map(|h| {
            serde_json::to_value(h)
                .ok()
                .and_then(|v| v.as_str().map(|s| s.to_string()))
                .unwrap_or_else(|| format!("{h:?}"))
        })
        .collect::<Vec<_>>()
        .join(", ");

    let provider_str = serde_json::to_value(agent.provider)
        .ok()
        .and_then(|v| v.as_str().map(|s| s.to_string()))
        .unwrap_or_else(|| format!("{:?}", agent.provider));

    let source_str = serde_json::to_value(agent.source_type)
        .ok()
        .and_then(|v| v.as_str().map(|s| s.to_string()))
        .unwrap_or_else(|| format!("{:?}", agent.source_type));

    let docs_str = format_vec(&agent.official_docs);
    let skills_str = format_vec(&agent.companion_skills);
    let variants_str = agent
        .harness_variants
        .as_ref()
        .map(|hv| {
            hv.iter()
                .map(|(k, v)| format!("{k}={v}"))
                .collect::<Vec<_>>()
                .join(", ")
        })
        .unwrap_or_else(|| "N/A".to_string());

    let lines = vec![
        detail_line("ID", &agent.id, theme),
        detail_line("Name", &agent.name, theme),
        detail_line("Type", &agent.entity_type, theme),
        detail_line("Provider", &provider_str, theme),
        detail_line("Harnesses", &harnesses_str, theme),
        detail_line("Summary", &agent.summary, theme),
        detail_line("Source Type", &source_str, theme),
        detail_line("Official Docs", &docs_str, theme),
        detail_line("Security Notes", &agent.security_notes, theme),
        detail_line("Last Verified", &agent.last_verified, theme),
        detail_line("Path", &agent.path, theme),
        detail_line("Companion Skills", &skills_str, theme),
        detail_line("Version", &format_option(&agent.version), theme),
        detail_line("Author", &format_option(&agent.author), theme),
        detail_line(
            "Execution Tier",
            &format_option(&agent.execution_tier),
            theme,
        ),
        detail_line("Lifecycle", &format_option(&agent.lifecycle), theme),
        detail_line("Harness Variants", &variants_str, theme),
    ];

    let paragraph = Paragraph::new(lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(agent.name.clone())
                .border_style(theme.border_style()),
        )
        .wrap(Wrap { trim: false })
        .scroll((scroll, 0));

    frame.render_widget(paragraph, area);
}

/// Render skill detail view with related agents.
pub fn render_skill_detail(
    skill: &Skill,
    related_agents: &[&Agent],
    area: Rect,
    frame: &mut Frame,
    scroll: u16,
    theme: &Theme,
) {
    let harnesses_str = skill
        .harnesses
        .iter()
        .map(|h| {
            serde_json::to_value(h)
                .ok()
                .and_then(|v| v.as_str().map(|s| s.to_string()))
                .unwrap_or_else(|| format!("{h:?}"))
        })
        .collect::<Vec<_>>()
        .join(", ");

    let provider_str = serde_json::to_value(skill.provider)
        .ok()
        .and_then(|v| v.as_str().map(|s| s.to_string()))
        .unwrap_or_else(|| format!("{:?}", skill.provider));

    let source_str = serde_json::to_value(skill.source_type)
        .ok()
        .and_then(|v| v.as_str().map(|s| s.to_string()))
        .unwrap_or_else(|| format!("{:?}", skill.source_type));

    let docs_str = format_vec(&skill.official_docs);

    let related_str = if related_agents.is_empty() {
        "N/A".to_string()
    } else {
        related_agents
            .iter()
            .map(|a| a.id.as_str())
            .collect::<Vec<_>>()
            .join(", ")
    };

    let lines = vec![
        detail_line("ID", &skill.id, theme),
        detail_line("Name", &skill.name, theme),
        detail_line("Type", &skill.entity_type, theme),
        detail_line("Provider", &provider_str, theme),
        detail_line("Harnesses", &harnesses_str, theme),
        detail_line("Summary", &skill.summary, theme),
        detail_line("Source Type", &source_str, theme),
        detail_line("Official Docs", &docs_str, theme),
        detail_line("Security Notes", &skill.security_notes, theme),
        detail_line("Last Verified", &skill.last_verified, theme),
        detail_line("Path", &skill.path, theme),
        detail_line("Author", &format_option(&skill.author), theme),
        detail_line("Version", &format_option(&skill.version), theme),
        detail_line("Category", &format_option(&skill.category), theme),
        detail_line(
            "Execution Tier",
            &format_option(&skill.execution_tier),
            theme,
        ),
        detail_line("Lifecycle", &format_option(&skill.lifecycle), theme),
        detail_line("Related Agents", &related_str, theme),
    ];

    let paragraph = Paragraph::new(lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(skill.name.clone())
                .border_style(theme.border_style()),
        )
        .wrap(Wrap { trim: false })
        .scroll((scroll, 0));

    frame.render_widget(paragraph, area);
}

/// Render MCP reference detail view.
pub fn render_mcp_detail(
    mcp: &McpReference,
    area: Rect,
    frame: &mut Frame,
    scroll: u16,
    theme: &Theme,
) {
    let harnesses_str = mcp
        .harnesses
        .iter()
        .map(|h| {
            serde_json::to_value(h)
                .ok()
                .and_then(|v| v.as_str().map(|s| s.to_string()))
                .unwrap_or_else(|| format!("{h:?}"))
        })
        .collect::<Vec<_>>()
        .join(", ");

    let provider_str = serde_json::to_value(mcp.provider)
        .ok()
        .and_then(|v| v.as_str().map(|s| s.to_string()))
        .unwrap_or_else(|| format!("{:?}", mcp.provider));

    let docs_str = format_vec(&mcp.official_docs);

    let trust_str = mcp
        .trust_matrix
        .as_ref()
        .map(|tm| {
            format!(
                "mutation={}, egress={}, creds={}, signed={}, pin={}",
                tm.mutation_capable,
                tm.requires_egress,
                tm.requires_credentials,
                tm.signed_release,
                tm.pin_strategy
            )
        })
        .unwrap_or_else(|| "N/A".to_string());

    let lines = vec![
        detail_line("ID", &mcp.id, theme),
        detail_line("Name", &mcp.name, theme),
        detail_line("Type", &mcp.entity_type, theme),
        detail_line("Provider", &provider_str, theme),
        detail_line("Harnesses", &harnesses_str, theme),
        detail_line("Summary", &mcp.summary, theme),
        detail_line("Vendor", &mcp.vendor, theme),
        detail_line("Auth Model", &mcp.auth_model, theme),
        detail_line("Project URL", &mcp.official_project_url, theme),
        detail_line("Install Example", &mcp.install_example, theme),
        detail_line("Unofficial Warning", &mcp.unofficial_warning, theme),
        detail_line("Official Docs", &docs_str, theme),
        detail_line("Security Notes", &mcp.security_notes, theme),
        detail_line("Last Verified", &mcp.last_verified, theme),
        detail_line("Path", &mcp.path, theme),
        detail_line("Trust Matrix", &trust_str, theme),
    ];

    let paragraph = Paragraph::new(lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(mcp.name.clone())
                .border_style(theme.border_style()),
        )
        .wrap(Wrap { trim: false })
        .scroll((scroll, 0));

    frame.render_widget(paragraph, area);
}

/// Render rule detail view.
pub fn render_rule_detail(rule: &Rule, area: Rect, frame: &mut Frame, scroll: u16, theme: &Theme) {
    let harnesses_str = rule
        .harnesses
        .iter()
        .map(|h| {
            serde_json::to_value(h)
                .ok()
                .and_then(|v| v.as_str().map(|s| s.to_string()))
                .unwrap_or_else(|| format!("{h:?}"))
        })
        .collect::<Vec<_>>()
        .join(", ");

    let provider_str = serde_json::to_value(rule.provider)
        .ok()
        .and_then(|v| v.as_str().map(|s| s.to_string()))
        .unwrap_or_else(|| format!("{:?}", rule.provider));

    let source_str = serde_json::to_value(rule.source_type)
        .ok()
        .and_then(|v| v.as_str().map(|s| s.to_string()))
        .unwrap_or_else(|| format!("{:?}", rule.source_type));

    let docs_str = format_vec(&rule.official_docs);

    let lines = vec![
        detail_line("ID", &rule.id, theme),
        detail_line("Name", &rule.name, theme),
        detail_line("Type", &rule.entity_type, theme),
        detail_line("Provider", &provider_str, theme),
        detail_line("Harnesses", &harnesses_str, theme),
        detail_line("Summary", &rule.summary, theme),
        detail_line("Source Type", &source_str, theme),
        detail_line("Official Docs", &docs_str, theme),
        detail_line("Security Notes", &rule.security_notes, theme),
        detail_line("Last Verified", &rule.last_verified, theme),
        detail_line("Path", &rule.path, theme),
        detail_line("Author", &format_option(&rule.author), theme),
    ];

    let paragraph = Paragraph::new(lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(rule.name.clone())
                .border_style(theme.border_style()),
        )
        .wrap(Wrap { trim: false })
        .scroll((scroll, 0));

    frame.render_widget(paragraph, area);
}

/// Labels that MUST appear in an agent detail rendering.
pub const AGENT_DETAIL_REQUIRED_LABELS: &[&str] = &[
    "ID",
    "Name",
    "Type",
    "Provider",
    "Harnesses",
    "Summary",
    "Source Type",
    "Official Docs",
    "Security Notes",
    "Last Verified",
    "Path",
    "Companion Skills",
    "Version",
    "Author",
    "Execution Tier",
    "Lifecycle",
    "Harness Variants",
];
