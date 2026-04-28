# AGENTS.md

## Purpose
- Store Azure marketplace agents with canonical identity and harness-specific variants.

## Patterns
- `agents/azure/<skill-id>-agent/AGENT.md` is the harness-neutral contract.
- `agents/azure/<skill-id>-agent/harnesses/codex.toml` is the Codex native variant.
- `agents/azure/<skill-id>-agent/harnesses/copilot.agent.md` is the GitHub Copilot / VS Code variant.
- `agents/azure/<skill-id>-agent/harnesses/claude-code.agent.md` is the Claude Code Markdown-family variant.
- `agents/azure/<skill-id>-agent/harnesses/cursor.agent.md` is the Cursor Markdown-family variant.
- `agents/azure/<skill-id>-agent/harnesses/gemini.agent.md` is the Gemini CLI Markdown-family variant.
- `agents/azure/<skill-id>-agent/harnesses/kiro-ide.agent.md` and `harnesses/kiro-cli.agent.json` are the split Kiro variants; do not pretend IDE Markdown and CLI JSON are interchangeable.
- `agents/azure/<skill-id>-agent/metadata.json` mirrors agent metadata beside the asset and aligns with `catalog/agents.json`.

## Rules
- Keep skill links pointed at `skills/azure/<skill-id>/SKILL.md`.
- Keep agent catalog IDs suffixed with `-agent` to avoid colliding with skill IDs.
- Keep prompts role-first and token-lean; load skill references only on demand.
- Treat Azure MCP runtime exposure as truth; do not hard-code undocumented namespace assumptions into the agent contract.
- When discussing Azure MCP setup, prefer Microsoft-documented consolidated mode guidance for AI agents, but adapt to the tools actually exposed in the active client.
- Run `npm run validate` after changes.
