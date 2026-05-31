use std::collections::HashMap;

use proptest::prelude::*;
use ratatui::{backend::TestBackend, layout::Rect, Terminal};
use vfa_tui::models::{Agent, AgentType, ExecutionTier, Harness, Lifecycle, Provider, SourceType};
use vfa_tui::ui::theme::Theme;
use vfa_tui::ui::widgets::detail::{render_agent_detail, AGENT_DETAIL_REQUIRED_LABELS};

fn arbitrary_agent(
    id: String,
    name: String,
    summary: String,
    version: Option<String>,
    author: Option<String>,
    execution_tier: Option<ExecutionTier>,
    lifecycle: Option<Lifecycle>,
) -> Agent {
    Agent {
        id,
        name,
        entity_type: AgentType::Agent,
        provider: Provider::Aws,
        harnesses: vec![Harness::Kiro],
        summary,
        companion_skills: vec!["skill-a".to_string()],
        source_type: SourceType::Original,
        official_docs: vec!["https://example.com".to_string()],
        security_notes: "none".to_string(),
        last_verified: "2024-01-01".to_string(),
        path: "agents/test".to_string(),
        harness_variants: HashMap::new(),
        author,
        version,
        execution_tier,
        lifecycle,
        provider_coverage: None,
    }
}

fn option_string() -> impl Strategy<Value = Option<String>> {
    prop_oneof![Just(None), "[a-z]{3,10}".prop_map(Some),]
}

fn option_execution_tier() -> impl Strategy<Value = Option<ExecutionTier>> {
    prop_oneof![
        Just(None),
        Just(Some(ExecutionTier::StaticReview)),
        Just(Some(ExecutionTier::ReadOnlyRuntime)),
        Just(Some(ExecutionTier::MutatingRuntime)),
    ]
}

fn option_lifecycle() -> impl Strategy<Value = Option<Lifecycle>> {
    prop_oneof![
        Just(None),
        Just(Some(Lifecycle::Experimental)),
        Just(Some(Lifecycle::Beta)),
        Just(Some(Lifecycle::Stable)),
        Just(Some(Lifecycle::Deprecated)),
    ]
}

proptest! {
    // Property 4: For any Agent struct, the detail renderer output contains labels
    // for ALL required fields. None fields render as "N/A".
    #[test]
    fn agent_detail_contains_all_required_labels(
        id in "[a-z][a-z0-9-]{2,20}",
        name in "[A-Z][a-zA-Z ]{2,20}",
        summary in "[a-zA-Z ]{5,50}",
        version in option_string(),
        author in option_string(),
        execution_tier in option_execution_tier(),
        lifecycle in option_lifecycle(),
    ) {
        let agent = arbitrary_agent(id, name, summary, version.clone(), author.clone(), execution_tier, lifecycle);

        let backend = TestBackend::new(120, 40);
        let mut terminal = Terminal::new(backend).unwrap();
        let theme = Theme::new(false);

        terminal.draw(|frame| {
            let area = Rect::new(0, 0, 120, 40);
            render_agent_detail(&agent, &[], area, frame, 0, &theme);
        }).unwrap();

        let buffer = terminal.backend().buffer().clone();
        let mut text = String::new();
        for y in 0..buffer.area.height {
            for x in 0..buffer.area.width {
                let cell = &buffer[(x, y)];
                text.push_str(cell.symbol());
            }
            text.push('\n');
        }

        // All required labels must appear
        for label in AGENT_DETAIL_REQUIRED_LABELS {
            prop_assert!(
                text.contains(&format!("{label}:")),
                "Missing label '{label}:' in rendered output"
            );
        }

        // Optional fields that are None should show N/A
        if version.is_none() {
            prop_assert!(text.contains("N/A"), "None version should show N/A");
        }
        if author.is_none() {
            prop_assert!(text.contains("N/A"), "None author should show N/A");
        }
    }

    // Property 17: Rendering is deterministic - same input produces same output.
    #[test]
    fn rendering_is_deterministic(
        id in "[a-z][a-z0-9-]{2,10}",
        name in "[A-Z][a-zA-Z]{2,10}",
        summary in "[a-zA-Z ]{5,30}",
    ) {
        let agent = arbitrary_agent(
            id, name, summary, Some("1.0".to_string()), Some("author".to_string()), None, None
        );

        let theme = Theme::new(true);

        let render_once = |agent: &Agent| -> String {
            let backend = TestBackend::new(80, 30);
            let mut terminal = Terminal::new(backend).unwrap();
            terminal.draw(|frame| {
                let area = Rect::new(0, 0, 80, 30);
                render_agent_detail(agent, &[], area, frame, 0, &theme);
            }).unwrap();
            let buffer = terminal.backend().buffer().clone();
            let mut text = String::new();
            for y in 0..buffer.area.height {
                for x in 0..buffer.area.width {
                    let cell = &buffer[(x, y)];
                    text.push_str(cell.symbol());
                }
                text.push('\n');
            }
            text
        };

        let output1 = render_once(&agent);
        let output2 = render_once(&agent);
        prop_assert_eq!(output1, output2, "Rendering should be deterministic");
    }
}
