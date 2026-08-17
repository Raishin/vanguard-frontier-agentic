//! Read-only discovery of executable workflows from `.claude/workflows/`.
//!
//! This module parses workflow metadata from JavaScript files in `.claude/workflows/`
//! for display-only purposes. Parsing is a minimal, line-based extractor of the
//! `export const meta = { ... }` block that every workflow script begins with.
//! This is display-only discovery; it does not execute or validate a workflow.
//! The harness remains the only thing that runs one.

use serde::{Deserialize, Serialize};

use crate::security::sanitize::has_control_bytes;

/// A single phase within a workflow's execution plan.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WorkflowPhase {
    pub title: String,
    #[serde(default)]
    pub detail: Option<String>,
}

/// A workflow definition discovered from `.claude/workflows/`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WorkflowDef {
    pub name: String,
    pub description: String,
    #[serde(default)]
    pub when_to_use: Option<String>,
    pub phases: Vec<WorkflowPhase>,
    pub path: String,
}

impl WorkflowDef {
    /// Parse workflow metadata from JavaScript source.
    ///
    /// Extracts `name`, `description`, optional `whenToUse`, and phases array
    /// from the `export const meta = { ... }` block. Returns `None` if the meta
    /// block is absent, required fields are missing, or any extracted value
    /// contains control bytes.
    ///
    /// This is a deliberately minimal, line-based extractor for display-only
    /// discovery. It does not evaluate JavaScript.
    pub fn parse_meta(source: &str, path: &str) -> Option<WorkflowDef> {
        // Find the export const meta line
        let meta_start = source.find("export const meta")?;
        let meta_section = &source[meta_start..];

        // Extract name field: "name": "..."
        let name = extract_field_string(meta_section, "name")?;
        if has_control_bytes(&name) {
            return None;
        }

        // Extract description field: "description": "..."
        let description = extract_field_string(meta_section, "description")?;
        if has_control_bytes(&description) {
            return None;
        }

        // Extract optional whenToUse field: "whenToUse": "..."
        let when_to_use = extract_field_string(meta_section, "whenToUse");
        if when_to_use.as_ref().is_some_and(|s| has_control_bytes(s)) {
            return None;
        }

        // Extract phases array
        let phases = extract_phases(meta_section)?;

        Some(WorkflowDef {
            name,
            description,
            when_to_use,
            phases,
            path: path.to_string(),
        })
    }
}

/// Extract a quoted string value from a field in the meta block.
/// Looks for `fieldname: "value"` or `"fieldname": "value"` and returns the value without quotes.
fn extract_field_string(source: &str, field: &str) -> Option<String> {
    // Try both quoted and unquoted field names
    let pattern_unquoted = format!("{}:", field);
    let pattern_quoted = format!("\"{}\":", field);

    let start = source
        .find(&pattern_quoted)
        .or_else(|| source.find(&pattern_unquoted))?;

    let pattern_len = if source[start..].starts_with(&pattern_quoted) {
        pattern_quoted.len()
    } else {
        pattern_unquoted.len()
    };

    // The meta block is a JavaScript object literal, so a string value may be
    // single- or double-quoted. Take whichever quote character opens first and
    // close on the matching one — closing on `"` regardless would walk straight
    // past a single-quoted value and capture unrelated text further down the
    // file. The shipped workflows use single quotes, so this is the common case
    // rather than an edge case.
    let rest = &source[start + pattern_len..];
    let (quote, value_start) = {
        let dq = rest.find('"');
        let sq = rest.find('\'');
        match (dq, sq) {
            (Some(d), Some(s)) => {
                if d < s {
                    ('"', d)
                } else {
                    ('\'', s)
                }
            }
            (Some(d), None) => ('"', d),
            (None, Some(s)) => ('\'', s),
            (None, None) => return None,
        }
    };

    let value_section = &rest[value_start + 1..];
    let end = value_section.find(quote)?;
    Some(value_section[..end].to_string())
}

/// Extract the phases array from the meta block.
/// Looks for `phases: [ { title: "...", detail: "..." }, ... ]` or with quoted keys.
fn extract_phases(source: &str) -> Option<Vec<WorkflowPhase>> {
    // Try both quoted and unquoted "phases" key
    let phases_start = source
        .find("phases:")
        .or_else(|| source.find("\"phases\":"))?;
    let array_start = source[phases_start..].find('[')?;
    let array_section = &source[phases_start + array_start..];
    let array_end = array_section.find(']')?;
    let array_content = &array_section[1..array_end];

    let mut phases = Vec::new();

    // Split by object boundaries
    let mut in_object = false;
    let mut object_start = 0;
    for (i, c) in array_content.char_indices() {
        match c {
            '{' => {
                if !in_object {
                    in_object = true;
                    object_start = i;
                }
            }
            '}' if in_object => {
                let obj_text = &array_content[object_start..=i];
                if let Some(phase) = parse_phase_object(obj_text) {
                    phases.push(phase);
                }
                in_object = false;
            }
            _ => {}
        }
    }

    if phases.is_empty() {
        return None;
    }

    Some(phases)
}

