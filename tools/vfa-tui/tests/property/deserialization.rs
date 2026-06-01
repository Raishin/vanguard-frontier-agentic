// Feature: rust-tui, Property 14: Strict deserialization rejects unknown fields
// **Validates: Requirements 15.2**
//
// For any valid catalog entry JSON object, if one or more unknown top-level fields
// are added, serde deserialization SHALL fail with an error indicating the unknown
// field name. The original valid object (without unknown fields) SHALL deserialize
// successfully.

use proptest::prelude::*;
use proptest::test_runner::Config;
use serde_json::{json, Value};
use vfa_tui::models::{Agent, McpReference, Rule, Skill};

/// Generate a valid Agent JSON object with all required fields.
fn valid_agent_json() -> Value {
    json!({
        "id": "test-agent-001",
        "name": "Test Agent",
        "type": "agent",
        "provider": "aws",
        "harnesses": ["codex", "cursor"],
        "summary": "A test agent for property testing",
        "source_type": "original",
        "official_docs": ["https://example.com/docs"],
        "security_notes": "No special notes",
        "last_verified": "2024-01-01",
        "path": "agents/aws/test-agent",
        "companion_skills": [],
        "execution_tier": "static-review",
        "lifecycle": "stable",
        "harness_variants": {},
        "author": "test-author",
        "version": "1.0.0",
        "provider_coverage": null
    })
}

/// Generate a valid Skill JSON object with all required fields.
fn valid_skill_json() -> Value {
    json!({
        "id": "test-skill-001",
        "name": "Test Skill",
        "type": "skill",
        "provider": "azure",
        "harnesses": ["claude-code"],
        "summary": "A test skill for property testing",
        "source_type": "original",
        "official_docs": ["https://example.com/skill-docs"],
        "security_notes": "No special notes",
        "last_verified": "2024-01-01",
        "path": "skills/azure/test-skill",
        "author": "test-author",
        "version": "1.0.0"
    })
}

/// Generate a valid McpReference JSON object with all required fields.
fn valid_mcp_reference_json() -> Value {
    json!({
        "id": "test-mcp-001",
        "name": "Test MCP Reference",
        "type": "mcp-reference",
        "provider": "gcp",
        "harnesses": ["kiro"],
        "summary": "A test MCP reference for property testing",
        "source_type": "reference-only",
        "official_docs": ["https://example.com/mcp-docs"],
        "security_notes": "Requires credentials",
        "last_verified": "2024-01-01",
        "path": "mcp/gcp/test-mcp",
        "official_project_url": "https://example.com/project",
        "vendor": "Google",
        "auth_model": "oauth2",
        "install_example": "npm install @test/mcp",
        "unofficial_warning": "",
        "trust_matrix": null
    })
}

/// Generate a valid Rule JSON object with all required fields.
fn valid_rule_json() -> Value {
    json!({
        "id": "test-rule-001",
        "name": "Test Rule",
        "type": "rule",
        "provider": "kubernetes",
        "harnesses": ["copilot"],
        "summary": "A test rule for property testing",
        "source_type": "adapted",
        "official_docs": ["https://example.com/rule-docs"],
        "security_notes": "No special notes",
        "last_verified": "2024-01-01",
        "path": "rules/kubernetes/test-rule",
        "author": "test-author"
    })
}

/// Strategy to generate valid unknown field names that won't collide with known fields.
/// Uses a prefix "unknown_" to avoid collisions with real field names.
fn unknown_field_name_strategy() -> impl Strategy<Value = String> {
    "[a-z][a-z0-9_]{2,15}".prop_map(|s| format!("unknown_{}", s))
}

/// Strategy to generate arbitrary JSON values for unknown fields.
fn arbitrary_json_value() -> impl Strategy<Value = Value> {
    prop_oneof![
        Just(Value::Null),
        any::<bool>().prop_map(Value::Bool),
        any::<i64>().prop_map(|n| Value::Number(n.into())),
        "[a-zA-Z0-9 _-]{0,50}".prop_map(Value::String),
        Just(json!([])),
        Just(json!({})),
    ]
}

/// Add unknown fields to a JSON object and return the modified object.
fn add_unknown_fields(base: &Value, field_names: &[String], field_values: &[Value]) -> Value {
    let mut obj = base.as_object().unwrap().clone();
    for (name, value) in field_names.iter().zip(field_values.iter()) {
        obj.insert(name.clone(), value.clone());
    }
    Value::Object(obj)
}

