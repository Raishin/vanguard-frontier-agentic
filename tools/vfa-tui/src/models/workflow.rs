//! Workflow catalog entries (`catalog/workflows.json`).
//!
//! Workflow scripts live in `.claude/workflows/` as executable JavaScript whose bodies
//! call runtime globals that only exist inside the workflow engine. The TUI never parses
//! those scripts: `scripts/generate-workflow-catalog.mjs` extracts each script's `meta`
//! block into generated catalog JSON, and this module deserializes that — the same
//! read-first contract every other catalog model follows.

use serde::{Deserialize, Serialize};

/// One phase of a workflow, as declared in the script's `meta.phases`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct WorkflowPhase {
    pub title: String,
    /// Short description of what the phase does. Empty when the script omits it.
    #[serde(default)]
    pub detail: String,
    /// Model tier override for the phase (e.g. `haiku`). Empty means the phase
    /// inherits the session model — which for the spec phase is deliberate, since
    /// planning must not be downgraded.
    #[serde(default)]
    pub model: String,
}

impl WorkflowPhase {
    /// Display label for the phase's model tier.
    ///
    /// An empty `model` is not missing data — it means the phase inherits the session
    /// model, so it renders as `inherit` rather than as a blank cell.
    pub fn model_label(&self) -> &str {
        if self.model.is_empty() {
            "inherit"
        } else {
            &self.model
        }
    }
}

/// A workflow discovered in `.claude/workflows/`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Workflow {
    pub id: String,
    pub name: String,
    /// Repo-relative path to the workflow script.
    pub path: String,
    pub description: String,
    /// Guidance on when to reach for this workflow. Empty when the script omits it.
    #[serde(default)]
    pub when_to_use: String,
    #[serde(default)]
    pub phases: Vec<WorkflowPhase>,
}

impl Workflow {
    /// The invocation a user types to run this workflow.
    pub fn invocation(&self) -> String {
        format!("/{}", self.name)
    }

    /// Number of declared phases.
    pub fn phase_count(&self) -> usize {
        self.phases.len()
    }

    /// Distinct model tiers this workflow spans, in phase order, first occurrence kept.
    ///
    /// Useful as a one-line cost signal in a list view: a workflow that is entirely
    /// `haiku` reads very differently from one that reaches the session model.
    pub fn model_tiers(&self) -> Vec<&str> {
        let mut seen: Vec<&str> = Vec::new();
        for phase in &self.phases {
            let label = phase.model_label();
            if !seen.contains(&label) {
                seen.push(label);
            }
        }
        seen
    }
}

/// The `catalog/workflows.json` document.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct WorkflowCatalog {
    pub version: String,
    pub description: String,
    pub workflows: Vec<Workflow>,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> WorkflowCatalog {
        WorkflowCatalog {
            version: "0.1.0".into(),
            description: "generated".into(),
            workflows: vec![Workflow {
                id: "agentic-delegation".into(),
                name: "agentic-delegation".into(),
                path: ".claude/workflows/agentic-delegation.js".into(),
                description: "Context7-grounded delegation".into(),
                when_to_use: "Multi-step work".into(),
                phases: vec![
                    WorkflowPhase {
                        title: "Recon".into(),
                        detail: "parallel sweeps".into(),
                        model: "haiku".into(),
                    },
                    WorkflowPhase {
                        title: "Spec".into(),
                        detail: "orchestrator tier".into(),
                        model: String::new(),
                    },
                    WorkflowPhase {
                        title: "Gate".into(),
                        detail: "gate suite".into(),
                        model: "haiku".into(),
                    },
                ],
            }],
        }
    }

    #[test]
    fn invocation_prefixes_the_name() {
        assert_eq!(sample().workflows[0].invocation(), "/agentic-delegation");
    }

    #[test]
    fn phase_count_matches_declared_phases() {
        assert_eq!(sample().workflows[0].phase_count(), 3);
    }

    #[test]
    fn empty_model_renders_as_inherit_not_blank() {
        let wf = sample();
        assert_eq!(wf.workflows[0].phases[0].model_label(), "haiku");
        assert_eq!(wf.workflows[0].phases[1].model_label(), "inherit");
    }

    #[test]
    fn model_tiers_dedupe_in_phase_order() {
        assert_eq!(
            sample().workflows[0].model_tiers(),
            vec!["haiku", "inherit"]
        );
    }

    #[test]
    fn round_trips_through_json() {
        let original = sample();
        let json = serde_json::to_string(&original).unwrap();
        let parsed: WorkflowCatalog = serde_json::from_str(&json).unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn optional_fields_default_when_absent() {
        let json = r#"{
            "version": "0.1.0",
            "description": "d",
            "workflows": [{
                "id": "w", "name": "w", "path": "p", "description": "x"
            }]
        }"#;
        let parsed: WorkflowCatalog = serde_json::from_str(json).unwrap();
        assert_eq!(parsed.workflows[0].when_to_use, "");
        assert!(parsed.workflows[0].phases.is_empty());
        assert_eq!(parsed.workflows[0].phase_count(), 0);
    }

    #[test]
    fn unknown_field_is_rejected() {
        // deny_unknown_fields is the contract that makes a generator change that adds a
        // key fail loudly here rather than being silently dropped from the display.
        let json = r#"{
            "version": "0.1.0",
            "description": "d",
            "workflows": [{
                "id": "w", "name": "w", "path": "p", "description": "x",
                "surpriseKey": true
            }]
        }"#;
        assert!(serde_json::from_str::<WorkflowCatalog>(json).is_err());
    }
}