/// Parse a single phase object from the phases array.
fn parse_phase_object(obj_text: &str) -> Option<WorkflowPhase> {
    let title = extract_field_string(obj_text, "title")?;
    if has_control_bytes(&title) {
        return None;
    }

    let detail = extract_field_string(obj_text, "detail");
    if detail.as_ref().is_some_and(|s| has_control_bytes(s)) {
        return None;
    }

    Some(WorkflowPhase { title, detail })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_meta_well_formed() {
        let source = r#"
export const meta = {
  name: "example-workflow",
  description: "A test workflow",
  whenToUse: "When you need it",
  phases: [
    { title: "Setup", detail: "Initialize environment" },
    { title: "Execute" }
  ]
};
"#;
        let result = WorkflowDef::parse_meta(source, "/test/path.js");
        assert!(result.is_some());
        let def = result.unwrap();
        assert_eq!(def.name, "example-workflow");
        assert_eq!(def.description, "A test workflow");
        assert_eq!(def.when_to_use, Some("When you need it".to_string()));
        assert_eq!(def.phases.len(), 2);
        assert_eq!(def.phases[0].title, "Setup");
        assert_eq!(
            def.phases[0].detail,
            Some("Initialize environment".to_string())
        );
        assert_eq!(def.phases[1].title, "Execute");
        assert_eq!(def.phases[1].detail, None);
    }

    #[test]
    fn parse_meta_no_meta_block() {
        let source = "// Just some JavaScript with no meta";
        let result = WorkflowDef::parse_meta(source, "/test/path.js");
        assert!(result.is_none());
    }

    #[test]
    fn parse_meta_missing_description() {
        let source = r#"
export const meta = {
  name: "example-workflow",
  phases: []
};
"#;
        let result = WorkflowDef::parse_meta(source, "/test/path.js");
        assert!(result.is_none());
    }

    #[test]
    fn parse_meta_control_byte_in_name() {
        let source = "export const meta = {
  name: \"test\x00bad\",
  description: \"desc\",
  phases: []
};";
        let result = WorkflowDef::parse_meta(source, "/test/path.js");
        assert!(result.is_none());
    }

    #[test]
    fn parse_meta_control_byte_in_description() {
        let source = "export const meta = {
  name: \"test\",
  description: \"desc\x1Fbad\",
  phases: []
};";
        let result = WorkflowDef::parse_meta(source, "/test/path.js");
        assert!(result.is_none());
    }

    #[test]
    fn parse_meta_control_byte_in_phase_title() {
        let source = "export const meta = {
  name: \"test\",
  description: \"desc\",
  phases: [
    { title: \"bad\x00phase\" }
  ]
};";
        let result = WorkflowDef::parse_meta(source, "/test/path.js");
        assert!(result.is_none());
    }

    #[test]
    fn parse_meta_optional_when_to_use_missing() {
        let source = r#"
export const meta = {
  name: "workflow",
  description: "A workflow",
  phases: [
    { title: "Step 1" }
  ]
};
"#;
        let result = WorkflowDef::parse_meta(source, "/test/path.js");
        assert!(result.is_some());
        let def = result.unwrap();
        assert_eq!(def.when_to_use, None);
    }

    #[test]
    fn parse_meta_no_phases() {
        let source = r#"
export const meta = {
  name: "test",
  description: "desc",
  phases: []
};
"#;
        let result = WorkflowDef::parse_meta(source, "/test/path.js");
        assert!(result.is_none());
    }

    #[test]
    fn parse_meta_preserves_path() {
        let source = r#"
export const meta = {
  name: "test",
  description: "desc",
  phases: [
    { title: "Step" }
  ]
};
"#;
        let result = WorkflowDef::parse_meta(source, "/some/workflow.js");
        assert!(result.is_some());
        assert_eq!(result.unwrap().path, "/some/workflow.js");
    }

    /// Single-quoted values are the common case: every workflow shipped in
    /// `.claude/workflows/` is written with single quotes. An extractor that
    /// closed on `"` regardless parsed the double-quoted fixtures above and
    /// silently failed on every real file, so both quotings are covered here.
    #[test]
    fn parse_meta_single_quoted_values() {
        let source = r#"
export const meta = {
  name: 'agentic-delegation',
  description: 'Haiku recon, Context7 verification, Sonnet writing, Haiku gates.',
  whenToUse: 'A multi-step change that decomposes into cheap parallel work.',
  phases: [
    { title: 'Recon', detail: 'parallel Haiku Explore sweeps' },
    { title: 'Verify', detail: 'Context7 retrieval then an adversarial refuter' },
    { title: 'Gates' }
  ],
}
"#;
        let def = WorkflowDef::parse_meta(source, "/w.js").expect("single-quoted meta must parse");
        assert_eq!(def.name, "agentic-delegation");
        assert!(def.description.starts_with("Haiku recon"));
        assert!(def.when_to_use.is_some());
        assert_eq!(def.phases.len(), 3);
        assert_eq!(def.phases[0].title, "Recon");
        assert_eq!(def.phases[2].title, "Gates");
        assert_eq!(def.phases[2].detail, None);
    }

    /// Regression guard against fixture drift: parse the workflow actually
    /// committed to this repository. A fixture the test author invented can
    /// diverge from what ships; this one cannot.
    #[test]
    fn parse_meta_parses_the_shipped_workflow() {
        let repo_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(|p| p.parent())
            .expect("workspace root");
        let path = repo_root.join(".claude/workflows/agentic-delegation.js");
        if !path.exists() {
            return; // packaged builds may not ship the .claude tree
        }
        let src = std::fs::read_to_string(&path).expect("read shipped workflow");
        let def = WorkflowDef::parse_meta(&src, &path.display().to_string())
            .expect("the shipped workflow must be discoverable");
        assert_eq!(def.name, "agentic-delegation");
        assert!(!def.description.is_empty());
        assert!(
            !def.phases.is_empty(),
            "shipped workflow declares phases; parser found none"
        );
    }
}