proptest! {
    #![proptest_config(Config::with_cases(256))]

    // --- Agent tests ---

    #[test]
    fn valid_agent_deserializes_successfully(_dummy in Just(())) {
        let json_str = serde_json::to_string(&valid_agent_json()).unwrap();
        let result = serde_json::from_str::<Agent>(&json_str);
        prop_assert!(
            result.is_ok(),
            "Valid agent JSON should deserialize successfully, got error: {:?}",
            result.err()
        );
    }

    #[test]
    fn agent_rejects_unknown_fields(
        field_names in proptest::collection::vec(unknown_field_name_strategy(), 1..5),
        field_values in proptest::collection::vec(arbitrary_json_value(), 1..5),
    ) {
        let base = valid_agent_json();
        let len = field_names.len().min(field_values.len());
        let modified = add_unknown_fields(&base, &field_names[..len], &field_values[..len]);
        let json_str = serde_json::to_string(&modified).unwrap();

        let result = serde_json::from_str::<Agent>(&json_str);
        prop_assert!(
            result.is_err(),
            "Agent deserialization should fail with unknown fields {:?}, but succeeded",
            &field_names[..len]
        );

        // Verify the error message mentions an unknown field
        let err_msg = result.unwrap_err().to_string();
        prop_assert!(
            err_msg.contains("unknown field"),
            "Error should mention 'unknown field', got: {}",
            err_msg
        );
    }

    // --- Skill tests ---

    #[test]
    fn valid_skill_deserializes_successfully(_dummy in Just(())) {
        let json_str = serde_json::to_string(&valid_skill_json()).unwrap();
        let result = serde_json::from_str::<Skill>(&json_str);
        prop_assert!(
            result.is_ok(),
            "Valid skill JSON should deserialize successfully, got error: {:?}",
            result.err()
        );
    }

    #[test]
    fn skill_rejects_unknown_fields(
        field_names in proptest::collection::vec(unknown_field_name_strategy(), 1..5),
        field_values in proptest::collection::vec(arbitrary_json_value(), 1..5),
    ) {
        let base = valid_skill_json();
        let len = field_names.len().min(field_values.len());
        let modified = add_unknown_fields(&base, &field_names[..len], &field_values[..len]);
        let json_str = serde_json::to_string(&modified).unwrap();

        let result = serde_json::from_str::<Skill>(&json_str);
        prop_assert!(
            result.is_err(),
            "Skill deserialization should fail with unknown fields {:?}, but succeeded",
            &field_names[..len]
        );

        let err_msg = result.unwrap_err().to_string();
        prop_assert!(
            err_msg.contains("unknown field"),
            "Error should mention 'unknown field', got: {}",
            err_msg
        );
    }

    // --- McpReference tests ---

    #[test]
    fn valid_mcp_reference_deserializes_successfully(_dummy in Just(())) {
        let json_str = serde_json::to_string(&valid_mcp_reference_json()).unwrap();
        let result = serde_json::from_str::<McpReference>(&json_str);
        prop_assert!(
            result.is_ok(),
            "Valid MCP reference JSON should deserialize successfully, got error: {:?}",
            result.err()
        );
    }

    #[test]
    fn mcp_reference_rejects_unknown_fields(
        field_names in proptest::collection::vec(unknown_field_name_strategy(), 1..5),
        field_values in proptest::collection::vec(arbitrary_json_value(), 1..5),
    ) {
        let base = valid_mcp_reference_json();
        let len = field_names.len().min(field_values.len());
        let modified = add_unknown_fields(&base, &field_names[..len], &field_values[..len]);
        let json_str = serde_json::to_string(&modified).unwrap();

        let result = serde_json::from_str::<McpReference>(&json_str);
        prop_assert!(
            result.is_err(),
            "McpReference deserialization should fail with unknown fields {:?}, but succeeded",
            &field_names[..len]
        );

        let err_msg = result.unwrap_err().to_string();
        prop_assert!(
            err_msg.contains("unknown field"),
            "Error should mention 'unknown field', got: {}",
            err_msg
        );
    }

    // --- Rule tests ---

    #[test]
    fn valid_rule_deserializes_successfully(_dummy in Just(())) {
        let json_str = serde_json::to_string(&valid_rule_json()).unwrap();
        let result = serde_json::from_str::<Rule>(&json_str);
        prop_assert!(
            result.is_ok(),
            "Valid rule JSON should deserialize successfully, got error: {:?}",
            result.err()
        );
    }

    #[test]
    fn rule_rejects_unknown_fields(
        field_names in proptest::collection::vec(unknown_field_name_strategy(), 1..5),
        field_values in proptest::collection::vec(arbitrary_json_value(), 1..5),
    ) {
        let base = valid_rule_json();
        let len = field_names.len().min(field_values.len());
        let modified = add_unknown_fields(&base, &field_names[..len], &field_values[..len]);
        let json_str = serde_json::to_string(&modified).unwrap();

        let result = serde_json::from_str::<Rule>(&json_str);
        prop_assert!(
            result.is_err(),
            "Rule deserialization should fail with unknown fields {:?}, but succeeded",
            &field_names[..len]
        );

        let err_msg = result.unwrap_err().to_string();
        prop_assert!(
            err_msg.contains("unknown field"),
            "Error should mention 'unknown field', got: {}",
            err_msg
        );
    }
}
