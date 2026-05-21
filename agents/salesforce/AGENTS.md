# AGENTS.md

## Purpose
- Store Salesforce marketplace agents with canonical identity and harness-specific variants.
- Enforce the T0/T1/T2/T3 execution tier model documented in `docs/execution-tiers.md`.

## Patterns
- `agents/salesforce/<skill-id>-agent/AGENT.md` is the harness-neutral contract.
- `agents/salesforce/<skill-id>-agent/LEAST-PRIVILEGES.md` is the agent's least-privilege Salesforce posture — execution tier, OAuth scopes, Run As account requirements, MCP server binding, blast-radius bound, refusal triggers, escalation path. Required for every agent in this folder.
- `agents/salesforce/<skill-id>-agent/harnesses/codex.toml` is the Codex native variant.
- `agents/salesforce/<skill-id>-agent/harnesses/copilot.agent.md` is the GitHub Copilot / VS Code variant.
- `agents/salesforce/<skill-id>-agent/harnesses/claude-code.agent.md` is the Claude Code Markdown-family variant.
- `agents/salesforce/<skill-id>-agent/harnesses/cursor.agent.md` is the Cursor Markdown-family variant.
- `agents/salesforce/<skill-id>-agent/harnesses/gemini.agent.md` is the Gemini CLI Markdown-family variant.
- `agents/salesforce/<skill-id>-agent/harnesses/kiro-ide.agent.md` and `harnesses/kiro-cli.agent.json` are the split Kiro variants; do not pretend IDE Markdown and CLI JSON are interchangeable.
- `agents/salesforce/<skill-id>-agent/metadata.json` mirrors agent metadata beside the asset and aligns with `catalog/agents.json`.

## Rules
- Keep skill links pointed at `skills/salesforce/<skill-id>/SKILL.md`.
- Keep agent catalog IDs suffixed with `-agent` to avoid colliding with skill IDs.
- Keep prompts role-first and token-lean; load skill references only on demand.
- Every agent must declare an execution tier (T0/T1/T2/T3). T3 production mutation is **PROHIBITED** for all agents — only humans operate via `salesforce-live-guard-agent`.
- T1/T2 agents must declare OAuth scopes as `api refresh_token` only — never `full`, `web`, `chatbot_api`, or `sfap_api`.
- T1/T2 agents must use a Run As account that **denies** `ModifyAllData`, `ViewAllData`, `ViewEncryptedData`, `ModifyMetadata`, `AuthorApex`, and `ManageConnectedApps`, with object/field reads scoped to the agent's declared domain only.
- Keep `harnesses/codex.toml` flat and template-aligned: no leading indentation on top-level keys and use TOML multiline strings for `developer_instructions`.
- Keep `AGENT.md` and Markdown harness adapters flush-left after frontmatter; do not indent the whole body or accidentally turn content into code blocks.
- Keep the maestro (`salesforce-maestro-agent`) as classification + routing only. It must never accept org credentials, session tokens, client secrets, or PII.
- All live-org mutation requests must be refused and escalated to `salesforce-live-guard-agent` with a named human decision owner and a structured case capsule.
- Treat runtime-exposed Salesforce MCP tool inventory as truth. Do not invent a tool just because documentation or local config mentions it.
- Run `npm run validate` after changes. `npm run validate:maestro-routing` covers the 30-domain routing eval for the Salesforce portfolio.
- Non-destructive business-automation roles should stay read-only and should not silently expand into mutation or remediation agents.
