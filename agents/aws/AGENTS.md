# AGENTS.md

## Purpose
- Store AWS marketplace agents with canonical identity and harness-specific variants.

## Patterns
- `agents/aws/<skill-id>-agent/AGENT.md` is the harness-neutral contract.
- `agents/aws/<skill-id>-agent/harnesses/codex.toml` is the Codex native variant.
- `agents/aws/<skill-id>-agent/harnesses/copilot.agent.md` is the GitHub Copilot / VS Code variant.
- `agents/aws/<skill-id>-agent/harnesses/claude-code.agent.md` is the Claude Code Markdown-family variant.
- `agents/aws/<skill-id>-agent/harnesses/cursor.agent.md` is the Cursor Markdown-family variant.
- `agents/aws/<skill-id>-agent/harnesses/gemini.agent.md` is the Gemini CLI Markdown-family variant.
- `agents/aws/<skill-id>-agent/harnesses/kiro-ide.agent.md` and `harnesses/kiro-cli.agent.json` are the split Kiro variants; do not pretend IDE Markdown and CLI JSON are interchangeable.
- `agents/aws/<skill-id>-agent/metadata.json` mirrors agent metadata beside the asset and aligns with `catalog/agents.json`.

## Rules
- Keep skill links pointed at `skills/aws/<skill-id>/SKILL.md`.
- Keep agent catalog IDs suffixed with `-agent` to avoid colliding with skill IDs.
- Keep prompts role-first and token-lean; load skill references only on demand.
- Keep `harnesses/codex.toml` flat and template-aligned: no leading indentation on top-level keys and use TOML multiline strings for `developer_instructions`.
- Prefer configured AWS MCP capability evidence when the active client exposes it, especially `AwsDocumentationMcpServer` for docs grounding.
- If `uvx` cannot run for AWS docs MCP setup, say: "I can't run uvx here, so I'm falling back to official AWS docs." Then fall back to trusted AWS documentation, Context7, and sanitized user evidence.
- Treat runtime-exposed AWS MCP tool inventory as truth. Do not invent a server, namespace, or tool just because documentation or local config mentions it.
- Run `npm run validate` after changes.
