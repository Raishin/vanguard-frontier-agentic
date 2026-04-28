# AGENTS.md

## Purpose
- Store OCI marketplace agents with canonical identity and harness-specific variants.

## Patterns
- `agents/oci/<skill-id>-agent/AGENT.md` is the harness-neutral contract.
- `agents/oci/<skill-id>-agent/harnesses/codex.toml` is the Codex native variant.
- `agents/oci/<skill-id>-agent/harnesses/copilot.agent.md` is the GitHub Copilot / VS Code variant.
- `agents/oci/<skill-id>-agent/harnesses/claude-code.agent.md` is the Claude Code Markdown-family variant.
- `agents/oci/<skill-id>-agent/harnesses/cursor.agent.md` is the Cursor Markdown-family variant.
- `agents/oci/<skill-id>-agent/harnesses/gemini.agent.md` is the Gemini CLI Markdown-family variant.
- `agents/oci/<skill-id>-agent/harnesses/kiro-ide.agent.md` and `harnesses/kiro-cli.agent.json` are the split Kiro variants; do not pretend IDE Markdown and CLI JSON are interchangeable.
- `agents/oci/<skill-id>-agent/metadata.json` mirrors `catalog/agents.json`.

## Rules
- Keep skill links pointed at `skills/oci/<skill-id>/SKILL.md`.
- Keep agent catalog IDs suffixed with `-agent` to avoid colliding with skill IDs.
- Do not create separate `agents/oci/codex/` or `agents/oci/copilot/` silos.
- Run `npm run validate` after changes.
