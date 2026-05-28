use serde::{Deserialize, Serialize};

/// AI harness platforms that can execute agents and skills.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Harness {
    Codex,
    Copilot,
    ClaudeCode,
    Cursor,
    Gemini,
    Kiro,
    Other,
}

/// How the catalog entry was created.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SourceType {
    Original,
    Adapted,
    ReferenceOnly,
}
